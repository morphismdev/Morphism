//! Closed-table reify: HList domain (kernel-only).
//!
//! Routes recursion through `ReifyTable` for closed-world reification.
//!
//! Only kernel keys that survive lowering are present here (dense U0..U0).

use crate::compiler::reify::ReifyTable;
use crate::compiler::table::ReifyByTable;
use crate::{Domain, Key, OpOnce, Tagged};
use crate::{D_HLIST, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: ReifyTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// HList semantics keys: dispatch using the hlist-domain table.
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_HLIST>, R_SEMANTICS, Ix>, Payload>>
    for ReifyTable<Input>
where
    ReifyByTable<Input, ReifyTableHlist<Input>>:
        OpOnce<Tagged<Key<Domain<D_HLIST>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <ReifyByTable<Input, ReifyTableHlist<Input>> as OpOnce<
        Tagged<Key<Domain<D_HLIST>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(self, prog: Tagged<Key<Domain<D_HLIST>, R_SEMANTICS, Ix>, Payload>) -> Self::OutVal {
        ReifyByTable::<Input, _>::new(reify_table_hlist::<Input>()).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition and handlers
// ─────────────────────────────────────────────────────────────────────────────

use core::marker::PhantomData;

use crate::{HlistPushBack, NullaryToken};

/// Reify table for HlistDomain semantics keys (indices `U0..U0`).
///
/// Dense encoding: only kernel keys that survive lowering.
pub type ReifyTableHlist<Input> = (
    CompileHlistPushBack<Input>, // U0: IX_HLIST_PUSH_BACK
);

#[inline]
pub const fn reify_table_hlist<Input>() -> ReifyTableHlist<Input> {
    (CompileHlistPushBack::new(),)
}

// U0: push_back (nullary program -> runnable op)

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileHlistPushBack<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileHlistPushBack<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input> OpOnce<NullaryToken> for CompileHlistPushBack<Input> {
    type OutVal = HlistPushBack;
    #[inline]
    fn run(self, _: NullaryToken) -> Self::OutVal {
        HlistPushBack
    }
}
