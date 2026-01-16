//! Boolean domain semantics: operations and elimination over boolean syntax.

pub mod ops;
pub use ops::*;

pub mod elim;
pub use elim::*;

use crate::kit::tokens::NullaryToken;
use crate::registry::bool::{IX_FALSE, IX_TRUE};
use crate::tag::Tagged;

// ─────────────────────────────────────────────────────────────────────────────
// Domain syntax (reflectable values)
// ─────────────────────────────────────────────────────────────────────────────

/// Type-level boolean true value.
pub type True = Tagged<IX_TRUE, NullaryToken>;

/// Type-level boolean false value.
pub type False = Tagged<IX_FALSE, NullaryToken>;
