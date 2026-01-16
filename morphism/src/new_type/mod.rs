mod map_newtypenode;
pub use map_newtypenode::*;

mod new_type_fold;
pub use new_type_fold::*;

use crate::{Tagged, IX_NEWTYPENODE};

/// NewType node: represents an opaque/atomic value in the generic representation
pub type NewTypeNode<S> = Tagged<IX_NEWTYPENODE, S>;
