//! GOOD: real lowering (rewrites into smaller/kernel pieces)
//!
//! These files contain actual compiler work (rules/rewrites):
//! - generic.rs: big desugaring; mostly the right direction
//!
//! Note: hlist.rs has been removed. HList lowering is now handled by kernel/hlist.rs
//! (only kernel keys remain, which are mostly identity constructors).

pub mod generic;
