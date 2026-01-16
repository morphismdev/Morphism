//! Closed-table lowering: Tagged domain (wrap/unwrap).
//!
//! Currently these are already base nodes, so lowering is identity.

use core::marker::PhantomData;

use crate::compiler::lower::LowerTable;
use crate::compiler::table::LowerByTable;
use crate::{
    // constructors
    unwrap_tagged,
    wrap_tagged,
    OpOnce,
    // semantic aliases
    UnwrapTagged,
    WrapTagged,
};
use crate::{Domain, Key, Tagged, D_TAGGED, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: LowerTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// Tagged-domain lowering: dispatch using the tagged-domain table (wrap/unwrap).
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_TAGGED>, R_SEMANTICS, Ix>, Payload>>
    for LowerTable<Input>
where
    LowerByTable<Input, (LowerWrapTagged, LowerUnwrapTagged)>:
        OpOnce<Tagged<Key<Domain<D_TAGGED>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <LowerByTable<Input, (LowerWrapTagged, LowerUnwrapTagged)> as OpOnce<
        Tagged<Key<Domain<D_TAGGED>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(self, prog: Tagged<Key<Domain<D_TAGGED>, R_SEMANTICS, Ix>, Payload>) -> Self::OutVal {
        LowerByTable::<Input, _>::new(lower_table_tagged()).run(prog)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerWrapTagged;

impl<K> OpOnce<PhantomData<fn() -> K>> for LowerWrapTagged {
    type OutVal = WrapTagged<K>;
    #[inline]
    fn run(self, _: PhantomData<fn() -> K>) -> Self::OutVal {
        wrap_tagged::<K>()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerUnwrapTagged;

impl<K> OpOnce<PhantomData<fn() -> K>> for LowerUnwrapTagged {
    type OutVal = UnwrapTagged<K>;
    #[inline]
    fn run(self, _: PhantomData<fn() -> K>) -> Self::OutVal {
        unwrap_tagged::<K>()
    }
}

/// Lowering table for Tagged-domain (indices U0..U1).
#[inline]
pub const fn lower_table_tagged() -> (LowerWrapTagged, LowerUnwrapTagged) {
    (LowerWrapTagged, LowerUnwrapTagged)
}
