use crate::kit::op::{OpOnce, OpTy, OpTyOut};
use crate::kit::tokens::NullaryToken;
use crate::registry::combinators::IX_APPLY;
use crate::tag::Tagged;

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAM - Reflected syntax node (AST)
// ═══════════════════════════════════════════════════════════════════════════

/// Apply program node: reflected syntax node.
///
/// This is a nullary combinator - the `(Op, Arg)` pair comes from the input
/// to the op, not from the program node payload.
pub type ApplyProg = Tagged<IX_APPLY, NullaryToken>;

/// Construct an apply program node.
#[inline]
pub const fn apply() -> ApplyProg {
    Tagged::new(NullaryToken)
}

// ═══════════════════════════════════════════════════════════════════════════
// SEMANTICS - Runnable op implementation
// ═══════════════════════════════════════════════════════════════════════════

/// Generic apply combinator: (Op, Arg) → Op(Arg)
///
/// This combinator takes a pair `(Op, Arg)` where `Op` is an operation
/// and `Arg` is its argument, and applies `Op` to `Arg`.
///
/// This is the fundamental "evaluation" combinator that turns a pair
/// of (operation, argument) into the result of applying the operation.
///
/// - Type-level: `(Op, Arg)` → `OpTyOut<Op, Arg>`
/// - Value-level: `(op, arg)` → `op.run(arg)`
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApplyOp;

/// Type-level apply: (Op, Arg) → Op(Arg)
impl<Op, Arg> OpTy<(Op, Arg)> for ApplyOp
where
    Op: OpTy<Arg>,
{
    type OutTy = OpTyOut<Op, Arg>;
}

/// Term-level apply: (op, arg) → op.run(arg)
impl<Op, Arg> OpOnce<(Op, Arg)> for ApplyOp
where
    Op: OpOnce<Arg>,
{
    type OutVal = <Op as OpOnce<Arg>>::OutVal;

    #[inline]
    fn run(self, (op, arg): (Op, Arg)) -> Self::OutVal {
        op.run(arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper: an operation that returns a constant value
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ConstVal<T>(T);

    impl<T: Clone, Arg> OpTy<Arg> for ConstVal<T> {
        type OutTy = T;
    }

    impl<T: Clone, Arg> OpOnce<Arg> for ConstVal<T> {
        type OutVal = T;

        #[inline]
        fn run(self, _arg: Arg) -> Self::OutVal {
            self.0.clone()
        }
    }

    #[test]
    fn type_level_apply() {
        type Op = ConstVal<u32>;
        type Arg = i32;
        type Result = OpTyOut<ApplyOp, (Op, Arg)>;

        // ApplyOp should apply Op to Arg, producing Op::OutTy
        use crate::assert_type_eq;
        assert_type_eq::<Result, u32>();
    }

    #[test]
    fn value_level_apply() {
        let op = ConstVal(100u32);
        let arg = 42i32;
        let result = ApplyOp.run((op, arg));

        assert_eq!(result, 100u32);
    }

    #[test]
    fn apply_with_id_arr() {
        use crate::IdOp;

        let op = IdOp;
        let arg = 42i32;
        let result = ApplyOp.run((op, arg));

        assert_eq!(result, 42i32);
    }
}
