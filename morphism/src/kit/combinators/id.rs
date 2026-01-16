use crate::kit::op::{OpOnce, OpTy};
use crate::kit::tokens::NullaryToken;
use crate::registry::combinators::IX_ID;
use crate::tag::Tagged;

// ═══════════════════════════════════════════════════════════════════════════
// IDENTITY - Morphism level, "do nothing" transformation
// ═══════════════════════════════════════════════════════════════════════════

/// Identity program node: reflected syntax node.
pub type Id = Tagged<IX_ID, NullaryToken>;

/// Construct an identity program node.
#[inline]
pub const fn id() -> Id {
    Id::new(NullaryToken)
}

// ═══════════════════════════════════════════════════════════════════════════
// SEMANTICS - Runnable op implementation
// ═══════════════════════════════════════════════════════════════════════════

/// Identity op: id : A → A
///
/// The "do nothing" morphism. Used when a node wants to indicate
/// "apply no transformation" (e.g., `NewTypeNode` returns `IdOp`).
///
/// - Value-level: a ↦ a
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdOp;

/// Type-level op: A → A
impl<Args> OpTy<Args> for IdOp {
    type OutTy = Args;
}

/// Term-level op: a ↦ a
impl<Args> OpOnce<Args> for IdOp {
    type OutVal = Args;

    #[inline]
    fn run(self, args: Args) -> Self::OutVal {
        args
    }
}
