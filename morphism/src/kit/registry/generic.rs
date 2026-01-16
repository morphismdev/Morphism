use crate::{Domain, Key, D_GENERIC, R_SEMANTICS, U0, U1, U2, U3, U4, U5};

pub type GenericDomain = Domain<D_GENERIC>;

// ****************************************************************************
// Semantics keys (R_SEMANTICS): generic operations
// ****************************************************************************

// ─────────────────────────────────────────────────────────────────────────────
// Map operations
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_MAP_NEWTYPENODE = Key<GenericDomain, R_SEMANTICS, U0>;
#[allow(non_camel_case_types)]
pub type IX_MAP_HLISTNODE = Key<GenericDomain, R_SEMANTICS, U1>;
#[allow(non_camel_case_types)]
pub type IX_MAP_CHILDREN = Key<GenericDomain, R_SEMANTICS, U2>;

// ─────────────────────────────────────────────────────────────────────────────
// Fold operations (left)
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_FOLD_NEWTYPENODE_L = Key<GenericDomain, R_SEMANTICS, U3>;
#[allow(non_camel_case_types)]
pub type IX_FOLD_CHILDREN_L = Key<GenericDomain, R_SEMANTICS, U4>;
#[allow(non_camel_case_types)]
pub type IX_FOLD_HLISTNODE_L = Key<GenericDomain, R_SEMANTICS, U5>;
