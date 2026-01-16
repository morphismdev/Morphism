use crate::{IfOut, OpOnce, OpTy, OpTyOut, Select, TupleGetAt};
use crate::{LtOp, NumEqOp, Sub, U11, U12};
use core::marker::PhantomData;

/// Chunked "get-at" operation over the showcase crate's **HList-encoded tuple** convention.
///
/// Encoding invariant (same as `hlist/`):
/// - If arity ≤ 11: it is a plain tuple `(E0, .., Ek)`.
/// - If arity > 11: it is encoded as `(E0, .., E10, Tail)` where `Tail` encodes the rest.
///
/// **Important:** the **tail slot is the 12th field** (index `U11`).
/// This means:
/// - `Ix < U11` selects from the head chunk (indices `U0..U10`).
/// - `Ix == U11` selects the `Tail` field itself.
/// - `Ix > U11` selects from within `Tail` at index `Ix - U12`
///   (so `U12` maps to the first element *inside* the tail).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HlistGetAt<Ix>(PhantomData<fn() -> Ix>);

impl<Ix> HlistGetAt<Ix> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

/// Type-level view (kept for existing bounds/readability).
pub type HlistGetAtTy<Ix> = HlistGetAt<Ix>;

/// Value-level view (symmetry with `HlistGetAtTy`).
pub type HlistGetAtVal<Ix> = HlistGetAt<Ix>;

/// Apply the get-at op at the type level (output type).
pub type ApplyHlistGetAtTy<Ix, T> = <HlistGetAtTy<Ix> as OpTy<T>>::OutTy;

/// Apply the get-at op at the value level (output value type).
pub type ApplyHlistGetAtVal<Ix, T> = <HlistGetAtVal<Ix> as OpOnce<T>>::OutVal;

// ─────────────────────────────────────────────────────────────────────────────
// Small tuples (arity ≤ 11): delegate to TupleGetAt (explicit impls, no blanket)
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! impl_small_tuple {
    ( $( $A:ident ),+ $(,)? ) => {
        impl<Ix, $( $A, )+> OpOnce<( $( $A, )+ )> for HlistGetAt<Ix>
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
// Head case: Ix < U11  => pick from head chunk (U0..U10)
// Tail slot: Ix == U11 => return Tail
// Recurse:   Ix > U11  => recurse into Tail at (Ix - U12)
// ─────────────────────────────────────────────────────────────────────────────

// We can't express disjoint cases as multiple impls (coherence), so we select a
// case-op at the type level and then delegate to it.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[doc(hidden)]
pub struct HeadCase<Ix>(PhantomData<fn() -> Ix>);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[doc(hidden)]
pub struct NotHeadCase<Ix>(PhantomData<fn() -> Ix>);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[doc(hidden)]
pub struct TailSlotCase;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[doc(hidden)]
pub struct RecurseCase<Ix>(PhantomData<fn() -> Ix>);

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

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail>
    OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> for TailSlotCase
{
    type OutVal = Tail;

    #[inline]
    fn run(self, t: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)) -> Self::OutVal {
        let (_a0, _a1, _a2, _a3, _a4, _a5, _a6, _a7, _a8, _a9, _a10, tail) = t;
        tail
    }
}

impl<Ix, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail>
    OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> for RecurseCase<Ix>
where
    Sub<U12>: OpTy<Ix>,
    HlistGetAt<OpTyOut<Sub<U12>, Ix>>: OpOnce<Tail>,
{
    type OutVal = <HlistGetAt<OpTyOut<Sub<U12>, Ix>> as OpOnce<Tail>>::OutVal;

    #[inline]
    fn run(self, t: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)) -> Self::OutVal {
        let (_a0, _a1, _a2, _a3, _a4, _a5, _a6, _a7, _a8, _a9, _a10, tail) = t;
        HlistGetAt::<OpTyOut<Sub<U12>, Ix>>::new().run(tail)
    }
}

impl<Ix, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail>
    OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> for NotHeadCase<Ix>
where
    NumEqOp<U11>: OpTy<Ix>,
    // Explicit expansion of `IfOut<NumEqOp<U11>, TailSlotCase, RecurseCase<Ix>, Ix>`:
    Select<TailSlotCase, RecurseCase<Ix>>: OpTy<OpTyOut<NumEqOp<U11>, Ix>>,
    OpTyOut<Select<TailSlotCase, RecurseCase<Ix>>, OpTyOut<NumEqOp<U11>, Ix>>:
        OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> + Default,
    IfOut<NumEqOp<U11>, TailSlotCase, RecurseCase<Ix>, Ix>:
        OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> + Default,
{
    type OutVal = <IfOut<NumEqOp<U11>, TailSlotCase, RecurseCase<Ix>, Ix> as OpOnce<(
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
        <IfOut<NumEqOp<U11>, TailSlotCase, RecurseCase<Ix>, Ix> as Default>::default().run(t)
    }
}

impl<Ix, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail>
    OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)> for HlistGetAt<Ix>
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
    use crate::{U0, U10, U11, U12, U13};

    #[test]
    fn hlist_get_at_head_indices_work_on_chunked_tuple() {
        let table = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ("tail0", "tail1"));
        assert_eq!(HlistGetAt::<U0>::new().run(table), 0);
        assert_eq!(HlistGetAt::<U10>::new().run(table), 10);
    }

    #[test]
    fn hlist_get_at_u11_returns_tail_slot() {
        let table = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ("tail0", "tail1"));
        assert_eq!(HlistGetAt::<U11>::new().run(table), ("tail0", "tail1"));
    }

    #[test]
    fn hlist_get_at_indices_beyond_u11_recurse_into_tail_at_ix_minus_u12() {
        // U12 maps to tail index U0; U13 maps to tail index U1
        let table = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ("tail0", "tail1"));
        assert_eq!(HlistGetAt::<U12>::new().run(table), "tail0");
        assert_eq!(HlistGetAt::<U13>::new().run(table), "tail1");
    }
}
