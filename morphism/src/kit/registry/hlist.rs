use crate::{Domain, Key, D_HLIST, R_SEMANTICS, R_SYNTAX, U0};

pub type HlistDomain = Domain<D_HLIST>;

// ****************************************************************************
// Syntax keys (R_SYNTAX): HList structural nodes
// ****************************************************************************

// ─────────────────────────────────────────────────────────────────────────────
// Node constructors
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_HLISTNODE = Key<HlistDomain, R_SYNTAX, U0>;

// ****************************************************************************
// Semantics keys (R_SEMANTICS): HList operations
// ****************************************************************************
//
// Only keys that are actually wired in the compiler pipeline are kept.
// Purged keys (must lower away completely, no longer in registry):
// - FILL, MAP, ZIP_WITH, FOLD_R, FOLD_L_HETERO, POP_FRONT, POP_BACK, PUSH_FRONT
//
// Current kernel keys (dense U0..U0):
// - U0: PUSH_BACK

// ─────────────────────────────────────────────────────────────────────────────
// List manipulation
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_HLIST_PUSH_BACK = Key<HlistDomain, R_SEMANTICS, U0>;
