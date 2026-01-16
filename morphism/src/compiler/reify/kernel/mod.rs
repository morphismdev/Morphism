//! MUST KEEP: actual runtime primitives (target "kit only")
//!
//! These are the actual runtime primitives that should remain after lowering:
//! - combinators.rs: kernel combinators
//! - tagged.rs: kernel tagged operations

pub mod combinators;
pub mod op;
pub mod hlist;
pub mod hlist_fold;
pub mod tagged;