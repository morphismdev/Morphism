//! Compiler-only: closed table selection.
//!
//! This module intentionally only exposes `ClosedTableGetAt*` to keep compiler table
//! selection bounded and compile-time predictable.

use core::marker::PhantomData;

use crate::{HlistFlatGetAt, OpOnce, OpTy};
use crate::{
    U0, U1, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U2, U20, U21, U22, U23, U3, U4, U5,
    U6, U7, U8, U9,
};

/// Closed-table selection (flat semantics), **bounded** to a small, explicit index set.
///
/// Why this exists:
/// - Generic `HlistFlatGetAt<Ix>` works great when `Ix` is known, but in generic contexts `Ix` can
///   remain an inference variable (`_`), and rustc will eagerly expand a huge `HlistFlatGetAt<_>`
///   obligation tree and overflow.
///
/// This selector avoids that failure mode by **not having a generic `Ix` impl**. Instead, it provides
/// concrete impls for a bounded set (`U0..U23`). If `Ix` is not fixed, rustc can't expand the heavy
/// arithmetic/branching machinery, and typechecking stays tractable.
///
/// Semantics (flat):
/// - tables are viewed as a flat logical list
/// - chunked encoding `(E0..E10, Tail)` is treated as `E0..E10 ++ Tail[..]`
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClosedTableGetAtFlat<Ix>(PhantomData<fn() -> Ix>);

impl<Ix> ClosedTableGetAtFlat<Ix> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

/// Convenience alias: closed tables always use flat semantics.
pub type ClosedTableGetAt<Ix> = ClosedTableGetAtFlat<Ix>;

macro_rules! impl_closed_get_at_via_hlist_flat {
    ($Ix:ty) => {
        impl<Table> OpTy<Table> for ClosedTableGetAtFlat<$Ix>
        where
            HlistFlatGetAt<$Ix>: OpTy<Table>,
        {
            type OutTy = <HlistFlatGetAt<$Ix> as OpTy<Table>>::OutTy;
        }

        impl<Table> OpOnce<Table> for ClosedTableGetAtFlat<$Ix>
        where
            HlistFlatGetAt<$Ix>: OpOnce<Table>,
        {
            type OutVal = <HlistFlatGetAt<$Ix> as OpOnce<Table>>::OutVal;

            #[inline]
            fn run(self, table: Table) -> Self::OutVal {
                HlistFlatGetAt::<$Ix>::new().run(table)
            }
        }
    };
}

// U0..U23: delegate to HlistFlatGetAt with flat semantics.
impl_closed_get_at_via_hlist_flat!(U0);
impl_closed_get_at_via_hlist_flat!(U1);
impl_closed_get_at_via_hlist_flat!(U2);
impl_closed_get_at_via_hlist_flat!(U3);
impl_closed_get_at_via_hlist_flat!(U4);
impl_closed_get_at_via_hlist_flat!(U5);
impl_closed_get_at_via_hlist_flat!(U6);
impl_closed_get_at_via_hlist_flat!(U7);
impl_closed_get_at_via_hlist_flat!(U8);
impl_closed_get_at_via_hlist_flat!(U9);
impl_closed_get_at_via_hlist_flat!(U10);
impl_closed_get_at_via_hlist_flat!(U11);
impl_closed_get_at_via_hlist_flat!(U12);
impl_closed_get_at_via_hlist_flat!(U13);
impl_closed_get_at_via_hlist_flat!(U14);
impl_closed_get_at_via_hlist_flat!(U15);
impl_closed_get_at_via_hlist_flat!(U16);
impl_closed_get_at_via_hlist_flat!(U17);
impl_closed_get_at_via_hlist_flat!(U18);
impl_closed_get_at_via_hlist_flat!(U19);
impl_closed_get_at_via_hlist_flat!(U20);
impl_closed_get_at_via_hlist_flat!(U21);
impl_closed_get_at_via_hlist_flat!(U22);
impl_closed_get_at_via_hlist_flat!(U23);
