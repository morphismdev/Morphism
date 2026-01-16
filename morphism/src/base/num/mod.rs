//! Foundational numeric layer: representations, aliases, and predicates.
//!
//! This module provides:
//! - **Natural number representation** (`nat.rs`): binary LSB-first encoding (`B0`, `B1`, `UTerm`, `UInt`) and normalization
//! - **Common naturals aliases** (`aliases.rs`): `U0..U64` convenience aliases
//! - **Predicates** (`predicates.rs`): small numeric predicates (odd/even/zero)
//!
//! Arithmetic operations live in `base::arithmetic`.
//!
//! See `base/num/README.md` for the full module contract.

pub mod nat;
pub use nat::*;

mod aliases;
pub use aliases::*;

pub mod predicates;
pub use predicates::*;
