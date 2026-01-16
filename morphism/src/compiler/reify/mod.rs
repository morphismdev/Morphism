use core::marker::PhantomData;

/// Closed-table reifier: supports a small subset of domains via per-domain tables.
///
/// Domain-specific routing impls are co-located with their table definitions in `reify/kernel/*.rs`:
/// - `op.rs`: D_OP domain
/// - `tagged.rs`: D_TAGGED domain
/// - `combinators.rs`: D_COMBINATORS domain
/// - `hlist.rs`: D_HLIST domain
/// - `hlist_fold.rs`: D_HLIST_FOLD domain
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReifyTable<Input>(PhantomData<fn() -> Input>);

impl<Input> ReifyTable<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

pub(crate) mod kernel;
