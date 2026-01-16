// ═══════════════════════════════════════════════════════════════════════════
// CONSTANT
// ═══════════════════════════════════════════════════════════════════════════

use crate::kit::op::{OpOnce, OpTy};
use crate::registry::combinators::IX_CONST_MOVE;
use crate::tag::Tagged;

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAM - Reflected syntax node (AST)
// ═══════════════════════════════════════════════════════════════════════════

/// Constant program node: reflected syntax node (move semantics).
pub type ConstMove<F> = Tagged<IX_CONST_MOVE, F>;

/// Construct a constant program node (move semantics).
#[inline]
pub const fn const_move<F>(f: F) -> ConstMove<F> {
    Tagged::new(f)
}

// ═══════════════════════════════════════════════════════════════════════════
// SEMANTICS - Runnable op implementation
// ═══════════════════════════════════════════════════════════════════════════

/// Constant op: f : A → F (clone semantics)
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstCOp<F>(pub F);

/// Type-level op: A ↦ F
impl<Args, F> OpTy<Args> for ConstCOp<F> {
    type OutTy = F;
}

/// Term-level op: a ↦ f (requires Clone)
impl<Args, F: Clone> OpOnce<Args> for ConstCOp<F> {
    type OutVal = F;

    #[inline]
    fn run(self, _args: Args) -> Self::OutVal {
        self.0.clone()
    }
}

/// Runnable op: Args -> T by move (no Clone bound).
///
/// This is the key optimization vs `ConstCOp<T>` which requires `T: Clone`.
/// `ConstMOp` moves the captured value out, making it suitable for non-Clone types.
/// Like `ConstCOp`, it ignores the input argument.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstMOp<T>(pub T);

impl<T> ConstMOp<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

/// Type-level op: Args ↦ T
impl<Args, T> OpTy<Args> for ConstMOp<T> {
    type OutTy = T;
}

/// Term-level op: args ↦ t (moves, no Clone bound)
impl<Args, T> OpOnce<Args> for ConstMOp<T> {
    type OutVal = T;

    #[inline]
    fn run(self, _args: Args) -> Self::OutVal {
        self.0
    }
}
