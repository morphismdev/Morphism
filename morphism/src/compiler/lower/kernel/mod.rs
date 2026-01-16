//! Closed-table lowering: kernel/base domains.
//!
//! This module contains the lowering tables for domains that we consider part of the
//! post-lowering kernel surface (i.e. they are expected to survive lowering and reach reify).
//!
//! Notes:
//! - Some entries are true lowering (e.g. combinators recursively lower children via `LowerTable`).
//! - Some entries are intentionally "identity-shaped" constructors (e.g. tagged/op-syntax),
//!   but we keep them here to make closed-table wiring explicit and uniform.

pub mod combinators;
pub mod tagged;
pub mod op;
pub mod hlist;
pub mod hlist_fold;
