use crate::{Domain, Key, D_HLIST_FOLD, R_SEMANTICS, U0, U1, U2};

pub type HlistFoldDomain = Domain<D_HLIST_FOLD>;

// ****************************************************************************
// Semantics keys (R_SEMANTICS): HList fold pipeline primitives
// ****************************************************************************
//
// These are kernel primitives that enable expressing HList operations
// (like fill) as pure AST fold pipelines, without requiring OP_SYNTAX.
//
// Current kernel keys (dense U0..U2):
// - U0: TO_SEGMENTS_L (payload: StepProg) -> runtime to_segments transformer
// - U1: COMPOSE_BALANCED_L (nullary) -> runtime balanced composer
// - U2: MAP (payload: StepProg) -> runtime bridge key for HlistMap runtime op

// ─────────────────────────────────────────────────────────────────────────────
// Fold pipeline primitives
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_HLIST_TO_SEGMENTS_L = Key<HlistFoldDomain, R_SEMANTICS, U0>;

#[allow(non_camel_case_types)]
pub type IX_HLIST_COMPOSE_BALANCED_L = Key<HlistFoldDomain, R_SEMANTICS, U1>;

// ─────────────────────────────────────────────────────────────────────────────
// Runtime bridge keys (kernel keys that reify to runtime ops via EvalProg)
// ─────────────────────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
pub type IX_HLIST_MAP = Key<HlistFoldDomain, R_SEMANTICS, U2>;
