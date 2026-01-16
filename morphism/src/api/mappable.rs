use crate::{Compile, MapChildrenProg, OpLift};
use crate::{Container, FromContainer, IntoContainer};
use crate::{Id, OpOnce, Tagged, Then};

// ===========================================================================
// Mappable: Eager API on top of the GAT plan trait
// ===========================================================================

pub trait Mappable: IntoContainer + FromContainer + Sized {
    fn map<Op, OutP>(self, op: Op) -> <Self as FromContainer>::Rewrap<OutP>
    where
        Compile<Tagged<Self::Tag, Self::Payload>>: OpOnce<Then<Id, MapChildrenProg<OpLift<Op>>>>,
        <Compile<Tagged<Self::Tag, Self::Payload>> as OpOnce<
            Then<Id, MapChildrenProg<OpLift<Op>>>,
        >>::OutVal: OpOnce<Tagged<Self::Tag, Self::Payload>, OutVal = Tagged<Self::Tag, OutP>>,
    {
        self.into_container().map(op).collect::<OutP>()
    }
}

impl<T> Mappable for T where T: IntoContainer + FromContainer {}

/// A tiny example trait showing how to "argue the return type" *without*
/// naming/binding a specific `Op` in the trait itself.
///
/// This is essentially a GAT-shaped wrapper around `Container::new(..).map(op)`.
pub trait MappablePlan: IntoContainer + Sized {
    type Plan<Op>;
    fn map_plan<Op>(self, op: Op) -> Self::Plan<Op>;
}

impl<T> MappablePlan for T
where
    T: IntoContainer,
{
    type Plan<Op> = Container<T, Then<Id, MapChildrenProg<OpLift<Op>>>>;

    #[inline]
    fn map_plan<Op>(self, op: Op) -> Self::Plan<Op> {
        self.into_container().map(op)
    }
}
