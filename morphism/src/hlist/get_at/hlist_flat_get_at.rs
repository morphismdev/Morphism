use crate::{IfOut, OpOnce, OpTy, OpTyOut, Select, Sub, TupleGetAt};
use crate::{LtOp, U11};
use core::marker::PhantomData;

/// Flat "get-at" operation over the showcase crate's **HList-encoded tuple** convention.
///
/// This is an **adapter** over the chunked encoding used by `HlistGetAt`.
///
/// ## Semantics (flat / logical indexing)
///
/// The chunked encoding for arity > 11 is:
///
/// - `(E0, .., E10, Tail)`
///
/// where `Tail` recursively encodes the remaining elements.
///
/// `HlistFlatGetAt<Ix>` treats the table as if it were a **single flat list** of elements:
///
/// - `Ix < U11`: select from the head chunk (`U0..U10`)
/// - `Ix >= U11`: recurse into `Tail` at index `(Ix - U11)`
///
/// Notably, unlike `HlistGetAt`, **`U11` is not a "tail slot"** here. Instead,
/// `U11` selects the **first element inside the tail** (i.e. the 12th element overall).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HlistFlatGetAt<Ix>(PhantomData<fn() -> Ix>);

impl<Ix> HlistFlatGetAt<Ix> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

/// Type-level view (kept for existing bounds/readability).
pub type HlistFlatGetAtTy<Ix> = HlistFlatGetAt<Ix>;

/// Value-level view (symmetry with `HlistFlatGetAtTy`).
pub type HlistFlatGetAtVal<Ix> = HlistFlatGetAt<Ix>;

/// Apply the get-at op at the type level (output type).
pub type ApplyHlistFlatGetAtTy<Ix, T> = <HlistFlatGetAtTy<Ix> as OpTy<T>>::OutTy;

/// Apply the get-at op at the value level (output value type).
pub type ApplyHlistFlatGetAtVal<Ix, T> = <HlistFlatGetAtVal<Ix> as OpOnce<T>>::OutVal;

// ─────────────────────────────────────────────────────────────────────────────
// Small tuples (arity ≤ 11): delegate to TupleGetAt (explicit impls, no blanket)
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! impl_small_tuple {
    ( $( $A:ident ),+ $(,)? ) => {
        impl<Ix, $( $A, )+> OpOnce<( $( $A, )+ )> for HlistFlatGetAt<Ix>
        where
            TupleGetAt<Ix>: OpOnce<( $( $A, )+ )>,
        {
            type OutVal = <TupleGetAt<Ix> as OpOnce<( $( $A, )+ )>>::OutVal;

            #[inline]
            fn run(self, t: ( $( $A, )+ )) -> Self::OutVal {
                TupleGetAt::<Ix>::new().run(t)
            }
        }
    };
}

impl_small_tuple!(A0);
impl_small_tuple!(A0, A1);
impl_small_tuple!(A0, A1, A2);
impl_small_tuple!(A0, A1, A2, A3);
impl_small_tuple!(A0, A1, A2, A3, A4);
impl_small_tuple!(A0, A1, A2, A3, A4, A5);
impl_small_tuple!(A0, A1, A2, A3, A4, A5, A6);
impl_small_tuple!(A0, A1, A2, A3, A4, A5, A6, A7);
impl_small_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8);
impl_small_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_small_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

// ─────────────────────────────────────────────────────────────────────────────
// Chunked tuples: (A0..A10, Tail)
// Flat semantics:
// - Ix < U11  => pick from head chunk (U0..U10)
// - Ix >= U11 => recurse into Tail at (Ix - U11)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[doc(hidden)]
pub struct HeadCase<Ix>(PhantomData<fn() -> Ix>);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[doc(hidden)]
pub struct NotHeadCase<Ix>(PhantomData<fn() -> Ix>);

impl<Ix, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail>
    OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> for HeadCase<Ix>
where
    TupleGetAt<Ix>: OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10)>,
{
    type OutVal = <TupleGetAt<Ix> as OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10)>>::OutVal;

    #[inline]
    fn run(self, t: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)) -> Self::OutVal {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, _tail) = t;
        TupleGetAt::<Ix>::new().run((a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10))
    }
}

impl<Ix, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail>
    OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> for NotHeadCase<Ix>
where
    Sub<U11>: OpTy<Ix>,
    HlistFlatGetAt<OpTyOut<Sub<U11>, Ix>>: OpOnce<Tail>,
{
    type OutVal = <HlistFlatGetAt<OpTyOut<Sub<U11>, Ix>> as OpOnce<Tail>>::OutVal;

    #[inline]
    fn run(self, t: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)) -> Self::OutVal {
        let (_a0, _a1, _a2, _a3, _a4, _a5, _a6, _a7, _a8, _a9, _a10, tail) = t;
        HlistFlatGetAt::<OpTyOut<Sub<U11>, Ix>>::new().run(tail)
    }
}

impl<Ix, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail>
    OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> for HlistFlatGetAt<Ix>
where
    LtOp<U11>: OpTy<Ix>,
    // Explicit expansion of `IfOut<LtOp<U11>, HeadCase<Ix>, NotHeadCase<Ix>, Ix>`:
    Select<HeadCase<Ix>, NotHeadCase<Ix>>: OpTy<OpTyOut<LtOp<U11>, Ix>>,
    OpTyOut<Select<HeadCase<Ix>, NotHeadCase<Ix>>, OpTyOut<LtOp<U11>, Ix>>:
        OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> + Default,
    IfOut<LtOp<U11>, HeadCase<Ix>, NotHeadCase<Ix>, Ix>:
        OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> + Default,
{
    type OutVal = <IfOut<LtOp<U11>, HeadCase<Ix>, NotHeadCase<Ix>, Ix> as OpOnce<(
        A0,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        Tail,
    )>>::OutVal;

    #[inline]
    fn run(self, t: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)) -> Self::OutVal {
        <IfOut<LtOp<U11>, HeadCase<Ix>, NotHeadCase<Ix>, Ix> as Default>::default().run(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{U0, U10, U11, U12, U21, U22, U23};

    #[test]
    fn flat_get_at_boundary_u11_is_first_element_of_tail() {
        let table = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ("tail0", "tail1"));

        assert_eq!(HlistFlatGetAt::<U0>::new().run(table), 0);
        assert_eq!(HlistFlatGetAt::<U10>::new().run(table), 10);
        assert_eq!(HlistFlatGetAt::<U11>::new().run(table), "tail0");
        assert_eq!(HlistFlatGetAt::<U12>::new().run(table), "tail1");
    }

    #[test]
    fn flat_get_at_recurses_across_nested_chunk_boundaries() {
        // Outer head: 0..10
        // Outer tail: a chunked tuple whose head is 11..21 and whose tail is ("tail2_0", "tail2_1")
        let table = (
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            (
                11,
                12,
                13,
                14,
                15,
                16,
                17,
                18,
                19,
                20,
                21,
                ("tail2_0", "tail2_1"),
            ),
        );

        assert_eq!(HlistFlatGetAt::<U11>::new().run(table), 11);
        assert_eq!(HlistFlatGetAt::<U21>::new().run(table), 21);
        assert_eq!(HlistFlatGetAt::<U22>::new().run(table), "tail2_0");
        assert_eq!(HlistFlatGetAt::<U23>::new().run(table), "tail2_1");
    }
}
