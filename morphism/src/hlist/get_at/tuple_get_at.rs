use crate::{OpOnce, OpTy, OpTyOut};
use crate::{U0, U1, U10, U11, U2, U3, U4, U5, U6, U7, U8, U9};
use core::marker::PhantomData;

/// First-class "get-at" op over tuples.
///
/// Extracts the element at a specific index from a tuple.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TupleGetAt<Ix>(PhantomData<fn() -> Ix>);

impl<Ix> TupleGetAt<Ix> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

/// Type-level view (kept for existing bounds/readability).
pub type TupleGetAtTy<Ix> = TupleGetAt<Ix>;

/// Value-level view (symmetry with `TupleGetAtTy`).
pub type TupleGetAtVal<Ix> = TupleGetAt<Ix>;

/// Apply the get-at op at the type level (output type).
pub type ApplyTupleGetAtTy<Ix, T> = <TupleGetAtTy<Ix> as OpTy<T>>::OutTy;

/// Apply the get-at op at the value level (output value type).
pub type ApplyTupleGetAtVal<Ix, T> = <TupleGetAtVal<Ix> as OpOnce<T>>::OutVal;

/// First-class type-level "get-at" op over tuples (via index isocrate).
///
/// This allows any index type `I` to be used with tuple indexing as long as
/// there exists an `IndexOf: OpTy<I, OutTy = Nat>` mapping.
///
/// Example:
/// ```rust
/// use morphism::{OpTy, TupleGetAtViaTy, U0};
///
/// struct FieldX;
/// struct FieldIndexOf;
/// impl OpTy<FieldX> for FieldIndexOf { type OutTy = U0; }
///
/// type GetX = TupleGetAtViaTy<FieldX, FieldIndexOf>;
/// ```
pub type TupleGetAtViaTy<I, IndexOf> = TupleGetAtTy<OpTyOut<IndexOf, I>>;

macro_rules! impl_get_at_one {
    ($Ix:ty, $OutTy:ident, $out:ident; $( $A:ident, $a:ident ),+ $(,)?) => {
        impl<$( $A ),+> OpOnce<( $( $A, )+ )> for TupleGetAt<$Ix> {
            type OutVal = $OutTy;

            #[inline]
            #[allow(unused_variables)]
            fn run(self, args: ( $( $A, )+ )) -> Self::OutVal {
                let ( $( $a, )+ ) = args;
                $out
            }
        }
    };
}

impl_get_at_one!(U0, A0, a0; A0, a0);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2, A3, a3);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2, A3, a3);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2, A3, a3);
impl_get_at_one!(U3, A3, a3; A0, a0, A1, a1, A2, a2, A3, a3);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4);
impl_get_at_one!(U3, A3, a3; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4);
impl_get_at_one!(U4, A4, a4; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5);
impl_get_at_one!(U3, A3, a3; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5);
impl_get_at_one!(U4, A4, a4; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5);
impl_get_at_one!(U5, A5, a5; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);
impl_get_at_one!(U3, A3, a3; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);
impl_get_at_one!(U4, A4, a4; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);
impl_get_at_one!(U5, A5, a5; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);
impl_get_at_one!(U6, A6, a6; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
impl_get_at_one!(U3, A3, a3; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
impl_get_at_one!(U4, A4, a4; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
impl_get_at_one!(U5, A5, a5; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
impl_get_at_one!(U6, A6, a6; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
impl_get_at_one!(U7, A7, a7; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
impl_get_at_one!(U3, A3, a3; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
impl_get_at_one!(U4, A4, a4; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
impl_get_at_one!(U5, A5, a5; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
impl_get_at_one!(U6, A6, a6; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
impl_get_at_one!(U7, A7, a7; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
impl_get_at_one!(U8, A8, a8; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);
impl_get_at_one!(U3, A3, a3; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);
impl_get_at_one!(U4, A4, a4; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);
impl_get_at_one!(U5, A5, a5; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);
impl_get_at_one!(U6, A6, a6; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);
impl_get_at_one!(U7, A7, a7; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);
impl_get_at_one!(U8, A8, a8; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);
impl_get_at_one!(U9, A9, a9; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U3, A3, a3; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U4, A4, a4; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U5, A5, a5; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U6, A6, a6; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U7, A7, a7; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U8, A8, a8; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U9, A9, a9; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);
impl_get_at_one!(U10, A10, a10; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10);

impl_get_at_one!(U0, A0, a0; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U1, A1, a1; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U2, A2, a2; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U3, A3, a3; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U4, A4, a4; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U5, A5, a5; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U6, A6, a6; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U7, A7, a7; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U8, A8, a8; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U9, A9, a9; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U10, A10, a10; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
impl_get_at_one!(U11, A11, a11; A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10, A11, a11);
