use crate::{Domain, Key, D_TAGGED, R_SEMANTICS, U0, U1};

pub type TaggedDomain = Domain<D_TAGGED>;

// ****************************************************************************
// Semantics keys (R_SEMANTICS): tagged type operations
// ****************************************************************************

// ─────────────────────────────────────────────────────────────────────────────
// Wrap and unwrap operations
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_WRAP_TAGGED = Key<TaggedDomain, R_SEMANTICS, U0>;
#[allow(non_camel_case_types)]
pub type IX_UNWRAP_TAGGED = Key<TaggedDomain, R_SEMANTICS, U1>;
