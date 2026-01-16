//! Mapping via `Container` (products / HLists) - LAZY API
//!
//! This example shows mapping over **product shapes** (HLists), implemented here
//! as Rust tuples via the `container/std_tuple_bridge` view.
//!
//! Key idea: you can avoid "binding the Op up front" in a bespoke trait by
//! making the trait method generic in `Op` and using a GAT to express the plan:
//! `Self::Plan<Op>`.
//!
//! This example demonstrates how to create a trait that **auto-implements** for
//! tuples and other container shapes, while allowing trait methods to
//! take **extra parameters** beyond just `self`. The trait builds an `Op` value
//! from those parameters and forwards to `MappablePlan::map_plan`.
//!
//! Notes:
//! - The same `map` stage works for multiple *shapes*: tuples are treated as
//!   HLists, **leaf wrappers** are treated as opaque nodes.
//! - For arity > 11, use the "tail slot" encoding: `(11 head, Tail)` where `Tail`
//!   is itself a tuple (possibly nested again).

use morphism::{Container, FromContainer, IntoContainer, Mappable, MappablePlan};
use morphism::{HlistNode, Id, OpOnce, Tagged, IX_NEWTYPENODE};

// ===========================================================================
// Step 1: Define the element operation (parameterized)
// ===========================================================================

/// An operation that adds configurable values to different types.
#[derive(Clone, Copy, Debug)]
pub struct AddParamsOp {
    pub add_i32: i32,
    pub add_f64: f64,
}

impl OpOnce<i32> for AddParamsOp {
    type OutVal = i32;
    fn run(self, x: i32) -> Self::OutVal {
        x + self.add_i32
    }
}

impl OpOnce<f64> for AddParamsOp {
    type OutVal = f64;
    fn run(self, x: f64) -> Self::OutVal {
        x + self.add_f64
    }
}

// When the *entire* tuple is wrapped as a NewTypeNode, the map op is applied to the
// whole value (opaque newtypenode), not element-wise.
impl OpOnce<(i32, i32, i32)> for AddParamsOp {
    type OutVal = (i32, i32, i32);
    fn run(self, (a, b, c): (i32, i32, i32)) -> Self::OutVal {
        (a + self.add_i32, b + self.add_i32, c + self.add_i32)
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
// Step 2: Auto-implemented trait with extra parameters (beyond `self`)
// ===========================================================================

/// A trait that auto-implements for any `MappablePlan` type (tuples, etc.)
/// and allows passing extra parameters to configure the operation.
pub trait AddParams {
    type Plan;
    /// Build a plan that adds `add_i32` to i32 values and `add_f64` to f64 values.
    fn add_params_plan(self, add_i32: i32, add_f64: f64) -> Self::Plan;
}

impl<T> AddParams for T
where
    T: MappablePlan,
{
    type Plan = <T as MappablePlan>::Plan<AddParamsOp>;

    #[inline]
    fn add_params_plan(self, add_i32: i32, add_f64: f64) -> Self::Plan {
        self.map_plan(AddParamsOp { add_i32, add_f64 })
    }
}

// ===========================================================================
// Step 3: Use it!
// ===========================================================================

fn main() {
    println!("--- Auto-implemented trait with extra parameters ---");
    let tuple = (10i32, 20i32, 30i32);

    // The trait method takes extra parameters beyond `self`!
    let out = tuple.add_params_plan(10, 10.0).collect();
    println!("{:?} + (10, 10.0) = {:?}", tuple, out);
    assert_eq!(out, (20, 30, 40));

    // Different parameters work too
    let out = tuple.add_params_plan(5, 5.0).collect();
    println!("{:?} + (5, 5.0) = {:?}", tuple, out);
    assert_eq!(out, (15, 25, 35));

    println!("\n--- Still works for heterogeneous tuples ---");
    let hetero = (50i32, 1.23f64, 100i32);
    let out = hetero.add_params_plan(10, 10.0).collect();
    println!("{:?} + (10, 10.0) = {:?}", hetero, out);
    assert_eq!(out, (60, 11.23, 110));

    println!("\n--- Leaf node wrapper (opaque tuple payload) ---");
    let leaf = Leaf3((1i32, 2i32, 3i32));
    let out = leaf.add_params_plan(10, 10.0).collect();
    println!("{leaf:?} + (10, 10.0) = {out:?}");
    assert_eq!(out, Leaf3((11, 12, 13)));

    println!("\n--- Direct map_plan API (for comparison) ---");
    // You can still use the generic `map_plan` API directly
    let tuple = (10i32, 20i32, 30i32);
    let add_out = tuple
        .map_plan(AddParamsOp {
            add_i32: 10,
            add_f64: 10.0,
        })
        .collect();
    let mul_out = tuple.map_plan(MulTwoOp).collect();
    println!("{:?} + (10, 10.0) = {:?}", tuple, add_out);
    println!("{:?} * 2 = {:?}", tuple, mul_out);
    assert_eq!(add_out, (20, 30, 40));
    assert_eq!(mul_out, (20, 40, 60));

    println!("\n--- Eager API (collects internally) ---");
    let out = tuple.map(AddParamsOp {
        add_i32: 10,
        add_f64: 10.0,
    });
    println!("{:?} + (10, 10.0) = {:?}", tuple, out);
    assert_eq!(out, (20, 30, 40));

    println!("\n--- Collect into the container representation (Tagged) ---");
    // Same computation, but collect into the explicit container host type.
    let tuple = (1i32, 2i32, 3i32);
    //: Tagged<IX_HLISTNODE, (i32, i32, i32)>
    let tagged: HlistNode<(i32, i32, i32)> = tuple.add_params_plan(10, 10.0).collect_into();
    assert_eq!(tagged.into_inner(), (11, 12, 13));

    println!("\n✓ All tests passed!");
}
