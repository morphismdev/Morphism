use crate::Tagged;
use crate::IX_FOLD_CHILDREN_L;

/// High-level prog: fold-left over children of a Generic node — dispatcher.
///
/// Dispatches to NewTypeNode/HlistNode based on input type during closed lowering.
///
/// **Dispatch happens during closed lowering**: `LowerTable` routes through the closed
/// generic lowering table (`LowerFoldChildrenL`), which dispatches based on the `Input` type.
pub type FoldChildrenLProg<FOrOpsProg, Acc> = Tagged<IX_FOLD_CHILDREN_L, (FOrOpsProg, Acc)>;

/// Construct a high-level fold children prog (pure).
///
/// This is the program-level API for composing programs.
/// Surface crates provide ergonomic wrappers that lift runtime ops into programs.
#[inline]
pub const fn fold_children_l_prog<FProg, Acc>(
    f_prog: FProg,
    acc: Acc,
) -> FoldChildrenLProg<FProg, Acc> {
    Tagged::new((f_prog, acc))
}
