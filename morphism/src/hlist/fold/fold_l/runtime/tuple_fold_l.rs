use crate::PartialLOp;
use crate::{OpOnce, OpTy, OpTyOut, UnitToken};

// ═══════════════════════════════════════════════════════════════════════════
// Runnable op implementation
// ═══════════════════════════════════════════════════════════════════════════

/// First-class left-fold hetero op (type-changing accumulator).
///
/// Interpreted as:
/// `TupleFoldL<F, Acc> : (A0, A1, ..) -> F(F(Acc, A0), A1) ..`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TupleFoldL<F, Acc> {
    pub f: F,
    pub acc: Acc,
}

impl<F, Acc> TupleFoldL<F, Acc> {
    #[inline]
    pub const fn new(f: F, acc: Acc) -> Self {
        Self { f, acc }
    }
}

/// Type-level view (kept for existing bounds/readability).
pub type TupleFoldLTy<F, Acc> = TupleFoldL<F, Acc>;

impl<F, Acc> OpTy<UnitToken> for TupleFoldL<F, Acc> {
    type OutTy = Acc;
}

// (A0,) -> F(Acc, A0)
impl<F, Acc, A0> OpTy<(A0,)> for TupleFoldL<F, Acc>
where
    F: OpTy<(Acc, A0)>,
{
    type OutTy = OpTyOut<F, (Acc, A0)>;
}

// (A0,) - single element
impl<F, A, Acc> OpOnce<(A,)> for TupleFoldL<F, Acc>
where
    F: OpOnce<(Acc, A)>,
{
    type OutVal = <F as OpOnce<(Acc, A)>>::OutVal;

    #[inline]
    fn run(self, args: (A,)) -> Self::OutVal {
        PartialLOp {
            env: self.acc,
            op: self.f,
        }
        .run(args.0)
    }
}

// n>=2: fold head into acc, recurse on tail
macro_rules! impl_tuple_foldl_hetero_tuple {
    ($A0:ident, $a0:ident, $($A:ident, $a:ident),+ $(,)?) => {
        impl<F, Acc, $A0, $($A,)+> OpTy<($A0, $($A,)+)> for TupleFoldL<F, Acc>
        where
            F: OpTy<(Acc, $A0)>,
            TupleFoldL<F, OpTyOut<F, (Acc, $A0)>>: OpTy<($($A,)+)>,
        {
            type OutTy = OpTyOut<
                TupleFoldL<F, OpTyOut<F, (Acc, $A0)>>,
                ($($A,)+)
            >;
        }

        impl<F, $A0, $($A,)+ Acc> OpOnce<($A0, $($A,)+)> for TupleFoldL<F, Acc>
        where
            F: OpOnce<(Acc, $A0)> + Clone,
            TupleFoldL<F, <F as OpOnce<(Acc, $A0)>>::OutVal>: OpOnce<($($A,)+)>,
        {
            type OutVal = <TupleFoldL<F, <F as OpOnce<(Acc, $A0)>>::OutVal> as OpOnce<($($A,)+)>>::OutVal;

            #[inline]
            fn run(self, args: ($A0, $($A,)+)) -> Self::OutVal {
                let ($a0, $($a,)+) = args;
                let acc_next = PartialLOp { env: self.acc, op: self.f.clone() }.run($a0);
                TupleFoldL::new(self.f, acc_next).run(($($a,)+))
            }
        }
    };
}

impl_tuple_foldl_hetero_tuple!(A0, a0, A1, a1);
impl_tuple_foldl_hetero_tuple!(A0, a0, A1, a1, A2, a2);
impl_tuple_foldl_hetero_tuple!(A0, a0, A1, a1, A2, a2, A3, a3);
impl_tuple_foldl_hetero_tuple!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4);
impl_tuple_foldl_hetero_tuple!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5);
impl_tuple_foldl_hetero_tuple!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);
impl_tuple_foldl_hetero_tuple!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
impl_tuple_foldl_hetero_tuple!(
    A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8
);
impl_tuple_foldl_hetero_tuple!(
    A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9
);
impl_tuple_foldl_hetero_tuple!(
    A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8, A9, a9, A10, a10
);
