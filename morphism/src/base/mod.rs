//! Primitive domains: boolean, numeric, order.
//!
//! See `base/README.md` for the module contract.

pub mod num;
pub use num::*;

pub mod order;
pub use order::*;

pub mod boolean;
pub use boolean::*;

mod arithmetic;
pub use arithmetic::*;
