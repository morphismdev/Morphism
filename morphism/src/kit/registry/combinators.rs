use crate::{Domain, Key, D_COMBINATORS, R_SEMANTICS, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9};

pub type CombinatorsDomain = Domain<D_COMBINATORS>;

// ****************************************************************************
// Semantics keys (R_SEMANTICS): op combinators
// ****************************************************************************

// ─────────────────────────────────────────────────────────────────────────────
// Core combinators
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_ID = Key<CombinatorsDomain, R_SEMANTICS, U0>;
#[allow(non_camel_case_types)]
pub type IX_THEN = Key<CombinatorsDomain, R_SEMANTICS, U1>;

// ─────────────────────────────────────────────────────────────────────────────
// Partial application
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_PARTIAL_L = Key<CombinatorsDomain, R_SEMANTICS, U2>;
#[allow(non_camel_case_types)]
pub type IX_PARTIAL_R = Key<CombinatorsDomain, R_SEMANTICS, U3>;

// ─────────────────────────────────────────────────────────────────────────────
// Tuple operations
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_FST = Key<CombinatorsDomain, R_SEMANTICS, U4>;
#[allow(non_camel_case_types)]
pub type IX_SND = Key<CombinatorsDomain, R_SEMANTICS, U5>;
#[allow(non_camel_case_types)]
pub type IX_FANOUT = Key<CombinatorsDomain, R_SEMANTICS, U6>;

// ─────────────────────────────────────────────────────────────────────────────
// Constant / bimap / apply
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_CONST_MOVE = Key<CombinatorsDomain, R_SEMANTICS, U7>;
#[allow(non_camel_case_types)]
pub type IX_BIMAP = Key<CombinatorsDomain, R_SEMANTICS, U8>;
#[allow(non_camel_case_types)]
pub type IX_APPLY = Key<CombinatorsDomain, R_SEMANTICS, U9>;
