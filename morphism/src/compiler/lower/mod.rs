use core::marker::PhantomData;

/// Closed-table lowerer: supports a small subset of domains via per-domain tables.
///
/// Domain-specific routing impls are co-located with their table definitions:
/// - `lower/kernel/*.rs`: kernel domains (op, tagged, combinators, hlist, hlist_fold)
/// - `lower/desugar/*.rs`: frontend domains (generic)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerTable<Input>(PhantomData<fn() -> Input>);

impl<Input> LowerTable<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

pub(crate) mod kernel;
pub(crate) mod desugar;