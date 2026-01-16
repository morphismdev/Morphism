use crate::{Tagged, IX_OP_LIFT};

/// Operation lift (reflected): opaque operation bridge (lifts user `OpOnce` into AST).
///
/// This is the bridge from "semantic world" (user provides `OpOnce`) to
/// "syntax world" (AST term that can be lowered/reified).
///
/// Users construct this via `op_lift(op)`. Surface crates provide ergonomic wrappers
/// that lift runtime ops into syntax terms.
pub type OpLift<Op> = Tagged<IX_OP_LIFT, Op>;

/// Construct an opaque operation lift (lifts a user `OpOnce` into AST).
///
/// This is the public API for lifting user operations into the syntax world.
/// Surface crates provide ergonomic wrappers that use this internally.
#[inline]
pub const fn op_lift<Op>(op: Op) -> OpLift<Op> {
    Tagged::new(op)
}
