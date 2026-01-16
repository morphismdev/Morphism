use crate::{Domain, Key, D_BOOL, R_SYNTAX, U0, U1};

pub type BoolDomain = Domain<D_BOOL>;

// ****************************************************************************
// Syntax keys (R_SYNTAX): boolean term constructors
// ****************************************************************************

// ─────────────────────────────────────────────────────────────────────────────
// Boolean value constructors
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_TRUE = Key<BoolDomain, R_SYNTAX, U0>;
#[allow(non_camel_case_types)]
pub type IX_FALSE = Key<BoolDomain, R_SYNTAX, U1>;
