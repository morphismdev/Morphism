//! HlistNode wrapper-aware operations (In/Into variants).
//!
//! These programs operate on `HlistNode<Children>` wrappers:
//! - **In**: unwrap → transform → wrap (preserves wrapper)
//! - **Into**: unwrap → transform (eliminates wrapper, returns raw result)

mod map_in;
pub use map_in::*;

mod fold_into_l;
pub use fold_into_l::*;
