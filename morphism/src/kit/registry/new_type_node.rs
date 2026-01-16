use crate::{Domain, Key, D_NEW_TYPE, R_SYNTAX, U0};

pub type NewTypeDomain = Domain<D_NEW_TYPE>;

// ****************************************************************************
// Syntax keys (R_SYNTAX): newtype node constructors
// ****************************************************************************

// ─────────────────────────────────────────────────────────────────────────────
// Node constructors
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_NEWTYPENODE = Key<NewTypeDomain, R_SYNTAX, U0>;
