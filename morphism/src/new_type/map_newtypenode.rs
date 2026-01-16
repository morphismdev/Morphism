use crate::Tagged;
use crate::IX_MAP_NEWTYPENODE;

/// High-level program: map operation over `NewTypeNode`'s inner payload (pure).
///
/// Payload is the operation syntax term itself.
/// This must be lowered to base combinators before compilation.
///
/// Lowering is handled by the closed generic lowering table (`LowerMapNewTypeNode`).
pub type MapNewTypeNodeProg<OpLift> = Tagged<IX_MAP_NEWTYPENODE, OpLift>;

/// Construct a high-level map newtypenode program (pure).
///
/// This is the program-level API for composing programs.
/// Surface crates provide ergonomic wrappers that lift runtime ops into programs.
#[inline]
pub const fn map_newtypenode_prog<OpLift>(op: OpLift) -> MapNewTypeNodeProg<OpLift> {
    Tagged::new(op)
}
