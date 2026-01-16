//! Foldable (fold over children) via `Container`.
//!
//! Shows:
//! - `x.fold_children_l(op, acc)`

use morphism::{Container, Foldable, FromContainer, IntoContainer};
use morphism::{Id, OpOnce, OpTy, Tagged, IX_NEWTYPENODE};

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CountL;

impl<S> OpTy<(usize, S)> for CountL {
    type OutTy = usize;
}

impl<S> OpOnce<(usize, S)> for CountL {
    type OutVal = usize;

    #[inline]
    fn run(self, (acc, _elem): (usize, S)) -> Self::OutVal {
        acc + 1
    }
}

fn main() {
    println!("--- Foldable: fold over children ---");

    let tuple = (1u8, 2u16, 3u32);

    let n_l: usize = tuple.fold_children_l(CountL, 0usize);
    assert_eq!(n_l, 3);

    println!("\n--- Leaf node wrapper (opaque payload has no children) ---");
    let leaf = Leaf3((1u8, 2u16, 3u32));

    // Leaf has no children; fold returns the accumulator unchanged.
    let n_l_leaf: usize = leaf.fold_children_l(CountL, 0usize);
    assert_eq!(n_l_leaf, 0);

    println!("✓ Foldable demo passed");
}
