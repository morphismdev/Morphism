use super::tuple_fold_l::{TupleFoldL, TupleFoldLTy};
use crate::{OpOnce, OpTy, OpTyOut};

/// Segment fold-left hetero as a transformer: `Acc -> AccNext`.
///
/// Captures:
/// - `f`: the fold step (hetero) with signature `(Acc, Elem) -> AccNext`
/// - `seg`: a non-empty tuple segment `(E0, E1, ..)`
///
/// Semantics:
/// `run(acc0) = TupleFoldL::new(f, acc0).run(seg)`
///
/// Notes:
/// - Segment is intentionally **non-empty** (because `TupleFoldL` has no runtime `OpOnce<UnitToken>` impl).
/// - This is a building block for balanced composition over chunk transformers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentFoldL<F, Seg> {
    pub f: F,
    pub seg: Seg,
}

impl<F, Seg> SegmentFoldL<F, Seg> {
    #[inline]
    pub const fn new(f: F, seg: Seg) -> Self {
        Self { f, seg }
    }
}

impl<Acc, F, Seg> OpTy<Acc> for SegmentFoldL<F, Seg>
where
    TupleFoldLTy<F, Acc>: OpTy<Seg>,
{
    type OutTy = OpTyOut<TupleFoldLTy<F, Acc>, Seg>;
}

impl<Acc, F, Seg> OpOnce<Acc> for SegmentFoldL<F, Seg>
where
    TupleFoldL<F, Acc>: OpOnce<Seg>,
{
    type OutVal = <TupleFoldL<F, Acc> as OpOnce<Seg>>::OutVal;

    #[inline]
    fn run(self, acc: Acc) -> Self::OutVal {
        TupleFoldL::new(self.f, acc).run(self.seg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generic hetero step: `(Acc, X) -> (Acc, X)` (builds a left-associated pair chain).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct StepPair;

    impl<Acc, X> OpTy<(Acc, X)> for StepPair {
        type OutTy = (Acc, X);
    }

    impl<Acc, X> OpOnce<(Acc, X)> for StepPair {
        type OutVal = (Acc, X);

        #[inline]
        fn run(self, (acc, x): (Acc, X)) -> Self::OutVal {
            (acc, x)
        }
    }

    #[test]
    fn segment_foldl_runs() {
        let t = SegmentFoldL::new(StepPair, (1u8, 2u16, 3u32));
        let out = t.run(());
        assert_eq!(out, ((((), 1u8), 2u16), 3u32));
    }

    #[test]
    fn segment_foldl_type_checks() {
        type Out = <SegmentFoldL<StepPair, (u8, u16, u32)> as OpTy<()>>::OutTy;
        let _x: Out = ((((), 0u8), 0u16), 0u32);
    }
}
