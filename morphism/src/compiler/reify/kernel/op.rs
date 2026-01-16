//! Closed-table reify: Op lift domain (`IX_OP_LIFT`).
//!
//! `IX_OP_LIFT` is the opaque escape hatch: it reifies by unwrapping the payload op as-is.

use crate::compiler::reify::ReifyTable;
use crate::compiler::table::ReifyByTable;
use crate::{Domain, Key, OpOnce, Tagged};
use crate::{D_OP, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: ReifyTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// Op-lift keys: dispatch using the op-domain table (single entry).
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_OP>, R_SEMANTICS, Ix>, Payload>>
    for ReifyTable<Input>
where
    ReifyByTable<Input, (CompileOpLift<Input>,)>:
        OpOnce<Tagged<Key<Domain<D_OP>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <ReifyByTable<Input, (CompileOpLift<Input>,)> as OpOnce<
        Tagged<Key<Domain<D_OP>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(self, prog: Tagged<Key<Domain<D_OP>, R_SEMANTICS, Ix>, Payload>) -> Self::OutVal {
        ReifyByTable::<Input, _>::new(reify_table_op::<Input>()).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition and handlers
// ─────────────────────────────────────────────────────────────────────────────

use core::marker::PhantomData;

/// Compile op for `IX_OP_LIFT`: payload op -> runnable op (identity).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileOpLift<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileOpLift<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, Op> OpOnce<Op> for CompileOpLift<Input> {
    type OutVal = Op;

    #[inline]
    fn run(self, op: Op) -> Self::OutVal {
        op
    }
}

/// Closed-table for the Op-lift domain reify rules (single entry at `U0`).
#[inline]
pub const fn reify_table_op<Input>() -> (CompileOpLift<Input>,) {
    (CompileOpLift::new(),)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::table::ReifyByTable;
    use crate::{op_lift, OpOnce};

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct Inc;

    impl OpOnce<i32> for Inc {
        type OutVal = i32;
        fn run(self, x: i32) -> Self::OutVal {
            x + 1
        }
    }

    #[test]
    fn reify_by_table_compiles_op_lift_by_unwrap() {
        let reify = ReifyByTable::<i32, _>::new(reify_table_op::<i32>());

        let prog = op_lift(Inc);
        let op = reify.run(prog);

        let out = op.run(41);
        assert_eq!(out, 42);
    }
}
