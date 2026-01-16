use crate::{Domain, Key, D_OP, R_SEMANTICS, U0};

pub type OpDomain = Domain<D_OP>;

// ****************************************************************************
// Semantics keys (R_SEMANTICS): operation lift bridge
// ****************************************************************************

// ─────────────────────────────────────────────────────────────────────────────
// Operation lift constructors
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_OP_LIFT = Key<OpDomain, R_SEMANTICS, U0>;
