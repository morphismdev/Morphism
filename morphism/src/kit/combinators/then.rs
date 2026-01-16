use crate::kit::op::{OpOnce, OpTy, OpTyOut};
use crate::registry::combinators::IX_THEN;
use crate::tag::Tagged;
use core::fmt::Debug;

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAM - Reflected syntax node (AST)
// ═══════════════════════════════════════════════════════════════════════════

/// Composition program node: reflected syntax node for (F ∘ G).
///
/// Pipeline style: `then(f, g)` means run `f` first, then `g`.
/// This matches Rust iterator style: `input |> f |> g`.
pub type Then<F, G> = Tagged<IX_THEN, (F, G)>;

/// Construct a composition program node.
///
/// Pipeline style: `then(f, g)` means run `f` first, then `g`.
#[inline]
pub const fn then<F, G>(f: F, g: G) -> Then<F, G> {
    Tagged::new((f, g))
}

// ═══════════════════════════════════════════════════════════════════════════
// SEMANTICS - Runnable op implementation
// ═══════════════════════════════════════════════════════════════════════════

/// Dimap: contravariant map on the left, covariant map on the right.
/// `Dimap<L, R, P> = Compose<R, Compose<P, L>>`
pub type Dimap<L, R, P> = ThenOp<R, ThenOp<P, L>>;

/// Composition of two ops: (F ∘ G) where F is applied first, then G.
///
/// This matches Rust iterator style: `input |> f |> g` means run `f` then `g`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThenOp<F, G> {
    f: F,
    g: G,
}

impl<F, G> ThenOp<F, G> {
    pub const fn new(f: F, g: G) -> Self {
        Self { f, g }
    }
}

/// Type-level composition:
///   (F ∘ G) A = G (F A)
impl<Args, F, G> OpTy<Args> for ThenOp<F, G>
where
    F: OpTy<Args>,
    G: OpTy<OpTyOut<F, Args>>,
{
    type OutTy = OpTyOut<G, OpTyOut<F, Args>>;
}

/// Term-level composition:
///   (f ∘ g) a = g (f a)
///
/// Pipeline style: input |> f |> g
impl<Args, F, G> OpOnce<Args> for ThenOp<F, G>
where
    F: OpOnce<Args>,
    G: OpOnce<<F as OpOnce<Args>>::OutVal>,
{
    type OutVal = <G as OpOnce<<F as OpOnce<Args>>::OutVal>>::OutVal;

    #[inline]
    fn run(self, args: Args) -> Self::OutVal {
        let intermediate = self.f.run(args);
        self.g.run(intermediate)
    }
}
