//! Introspection operations over tagging primitives.
//!
//! This module defines semantic operations (ops) that operate on `tag::{Key, Domain, Tagged}`.
//! These operations are not part of the tagging primitives themselves and must remain separate from `tag`.
//!
//! ## Contents
//!
//! - Projections:
//!   - `KeyOf`: `Tagged<TagKey, Payload> -> TagKey`
//!   - `PayloadOf`: `Tagged<TagKey, Payload> -> Payload`
//!
//! ## Scope
//!
//! This module is limited to **pure projection operations** that reason about keys and tags.
//! It does not define pattern matching, predicates, or general case analysis.

use crate::{OpOnce, OpTy, Tagged};

/// Extract the key from a tagged value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyOf;

impl<TagKey, Payload> OpTy<Tagged<TagKey, Payload>> for KeyOf {
    type OutTy = TagKey;
}

/// Value-level key extraction.
///
/// Keys are zero-sized, so this returns `TagKey::default()`.
impl<TagKey, Payload> OpOnce<Tagged<TagKey, Payload>> for KeyOf
where
    TagKey: Default,
{
    type OutVal = TagKey;

    #[inline]
    fn run(self, _input: Tagged<TagKey, Payload>) -> Self::OutVal {
        TagKey::default()
    }
}

/// Extract the payload from a tagged value: `Tagged<TagKey, Payload> -> Payload`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PayloadOf;

impl<TagKey, Payload> OpTy<Tagged<TagKey, Payload>> for PayloadOf {
    type OutTy = Payload;
}

impl<TagKey, Payload> OpOnce<Tagged<TagKey, Payload>> for PayloadOf {
    type OutVal = Payload;

    #[inline]
    fn run(self, input: Tagged<TagKey, Payload>) -> Self::OutVal {
        input.into_inner()
    }
}
