use crate::kit::op::{OpOnce, OpTy};
use crate::registry::combinators::{IX_PARTIAL_L, IX_PARTIAL_R};
use crate::tag::Tagged;

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAMS - Reflected syntax nodes (AST)
// ═══════════════════════════════════════════════════════════════════════════

/// Left partial application program node: reflected syntax node.
pub type PartialL<Env, Op> = Tagged<IX_PARTIAL_L, (Env, Op)>;

/// Right partial application program node: reflected syntax node.
pub type PartialR<Op, Env> = Tagged<IX_PARTIAL_R, (Op, Env)>;

/// Construct a left partial application program node.
#[inline]
pub const fn partial_l<Env, Op>(env: Env, op: Op) -> PartialL<Env, Op> {
    Tagged::new((env, op))
}

/// Construct a right partial application program node.
#[inline]
pub const fn partial_r<Op, Env>(op: Op, env: Env) -> PartialR<Op, Env> {
    Tagged::new((op, env))
}

// ═══════════════════════════════════════════════════════════════════════════
// SEMANTICS - Runnable op implementations
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PartialLOp<Env, Op> {
    pub env: Env,
    pub op: Op,
}

// Type-level: A ↦ Output = Op::Output<(Env, A)>
impl<Env, Op, A> OpTy<A> for PartialLOp<Env, Op>
where
    Op: OpTy<(Env, A)>,
{
    type OutTy = <Op as OpTy<(Env, A)>>::OutTy;
}

// Term-level: a ↦ op.run((env, a))
impl<Env, Op, A> OpOnce<A> for PartialLOp<Env, Op>
where
    Op: OpOnce<(Env, A)>,
{
    type OutVal = <Op as OpOnce<(Env, A)>>::OutVal;

    #[inline]
    fn run(self, a: A) -> Self::OutVal {
        self.op.run((self.env, a))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PartialROp<Op, Env> {
    pub op: Op,
    pub env: Env,
}

// Type-level: A ↦ Output = Op::Output<(A, Env)>
impl<Op, Env, A> OpTy<A> for PartialROp<Op, Env>
where
    Op: OpTy<(A, Env)>,
{
    type OutTy = <Op as OpTy<(A, Env)>>::OutTy;
}

// Term-level: a ↦ op.run((a, env))
impl<Op, Env, A> OpOnce<A> for PartialROp<Op, Env>
where
    Op: OpOnce<(A, Env)>,
{
    type OutVal = <Op as OpOnce<(A, Env)>>::OutVal;

    #[inline]
    fn run(self, a: A) -> Self::OutVal {
        self.op.run((a, self.env))
    }
}
