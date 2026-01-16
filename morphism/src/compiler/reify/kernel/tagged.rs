//! Closed-table reify pilot: `Tagged` combinators domain (Wrap/Unwrap).
//!
//! This module proves the closed-table reification pipeline on a **tiny** set of keys
//! (two entries). This avoids the "missing impl cascade" while establishing the core
//! mechanics we will later scale to other domains.

use crate::compiler::reify::ReifyTable;
use crate::compiler::table::ReifyByTable;
use crate::{Domain, Key, OpOnce, Tagged};
use crate::{D_TAGGED, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: ReifyTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// Tagged-domain keys: dispatch using the tagged-domain table (Wrap/Unwrap).
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_TAGGED>, R_SEMANTICS, Ix>, Payload>>
    for ReifyTable<Input>
where
    ReifyByTable<Input, (CompileWrapTagged<Input>, CompileUnwrapTagged<Input>)>:
        OpOnce<Tagged<Key<Domain<D_TAGGED>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal =
        <ReifyByTable<Input, (CompileWrapTagged<Input>, CompileUnwrapTagged<Input>)> as OpOnce<
            Tagged<Key<Domain<D_TAGGED>, R_SEMANTICS, Ix>, Payload>,
        >>::OutVal;

    #[inline]
    fn run(self, prog: Tagged<Key<Domain<D_TAGGED>, R_SEMANTICS, Ix>, Payload>) -> Self::OutVal {
        ReifyByTable::<Input, _>::new(reify_table_tagged::<Input>()).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition and handlers
// ─────────────────────────────────────────────────────────────────────────────

use core::marker::PhantomData;

use crate::{UnwrapTaggedOp, WrapTaggedOp};

/// Compile op: payload -> runnable op for `IX_WRAP_TAGGED`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileWrapTagged<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileWrapTagged<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, K> OpOnce<PhantomData<fn() -> K>> for CompileWrapTagged<Input> {
    type OutVal = WrapTaggedOp<K>;

    #[inline]
    fn run(self, _payload: PhantomData<fn() -> K>) -> Self::OutVal {
        WrapTaggedOp::<K>::new()
    }
}

/// Compile op: payload -> runnable op for `IX_UNWRAP_TAGGED`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileUnwrapTagged<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileUnwrapTagged<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, K> OpOnce<PhantomData<fn() -> K>> for CompileUnwrapTagged<Input> {
    type OutVal = UnwrapTaggedOp<K>;

    #[inline]
    fn run(self, _payload: PhantomData<fn() -> K>) -> Self::OutVal {
        UnwrapTaggedOp::<K>::new()
    }
}

/// Closed-table for the Tagged-domain reify rules (v1, small table).
///
/// Index layout must match the registry:
/// - `IX_WRAP_TAGGED`:  U0
/// - `IX_UNWRAP_TAGGED`: U1
#[inline]
pub const fn reify_table_tagged<Input>() -> (CompileWrapTagged<Input>, CompileUnwrapTagged<Input>) {
    (CompileWrapTagged::new(), CompileUnwrapTagged::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::table::ReifyByTable;
    use crate::Tagged;
    use crate::{unwrap_tagged, wrap_tagged};

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct MyKey;

    #[test]
    fn reify_by_table_compiles_wrap_tagged() {
        let reify = ReifyByTable::<(), _>::new(reify_table_tagged::<()>());

        let prog = wrap_tagged::<MyKey>();
        let op = reify.run(prog);

        let out = op.run(123_i32);
        assert_eq!(out, Tagged::new(123_i32));
    }

    #[test]
    fn reify_by_table_compiles_unwrap_tagged() {
        let reify = ReifyByTable::<(), _>::new(reify_table_tagged::<()>());

        let prog = unwrap_tagged::<MyKey>();
        let op = reify.run(prog);

        let out = op.run(Tagged::new("hello"));
        assert_eq!(out, "hello");
    }
}
