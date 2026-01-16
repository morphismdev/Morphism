//! Closed-table lowering: Op lift domain (`IX_OP_LIFT`).
//!
//! Lowering is identity (pass-through): `OpLift(op) -> OpLift(op)`.

use crate::compiler::lower::LowerTable;
use crate::compiler::table::LowerByTable;
use crate::{Domain, Key, OpOnce, Tagged};
use crate::{D_OP, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: LowerTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// Op-lift lowering: identity (pass-through).
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_OP>, R_SEMANTICS, Ix>, Payload>>
    for LowerTable<Input>
where
    LowerByTable<Input, (LowerOpLift,)>:
        OpOnce<Tagged<Key<Domain<D_OP>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <LowerByTable<Input, (LowerOpLift,)> as OpOnce<
        Tagged<Key<Domain<D_OP>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(self, prog: Tagged<Key<Domain<D_OP>, R_SEMANTICS, Ix>, Payload>) -> Self::OutVal {
        LowerByTable::<Input, _>::new(lower_table_op()).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition and handlers
// ─────────────────────────────────────────────────────────────────────────────

use crate::{
    // constructors
    op_lift,
    // semantic aliases
    OpLift,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerOpLift;

impl<Op> OpOnce<Op> for LowerOpLift {
    type OutVal = OpLift<Op>;
    #[inline]
    fn run(self, op: Op) -> Self::OutVal {
        op_lift(op)
    }
}

/// Lowering table for Op-domain (single entry at U0).
#[inline]
pub const fn lower_table_op() -> (LowerOpLift,) {
    (LowerOpLift,)
}
