//! Core combinator infrastructure: type-level and term-level morphisms.

pub mod id;
pub use id::*;

pub mod then;
pub use then::*;

mod partial;
pub use partial::*;

pub mod const_op;
pub use const_op::*;

mod fanout;
pub use fanout::*;

pub mod bimap;
pub use bimap::*;

mod apply;
pub use apply::*;

pub mod projections;
pub use projections::*;

mod tagged;
pub use tagged::*;
