use crate::kit::op::{OpOnce, OpTy};
use crate::registry::combinators::IX_FANOUT;
use crate::tag::Tagged;

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAM - Reflected syntax node (AST)
// ═══════════════════════════════════════════════════════════════════════════

/// Fanout program node: reflected syntax node for (F, G).
pub type Fanout<F, G> = Tagged<IX_FANOUT, (F, G)>;

/// Construct a fanout program node.
#[inline]
pub const fn fanout<F, G>(f: F, g: G) -> Fanout<F, G> {
    Tagged::new((f, g))
}

// ═══════════════════════════════════════════════════════════════════════════
// SEMANTICS - Runnable op implementation
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FanoutOp<F, G> {
    pub f: F,
    pub g: G,
}

/// Type-level fanout: (F, G) A = (F A, G A)
impl<Args, F, G> OpTy<Args> for FanoutOp<F, G>
where
    F: OpTy<Args>,
    G: OpTy<Args>,
{
    type OutTy = (F::OutTy, G::OutTy);
}

impl<Args: Clone, F, G> OpOnce<Args> for FanoutOp<F, G>
where
    F: OpOnce<Args>,
    G: OpOnce<Args>,
{
    type OutVal = (F::OutVal, G::OutVal);

    #[inline]
    fn run(self, args: Args) -> Self::OutVal {
        (self.f.run(args.clone()), self.g.run(args))
    }
}
