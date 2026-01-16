//! Tagged combinators: wrap/unwrap/tag/retag operations on `Tagged<K, T>`.
//!
//! This module provides combinators for constructing and transforming tagged types.
//! These are DSL building blocks, not reflection operations.

use crate::kit::op::{OpOnce, OpTy};
use crate::registry::{IX_UNWRAP_TAGGED, IX_WRAP_TAGGED};
use crate::tag::Tagged;
use core::marker::PhantomData;

// ═══════════════════════════════════════════════════════════════════════════
// WRAP_TAGGED - Wrap a value into a Tagged type
// ═══════════════════════════════════════════════════════════════════════════

/// Program node: wrap `T` into `Tagged<K, T>` (payload only carries `K` at the type level).
pub type WrapTagged<K> = Tagged<IX_WRAP_TAGGED, PhantomData<fn() -> K>>;

#[inline]
pub const fn wrap_tagged<K>() -> WrapTagged<K> {
    Tagged::new(PhantomData)
}

/// Runnable op: wrap `T` into `Tagged<K, T>`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrapTaggedOp<K>(PhantomData<fn() -> K>);

impl<K> WrapTaggedOp<K> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<K, T> OpTy<T> for WrapTaggedOp<K> {
    type OutTy = Tagged<K, T>;
}

impl<K, T> OpOnce<T> for WrapTaggedOp<K> {
    type OutVal = Tagged<K, T>;

    #[inline]
    fn run(self, t: T) -> Self::OutVal {
        Tagged::new(t)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UNWRAP_TAGGED - Unwrap a Tagged type into its inner value
// ═══════════════════════════════════════════════════════════════════════════

/// Program node: unwrap `Tagged<K, T>` into `T` (payload only carries `K` at the type level).
pub type UnwrapTagged<K> = Tagged<IX_UNWRAP_TAGGED, PhantomData<fn() -> K>>;

#[inline]
pub const fn unwrap_tagged<K>() -> UnwrapTagged<K> {
    Tagged::new(PhantomData)
}

/// Runnable op: unwrap `Tagged<K, T>` into `T`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnwrapTaggedOp<K>(PhantomData<fn() -> K>);

impl<K> UnwrapTaggedOp<K> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<K, T> OpTy<Tagged<K, T>> for UnwrapTaggedOp<K> {
    type OutTy = T;
}

impl<K, T> OpOnce<Tagged<K, T>> for UnwrapTaggedOp<K> {
    type OutVal = T;

    #[inline]
    fn run(self, x: Tagged<K, T>) -> Self::OutVal {
        x.into_inner()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Tag/Retag operations
// ═══════════════════════════════════════════════════════════════════════════

/// Generic tag-with-key Lifter: T -> Tagged<K, T>
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TagWith<K>(PhantomData<fn() -> K>);

impl<K, T> OpTy<T> for TagWith<K> {
    type OutTy = Tagged<K, T>;
}

/// Generic retag: Tagged<K1, T> -> Tagged<K2, T>
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReTag<K2>(PhantomData<fn() -> K2>);

impl<K1, K2, T> OpTy<Tagged<K1, T>> for ReTag<K2> {
    type OutTy = Tagged<K2, T>;
}
