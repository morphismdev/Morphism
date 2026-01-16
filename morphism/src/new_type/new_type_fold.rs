use crate::{Tagged, IX_FOLD_NEWTYPENODE_L};

/// High-level program: fold-left over `NewTypeNode<_>`'s (non-existent) children.
///
/// Payload is `(f_prog, acc)`, but newtypenode has no children so it returns `acc` unchanged.
///
/// Lowering is handled by the closed generic lowering table (`LowerFoldNewTypeNodeL`).
pub type FoldNewTypeNodeLProg<FProg, Acc> = Tagged<IX_FOLD_NEWTYPENODE_L, (FProg, Acc)>;

/// Construct a high-level fold newtypenode prog (pure).
///
/// This is the program-level API for composing programs.
/// Surface crates provide ergonomic wrappers that lift runtime ops into programs.
#[inline]
pub const fn fold_newtypenode_l_prog<FProg, Acc>(
    f_prog: FProg,
    acc: Acc,
) -> FoldNewTypeNodeLProg<FProg, Acc> {
    Tagged::new((f_prog, acc))
}
