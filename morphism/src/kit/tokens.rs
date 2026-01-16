//! Syntax tokens: payload markers for tagged nullary terms.
//!
//! These are **representation utilities** used by reflected program/value nodes
//! across `kit` (program construction) and `base` (domains).
//! They are intentionally tiny:
//! - `NullaryToken`: a zero-sized payload for nullary tagged syntax.
//! - `UnitToken`: a reflectable unit value (plain ZST).

/// Zero-sized payload used for nullary tagged syntax values.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NullaryToken;

/// Reflectable unit value (schema token).
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnitToken;
