//! Comparison operations for type-level numbers.
//!
//! Public ops: equality/relational/min-max.
//! Internal nat comparison machinery lives in `cmp.rs` (not re-exported).

mod equality;
pub use equality::*;

pub mod relational;
pub use relational::*;

mod cmp;
