mod from_container;
pub use from_container::*;

mod into_container;
pub use into_container::*;

mod std_tuple_bridge;

use crate::Compile;
use crate::{
    fold_children_l_prog, id, map_children_prog, op_lift, FoldChildrenLProg, MapChildrenProg,
    OpLift,
};
use crate::{then, Id, OpOnce, Tagged, Then};
use core::marker::PhantomData;

/// Lazy container builder:
/// stores an input `Tagged<Tag, Payload>` plus a program `Prog`,
/// and remembers the originating host type `T` for type inference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Container<T, Prog = Id>
where
    T: IntoContainer,
{
    input: Tagged<T::Tag, T::Payload>,
    prog: Prog,
    _t: PhantomData<fn() -> T>,
}

impl<T> Container<T, Id>
where
    T: IntoContainer,
{
    /// Start a container builder from a concrete value (via `IntoContainer`).
    #[inline]
    pub fn new(value: T) -> Self {
        value.into_container()
    }

    /// Start a container builder from an existing container representation.
    ///
    /// This is useful when you already have `Tagged<Tag, Payload>` (e.g. from
    /// calling `IntoContainer::into_container()` directly).
    #[inline]
    pub fn from_tagged(input: Tagged<T::Tag, T::Payload>) -> Self {
        Self {
            input,
            prog: id(),
            _t: PhantomData,
        }
    }
}

impl<T, Prog> Container<T, Prog>
where
    T: IntoContainer,
{
    /// Append a program stage (pipeline composition).
    #[inline]
    pub fn then<Next>(self, next: Next) -> Container<T, Then<Prog, Next>> {
        Container {
            input: self.input,
            prog: then(self.prog, next),
            _t: PhantomData,
        }
    }

    #[inline]
    pub fn run<Out>(self) -> Out
    where
        Compile<Tagged<T::Tag, T::Payload>>: OpOnce<Prog>,
        <Compile<Tagged<T::Tag, T::Payload>> as OpOnce<Prog>>::OutVal:
            OpOnce<Tagged<T::Tag, T::Payload>, OutVal = Out>,
    {
        let compiled = Compile::<Tagged<T::Tag, T::Payload>>::new().run(self.prog);
        compiled.run(self.input)
    }

    /// Compile + run, then reconstruct the originating type `T` using `FromContainer`.
    /// This is the default collect path - no type annotations needed!
    /// The output payload type is inferred from the program.
    #[inline]
    pub fn collect<OutP>(self) -> T::Rewrap<OutP>
    where
        T: FromContainer,
        Compile<Tagged<T::Tag, T::Payload>>: OpOnce<Prog>,
        <Compile<Tagged<T::Tag, T::Payload>> as OpOnce<Prog>>::OutVal:
            OpOnce<Tagged<T::Tag, T::Payload>, OutVal = Tagged<T::Tag, OutP>>,
    {
        let out: Tagged<T::Tag, OutP> = self.run();
        T::from_container(out.into_inner())
    }

    /// Compile + run, then reconstruct `U` using `FromContainer`.
    /// Use this when you need explicit control over the output type.
    #[inline]
    pub fn collect_into<U>(self) -> U
    where
        U: IntoContainer<Tag = T::Tag>,
        U: FromContainer<Rewrap<<U as IntoContainer>::Payload> = U>,
        Compile<Tagged<T::Tag, T::Payload>>: OpOnce<Prog>,
        <Compile<Tagged<T::Tag, T::Payload>> as OpOnce<Prog>>::OutVal: OpOnce<
            Tagged<T::Tag, T::Payload>,
            OutVal = Tagged<T::Tag, <U as IntoContainer>::Payload>,
        >,
    {
        let out: Tagged<T::Tag, <U as IntoContainer>::Payload> = self.run();
        U::from_container(out.into_inner())
    }

    /// Compile + run, apply a payload transformation `M`, then reconstruct `U`.
    /// i.e: explicit backward arrow of the container.
    #[inline]
    pub fn reinterpret_into<U, M, OutP>(self, m: M) -> U
    where
        U: IntoContainer<Tag = T::Tag>,
        U: FromContainer<Rewrap<<U as IntoContainer>::Payload> = U>,
        Compile<Tagged<T::Tag, T::Payload>>: OpOnce<Prog>,
        <Compile<Tagged<T::Tag, T::Payload>> as OpOnce<Prog>>::OutVal:
            OpOnce<Tagged<T::Tag, T::Payload>, OutVal = Tagged<T::Tag, OutP>>,
        M: OpOnce<Tagged<T::Tag, OutP>, OutVal = Tagged<T::Tag, <U as IntoContainer>::Payload>>,
    {
        let compiled = Compile::<Tagged<T::Tag, T::Payload>>::new().run(self.prog);
        let tagged_out = compiled.run(self.input);
        let transformed = m.run(tagged_out);
        U::from_container(transformed.into_inner())
    }

    /// map over the generic node's children.
    #[inline]
    pub fn map<Op>(self, op: Op) -> Container<T, Then<Prog, MapChildrenProg<OpLift<Op>>>> {
        self.then(map_children_prog(op_lift(op)))
    }

    /// Fold over the generic node's children (left-associative).
    #[inline]
    pub fn fold_children_l<Op, Acc>(
        self,
        op: Op,
        acc: Acc,
    ) -> Container<T, Then<Prog, FoldChildrenLProg<OpLift<Op>, Acc>>> {
        self.then(fold_children_l_prog(op_lift(op), acc))
    }
}
