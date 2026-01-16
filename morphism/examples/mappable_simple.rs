//! Mapping via `Container` (products / HLists) - LAZY API
//!
//! This example shows mapping over **product shapes** (HLists), implemented here
//! as Rust tuples via the `container/std_tuple_bridge` view.
//!
//! Key idea: you can avoid "binding the Op up front" in a bespoke trait by
//! making the trait method generic in `Op` and using a GAT to express the plan:
//! `Self::Plan<Op>`.
//!
//! Notes:
//! - The same `map` stage works for multiple *shapes*: tuples are treated as
//!   HLists, **leaf wrappers** are treated as opaque nodes.
//! - For arity > 11, use the "tail slot" encoding: `(11 head, Tail)` where `Tail`
//!   is itself a tuple (possibly nested again).

use morphism::{Container, FromContainer, IntoContainer, Mappable, MappablePlan};
use morphism::{HlistNode, Id, OpOnce, Tagged, IX_NEWTYPENODE};

// ===========================================================================
// Step 1: Define the element operation
// ===========================================================================

#[derive(Clone, Copy, Default)]
pub struct AddTenOp;

impl OpOnce<i32> for AddTenOp {
    type OutVal = i32;
    fn run(self, x: i32) -> Self::OutVal {
        x + 10
    }
}

impl OpOnce<f64> for AddTenOp {
    type OutVal = f64;
    fn run(self, x: f64) -> Self::OutVal {
        x + 10.0
    }
}

// When the *entire* tuple is wrapped as a NewTypeNode, the map op is applied to the
// whole value (opaque newtypenode), not element-wise.
impl OpOnce<(i32, i32, i32)> for AddTenOp {
    type OutVal = (i32, i32, i32);
    fn run(self, (a, b, c): (i32, i32, i32)) -> Self::OutVal {
        (a + 10, b + 10, c + 10)
    }
}

#[derive(Clone, Copy, Default)]
pub struct MulTwoOp;

impl OpOnce<i32> for MulTwoOp {
    type OutVal = i32;
    fn run(self, x: i32) -> Self::OutVal {
        x * 2
    }
}

/// A wrapper that *forces* its payload to be treated as a NewTypeNode (opaque).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Leaf3<T>(pub T);

impl<T> IntoContainer for Leaf3<T> {
    type Tag = IX_NEWTYPENODE;
    type Payload = T;

    #[inline]
    fn into_container(self) -> Container<Self, Id> {
        Container::<Self, Id>::from_tagged(Tagged::new(self.0))
    }
}

impl<T> FromContainer for Leaf3<T> {
    type Rewrap<NewPayload> = Leaf3<NewPayload>;

    #[inline]
    fn from_container<NewPayload>(payload: NewPayload) -> Self::Rewrap<NewPayload> {
        Leaf3(payload)
    }
}

// ===========================================================================
// Step 2: Another trait layered on top of `MappablePlan`
// ===========================================================================

pub trait AddTen {
    type Plan;
    fn add_10_plan(self) -> Self::Plan;
}

impl<T> AddTen for T
where
    T: MappablePlan,
{
    type Plan = <T as MappablePlan>::Plan<AddTenOp>;

    #[inline]
    fn add_10_plan(self) -> Self::Plan {
        self.map_plan(AddTenOp)
    }
}

// ===========================================================================
// Step 3: Use it!
// ===========================================================================

fn main() {
    println!("--- Generic plan trait (Op passed, not bound in the trait) ---");
    let tuple = (10i32, 20i32, 30i32);

    // Build plans by passing the op value; the trait itself doesn't fix `Op`.
    let add_out = tuple.map_plan(AddTenOp).collect();
    let mul_out = tuple.map_plan(MulTwoOp).collect();

    println!("{:?} + 10 = {:?}", tuple, add_out);
    println!("{:?} * 2  = {:?}", tuple, mul_out);
    assert_eq!(add_out, (20, 30, 40));
    assert_eq!(mul_out, (20, 40, 60));

    println!("\n--- Another trait implemented for anything MappablePlan ---");
    let out = tuple.add_10_plan().collect();
    println!("{:?} + 10 = {:?}", tuple, out);
    assert_eq!(out, (20, 30, 40));

    println!("\n--- Eager API (collects internally) ---");
    let out = tuple.map(AddTenOp);
    println!("{:?} + 10 = {:?}", tuple, out);
    assert_eq!(out, (20, 30, 40));

    println!("\n--- Heterogeneous tuples ---");
    let hetero = (50i32, 1.23f64, 100i32);
    let out = hetero.map_plan(AddTenOp).collect();
    println!("{:?} + 10 = {:?}", hetero, out);
    assert_eq!(out, (60, 11.23, 110));

    println!("\n--- Leaf node wrapper (opaque tuple payload) ---");
    let leaf = Leaf3((1i32, 2i32, 3i32));
    let out = leaf.map_plan(AddTenOp).collect();
    println!("{leaf:?} + 10 = {out:?}");
    assert_eq!(out, Leaf3((11, 12, 13)));

    println!("\n--- Collect into the container representation (Tagged) ---");
    // Same computation, but collect into the explicit container host type.
    let tuple = (1i32, 2i32, 3i32);
    //: Tagged<IX_HLISTNODE, (i32, i32, i32)>
    let tagged: HlistNode<(i32, i32, i32)> = tuple.map_plan(AddTenOp).collect_into();
    assert_eq!(tagged.into_inner(), (11, 12, 13));

    println!("\n✓ All tests passed!");
}
