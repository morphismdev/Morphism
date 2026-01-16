//! Closed-table lowering: Combinators domain (full table `U0..U9`).
//!
//! Routes recursion through `LowerTable` for closed-world lowering.

use crate::compiler::lower::LowerTable;
use crate::compiler::table::LowerByTable;
use crate::{Domain, Key, OpOnce, Tagged};
use crate::{D_COMBINATORS, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: LowerTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// Combinators-domain lowering: partially recursive (THEN and friends) via a table.
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_COMBINATORS>, R_SEMANTICS, Ix>, Payload>>
    for LowerTable<Input>
where
    LowerByTable<Input, LowerTableCombinators<Input>>:
        OpOnce<Tagged<Key<Domain<D_COMBINATORS>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <LowerByTable<Input, LowerTableCombinators<Input>> as OpOnce<
        Tagged<Key<Domain<D_COMBINATORS>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(
        self,
        prog: Tagged<Key<Domain<D_COMBINATORS>, R_SEMANTICS, Ix>, Payload>,
    ) -> Self::OutVal {
        LowerByTable::<Input, _>::new(lower_table_combinators::<Input>()).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition and handlers
// ─────────────────────────────────────────────────────────────────────────────

use core::marker::PhantomData;

use crate::{
    apply, bimap, const_move, fanout, fst, id, partial_l, partial_r, snd, then, ApplyProg, Bimap,
    ConstMove, Fanout, Fst, Id, NullaryToken, PartialL, PartialR, Snd, Then,
};

/// Lowering table for CombinatorsDomain (indices `U0..U9`).
///
/// Dense encoding: only kernel keys that survive lowering.
pub type LowerTableCombinators<Input> = (
    LowerId,              // U0: IX_ID
    LowerThen<Input>,     // U1: IX_THEN
    LowerPartialL<Input>, // U2: IX_PARTIAL_L
    LowerPartialR<Input>, // U3: IX_PARTIAL_R
    LowerFst,             // U4: IX_FST
    LowerSnd,             // U5: IX_SND
    LowerFanout<Input>,   // U6: IX_FANOUT
    LowerConstMove,       // U7: IX_CONST_MOVE
    LowerBimap<Input>,    // U8: IX_BIMAP
    LowerApply,           // U9: IX_APPLY
);

#[inline]
pub const fn lower_table_combinators<Input>() -> LowerTableCombinators<Input> {
    (
        LowerId,
        LowerThen::new(),
        LowerPartialL::new(),
        LowerPartialR::new(),
        LowerFst,
        LowerSnd,
        LowerFanout::new(),
        LowerConstMove,
        LowerBimap::new(),
        LowerApply,
    )
}

// U0/U4/U5/U7/U9: nullary identity (wrap payload back into same key)

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerId;
impl OpOnce<NullaryToken> for LowerId {
    type OutVal = Id;
    fn run(self, _: NullaryToken) -> Self::OutVal {
        id()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerFst;
impl OpOnce<NullaryToken> for LowerFst {
    type OutVal = Fst;
    fn run(self, _: NullaryToken) -> Self::OutVal {
        fst()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerSnd;
impl OpOnce<NullaryToken> for LowerSnd {
    type OutVal = Snd;
    fn run(self, _: NullaryToken) -> Self::OutVal {
        snd()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerConstMove;
impl<F> OpOnce<F> for LowerConstMove {
    type OutVal = ConstMove<F>;
    fn run(self, f: F) -> Self::OutVal {
        const_move(f)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerApply;
impl OpOnce<NullaryToken> for LowerApply {
    type OutVal = ApplyProg;
    fn run(self, _: NullaryToken) -> Self::OutVal {
        apply()
    }
}

// Recursive lowering: THEN/FANOUT/BIMAP/PARTIAL*

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerThen<Input>(PhantomData<fn() -> Input>);
impl<Input> LowerThen<Input> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, F, G> OpOnce<(F, G)> for LowerThen<Input>
where
    LowerTable<Input>: OpOnce<F>,
    LowerTable<Input>: OpOnce<G>,
{
    type OutVal =
        Then<<LowerTable<Input> as OpOnce<F>>::OutVal, <LowerTable<Input> as OpOnce<G>>::OutVal>;

    fn run(self, (f, g): (F, G)) -> Self::OutVal {
        then(
            LowerTable::<Input>::new().run(f),
            LowerTable::<Input>::new().run(g),
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerFanout<Input>(PhantomData<fn() -> Input>);
impl<Input> LowerFanout<Input> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, F, G> OpOnce<(F, G)> for LowerFanout<Input>
where
    crate::compiler::lower::LowerTable<Input>: OpOnce<F>,
    crate::compiler::lower::LowerTable<Input>: OpOnce<G>,
{
    type OutVal = Fanout<
        <crate::compiler::lower::LowerTable<Input> as OpOnce<F>>::OutVal,
        <crate::compiler::lower::LowerTable<Input> as OpOnce<G>>::OutVal,
    >;

    fn run(self, (f, g): (F, G)) -> Self::OutVal {
        fanout(
            crate::compiler::lower::LowerTable::<Input>::new().run(f),
            crate::compiler::lower::LowerTable::<Input>::new().run(g),
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerBimap<Input>(PhantomData<fn() -> Input>);
impl<Input> LowerBimap<Input> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, F, G> OpOnce<(F, G)> for LowerBimap<Input>
where
    crate::compiler::lower::LowerTable<Input>: OpOnce<F>,
    crate::compiler::lower::LowerTable<Input>: OpOnce<G>,
{
    type OutVal = Bimap<
        <crate::compiler::lower::LowerTable<Input> as OpOnce<F>>::OutVal,
        <crate::compiler::lower::LowerTable<Input> as OpOnce<G>>::OutVal,
    >;

    fn run(self, (f, g): (F, G)) -> Self::OutVal {
        bimap(
            crate::compiler::lower::LowerTable::<Input>::new().run(f),
            crate::compiler::lower::LowerTable::<Input>::new().run(g),
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerPartialL<Input>(PhantomData<fn() -> Input>);
impl<Input> LowerPartialL<Input> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, Env, Op> OpOnce<(Env, Op)> for LowerPartialL<Input>
where
    crate::compiler::lower::LowerTable<Input>: OpOnce<Op>,
{
    type OutVal = PartialL<Env, <crate::compiler::lower::LowerTable<Input> as OpOnce<Op>>::OutVal>;

    fn run(self, (env, op): (Env, Op)) -> Self::OutVal {
        partial_l(
            env,
            crate::compiler::lower::LowerTable::<Input>::new().run(op),
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerPartialR<Input>(PhantomData<fn() -> Input>);
impl<Input> LowerPartialR<Input> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, Op, Env> OpOnce<(Op, Env)> for LowerPartialR<Input>
where
    crate::compiler::lower::LowerTable<Input>: OpOnce<Op>,
{
    type OutVal = PartialR<<crate::compiler::lower::LowerTable<Input> as OpOnce<Op>>::OutVal, Env>;

    fn run(self, (op, env): (Op, Env)) -> Self::OutVal {
        partial_r(
            crate::compiler::lower::LowerTable::<Input>::new().run(op),
            env,
        )
    }
}
