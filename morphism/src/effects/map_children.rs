use crate::Tagged;
use crate::IX_MAP_CHILDREN;

/// High-level program: map over children of a Generic node (NewTypeNode/HlistNode) — dispatcher.
///
/// Payload is the operation syntax term itself.
/// This must be lowered to base combinators before compilation.
///
/// **Dispatch happens during closed lowering**: `LowerTable` routes through the closed
/// generic lowering table (`LowerMapChildren`), which dispatches based on the `Input` type
/// (NewTypeNode vs HlistNode), lowering to the appropriate specialized program.
pub type MapChildrenProg<OpLift> = Tagged<IX_MAP_CHILDREN, OpLift>;

/// Construct a high-level map children program (pure).
///
/// This is the program-level API for composing programs.
/// Surface crates provide ergonomic wrappers that lift runtime ops into programs.
#[inline]
pub const fn map_children_prog<OpLift>(op_lift: OpLift) -> MapChildrenProg<OpLift> {
    Tagged::new(op_lift)
}
