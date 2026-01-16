//! Bimap combinator: applies two ops to the components of a pair.
//!
//! This combinator takes a pair `(A, B)` and two ops `F: A -> FOut` and `G: B -> GOut`,
//! then returns `(FOut, GOut)` by applying `F` to the first component and `G` to the second.

use crate::kit::op::{OpOnce, OpTy};
use crate::registry::combinators::IX_BIMAP;
use crate::tag::Tagged;

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAM - Reflected syntax node (AST)
// ═══════════════════════════════════════════════════════════════════════════

/// Bimap program node: reflected syntax node for (F, G).
///
/// Takes a pair `(A, B)` and applies `F` to `A` and `G` to `B`.
pub type Bimap<F, G> = Tagged<IX_BIMAP, (F, G)>;

/// Construct a bimap program node.
///
/// Takes two ops `F` and `G`, and applies them to the components of a pair.
#[inline]
pub const fn bimap<F, G>(f: F, g: G) -> Bimap<F, G> {
    Tagged::new((f, g))
}

// ═══════════════════════════════════════════════════════════════════════════
// SEMANTICS - Runnable op implementation
// ═══════════════════════════════════════════════════════════════════════════

/// Bimap op: applies `F` to the first component and `G` to the second component of a pair.
///
/// Takes `(A, B)` and returns `(F::OutVal, G::OutVal)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BimapOp<F, G> {
    pub f: F,
    pub g: G,
}

impl<A, B, F, G> OpTy<(A, B)> for BimapOp<F, G>
where
    F: OpTy<A>,
    G: OpTy<B>,
{
    type OutTy = (F::OutTy, G::OutTy);
}

impl<A, B, F, G> OpOnce<(A, B)> for BimapOp<F, G>
where
    F: OpOnce<A>,
    G: OpOnce<B>,
{
    type OutVal = (F::OutVal, G::OutVal);

    #[inline]
    fn run(self, (a, b): (A, B)) -> Self::OutVal {
        (self.f.run(a), self.g.run(b))
    }
}
