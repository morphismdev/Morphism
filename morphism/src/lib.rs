//! # `morphism`
//!
//! A **showcase snapshot** for a Rust generic-programming DSL built around two layers:
//! - **Semantics**: runnable operations (`OpTy`, `OpOnce`).
//! - **Syntax**: reflected program nodes (`Tagged<K, Payload>`) that can be compiled.
//!
//! It includes a deliberately **closed-world compiler** (bounded dispatch tables) to keep the demo small and predictable.
//!
//! ## Quickstart
//!
//! Run the examples:
//! - `cargo run --example mappable_simple`
//! - `cargo run --example mappable_multi`
//! - `cargo run --example foldable`
//!
//! ## Where to look
//! - `kit/`: kernel traits, combinators, and the `registry` (keys / identity).
//! - `base/`: primitive domains (bool/num/order/arithmetic).
//! - `container/`: Rust ↔ DSL bridge (`Container`, tuple bridge).
//! - `effects/`: higher-level AST constructors (frontend syntax terms).
//! - `compiler/`: closed lowering + reify pipeline.
//! - `hlist/`: tuple-based "HList" product plumbing (map/fold/get-at).
//! - `new_type/`: `NewTypeNode` (opaque/atomic boundary).
//!
//! Status: pre-release; APIs will change quickly.

pub mod kit;
pub use kit::*;

pub mod base;
pub use base::*;

mod type_eq;
pub use type_eq::*;

mod compiler;
pub use compiler::*;

pub mod hlist;
pub use hlist::*;

mod api;
pub use api::*;

mod container;
pub use container::*;

mod effects;
pub use effects::*;

mod new_type;
pub use new_type::*;

pub trait Generic {
    type Shape; // constructor tag (Key<Domain, Nat>)
    type Children; // immediate children representation
}

impl<K, Children> Generic for Tagged<K, Children> {
    type Shape = K;
    type Children = Children;
}
