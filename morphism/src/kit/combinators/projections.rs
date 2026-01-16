use crate::kit::op::{OpOnce, OpTy};
use crate::kit::tokens::NullaryToken;
use crate::registry::combinators::{IX_FST, IX_SND};
use crate::tag::Tagged;

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAM - Reflected syntax nodes (AST)
// ═══════════════════════════════════════════════════════════════════════════

/// First projection program node: reflected syntax node.
pub type Fst = Tagged<IX_FST, NullaryToken>;

/// Second projection program node: reflected syntax node.
pub type Snd = Tagged<IX_SND, NullaryToken>;

/// Construct a first projection program node.
#[inline]
pub const fn fst() -> Fst {
    Tagged::new(NullaryToken)
}

/// Construct a second projection program node.
#[inline]
pub const fn snd() -> Snd {
    Tagged::new(NullaryToken)
}

// ═══════════════════════════════════════════════════════════════════════════
// SEMANTICS - Runnable op implementations
// ═══════════════════════════════════════════════════════════════════════════

/// First projection: (A, B) → A
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FstOp;

impl<A, B> OpTy<(A, B)> for FstOp {
    type OutTy = A;
}

impl<A, B> OpOnce<(A, B)> for FstOp {
    type OutVal = A;

    #[inline]
    fn run(self, args: (A, B)) -> Self::OutVal {
        args.0
    }
}

/// Second projection: (A, B) → B
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SndOp;

impl<A, B> OpTy<(A, B)> for SndOp {
    type OutTy = B;
}

impl<A, B> OpOnce<(A, B)> for SndOp {
    type OutVal = B;

    #[inline]
    fn run(self, args: (A, B)) -> Self::OutVal {
        args.1
    }
}
