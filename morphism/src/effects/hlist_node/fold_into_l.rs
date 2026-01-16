use crate::Tagged;
use crate::IX_FOLD_HLISTNODE_L;

/// High-level program: fold-left over `HlistNode<Children>`'s children (Into variant).
///
/// This is the wrapper-eliminating version: unwrap → fold → return accumulator.
/// Lowers to: `unwrap_hlist |> foldl_prog(lowered_f, acc)` (with fold lowered using Children input)
pub type FoldHlistNodeLIntoAccProg<FProg, Acc> = Tagged<IX_FOLD_HLISTNODE_L, (FProg, Acc)>;

#[inline]
pub const fn fold_hlist_node_l_into_acc_prog<FProg, Acc>(
    f_prog: FProg,
    acc: Acc,
) -> FoldHlistNodeLIntoAccProg<FProg, Acc> {
    Tagged::new((f_prog, acc))
}
