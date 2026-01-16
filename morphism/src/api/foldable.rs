//! Typeclass-style frontend for folding over the *children* of a generic node.
//!
//! Eager (consuming) API:
//! - `x.fold_children_l(op, acc)`

use crate::IntoContainer;
use crate::{Compile, OpOnce, Tagged, Then};
use crate::{FoldChildrenLProg, Id, OpLift};

type FoldChildrenLPipe<Op, Acc> = Then<Id, FoldChildrenLProg<OpLift<Op>, Acc>>;

pub trait Foldable: IntoContainer + Sized {
    #[inline]
    fn fold_children_l<Op, Acc, Out>(self, op: Op, acc: Acc) -> Out
    where
        Compile<Tagged<Self::Tag, Self::Payload>>: OpOnce<FoldChildrenLPipe<Op, Acc>>,
        <Compile<Tagged<Self::Tag, Self::Payload>> as OpOnce<FoldChildrenLPipe<Op, Acc>>>::OutVal:
            OpOnce<Tagged<Self::Tag, Self::Payload>, OutVal = Out>,
    {
        self.into_container().fold_children_l(op, acc).run()
    }
}

impl<T> Foldable for T where T: IntoContainer {}
