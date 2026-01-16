//! Tagging primitives for reflected programs.
//!
//! This module defines the minimal type-level vocabulary used to identify and carry
//! reflected program nodes throughout the showcase crate.
//!
//! The types in this module are **structure only**. They do not define semantics,
//! compilation, lowering, or dispatch. Those concerns live in separate modules.
//!
//! ## Definitions
//!
//! - `Domain<Code>`: a namespace marker. `Code` is a globally unique type-level natural.
//! - `Key<D, R, N>`: an identifier within a domain `D`, with role `R`, indexed by `N`.
//!   - `D`: the domain (e.g., `Domain<D_BOOL>`)
//!   - `R`: the role code (e.g., `R_SYNTAX`, `R_SEMANTICS`)
//!   - `N`: the index (contiguous within each `(Domain, Role)` pair)
//! - `Tagged<TagKey, Payload>`: a payload `Payload` tagged with a key `TagKey`. This is the canonical container
//!   for reflected syntax nodes.
//!
//! ## Invariants
//!
//! - Domain codes are globally unique. They are allocated in `registry::codes`.
//! - Role codes are defined in `registry::roles`.
//! - Within a `(Domain, Role)` pair, indices are contiguous and unique.
//! - `Key<..>` values are zero-sized and must remain so.
//! - `Tagged<TagKey, Payload>` is `#[repr(transparent)]` and must remain so. This guarantees
//!   that `Tagged<TagKey, Payload>` has the same runtime layout as `Payload`.
//!
//! ## Non-goals
//!
//! - No semantic operations (ops) over keys or tags.
//! - No registry declarations (`D_*`, `IX_*`).
//! - No domain-specific types or behavior.

use std::marker::PhantomData;

/// Namespace marker for a family of keys.
///
/// `Code` is a globally unique type-level natural number. Domain codes are defined in
/// `registry::codes`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Domain<Code>(PhantomData<fn() -> Code>);

/// Identifier within a domain: `Key<Domain, RoleCode, Index>`.
///
/// The domain and role parameters prevent cross-domain/role collisions even when the same index type
/// is reused across domains or roles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(clippy::type_complexity)] // PhantomData with tuple is intentionally complex for type-level encoding
pub struct Key<D, R, N>(PhantomData<fn() -> (D, R, N)>);

/// Attach a key `TagKey` to a payload `Payload`.
///
/// `Tagged<TagKey, Payload>` is the canonical representation for reflected program nodes:
/// - `TagKey` identifies the operation/node at the type level.
/// - `Payload` stores the payload (parameters, children, environment).
///
/// This type is `#[repr(transparent)]` to guarantee that `Tagged<TagKey, Payload>` has the same
/// runtime layout as `Payload`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tagged<TagKey, Payload>(pub Payload, PhantomData<fn() -> TagKey>);

impl<TagKey, Payload> Tagged<TagKey, Payload> {
    /// Construct a tagged payload.
    #[inline]
    pub const fn new(payload: Payload) -> Self {
        Self(payload, PhantomData)
    }

    /// Extract the payload.
    #[inline]
    pub fn into_inner(self) -> Payload {
        self.0
    }
}

impl<TagKey, Payload> Default for Tagged<TagKey, Payload>
where
    Payload: Default,
{
    #[inline]
    fn default() -> Self {
        Self::new(Payload::default())
    }
}
