use crate::Tagged;
use crate::IX_MAP_HLISTNODE;

/// High-level program: map operation over `HlistNode`'s children (In variant).
///
/// This is the wrapper-aware version: unwrap → map → wrap.
/// Payload is the operation syntax term itself.
/// This must be lowered to base combinators before compilation.
///
/// **Lowering happens via closed pipeline**: `LowerTable` routes through the closed
/// generic lowering table (`LowerMapHlistNode`).
pub type MapInHlistNodeProg<OpLift> = Tagged<IX_MAP_HLISTNODE, OpLift>;

/// Construct a high-level map hlist node program (In variant).
///
/// This is the advanced API for composing programs.
#[inline]
pub const fn map_in_hlist_node_prog<OpLift>(op: OpLift) -> MapInHlistNodeProg<OpLift> {
    Tagged::new(op)
}
