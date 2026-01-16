//! Closed-table lowering: HList domain (kernel-only).
//!
//! Lowering rules for kernel keys:
//! - push_back: identity (nullary)
//!
//! Only kernel keys that survive lowering are present here (dense `U0..U0`).

use crate::compiler::lower::LowerTable;
use crate::compiler::table::LowerByTable;
use crate::{Domain, Key, OpOnce, Tagged};
use crate::{D_HLIST, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: LowerTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// HList semantics lowering: dispatch using the hlist-domain table.
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_HLIST>, R_SEMANTICS, Ix>, Payload>>
    for LowerTable<Input>
where
    LowerByTable<Input, LowerTableHlist>:
        OpOnce<Tagged<Key<Domain<D_HLIST>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <LowerByTable<Input, LowerTableHlist> as OpOnce<
        Tagged<Key<Domain<D_HLIST>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(self, prog: Tagged<Key<Domain<D_HLIST>, R_SEMANTICS, Ix>, Payload>) -> Self::OutVal {
        LowerByTable::<Input, _>::new((LowerHlistPushBack,)).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition and handlers
// ─────────────────────────────────────────────────────────────────────────────

use crate::NullaryToken;
use crate::{
    // constructors
    hlist_push_back_prog,
    // semantic aliases
    HlistPushBackProg,
};

/// Lowering table for HlistDomain semantics keys (indices `U0..U0`).
///
/// Dense encoding: only kernel keys that survive lowering.
pub type LowerTableHlist = (
    LowerHlistPushBack, // U0: IX_HLIST_PUSH_BACK
);

// U0: nullary push_back (identity)

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerHlistPushBack;

impl OpOnce<NullaryToken> for LowerHlistPushBack {
    type OutVal = HlistPushBackProg;
    #[inline]
    fn run(self, _: NullaryToken) -> Self::OutVal {
        hlist_push_back_prog()
    }
}
