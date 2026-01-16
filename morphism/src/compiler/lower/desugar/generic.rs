//! Closed-table lowering: Generic domain (FP structural-generic).
//!
//! `D_GENERIC` is the structural-generic frontend domain for Rust ADT structure:
//! - `NewTypeNode<S>`: newtype wrapper
//! - `HlistNode<Children>`: heterogeneous list (product)
//!
//! This module wires closed lowering for the full current `D_GENERIC` key set.
//! All keys must lower away completely (no `D_GENERIC` keys reach reify).
//!
//! Current closed lowering coverage in this module:
//! - `IX_MAP_NEWTYPENODE` (U0)
//! - `IX_MAP_HLISTNODE` (U1)
//! - `IX_MAP_CHILDREN` (U2) (dispatches by input shape: NewTypeNode/HlistNode)
//! - `IX_FOLD_NEWTYPENODE_L` (U3)
//! - `IX_FOLD_CHILDREN_L` (U4) (dispatches by input shape)
//! - `IX_FOLD_HLISTNODE_L` (U5) (restricted to `OpLift<Op>` step programs)
//!
//! Note: Some implementations are intentionally restricted (e.g. HList fold steps must be `OpLift<Op>`,
//! dispatchers only apply to the two structural node shapes) to keep compile times stable.

use core::marker::PhantomData;

use crate::compiler::lower::LowerTable;
use crate::compiler::table::LowerByTable;
use crate::{
    apply, const_move, fold_newtypenode_l_prog, hlist_compose_balanced_l_prog, hlist_map_prog,
    hlist_to_segments_l_prog, map_in_hlist_node_prog, map_newtypenode_prog, partial_r, then,
    unwrap_tagged, wrap_tagged, ConstMove, FoldNewTypeNodeLProg, HlistMapProg, HlistNode,
    MapInHlistNodeProg, MapNewTypeNodeProg, NewTypeNode, OpLift, OpOnce, PartialR, Then,
    UnwrapTagged, WrapTagged, IX_HLISTNODE, IX_NEWTYPENODE,
};
use crate::{Domain, Key, Tagged, D_GENERIC, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: LowerTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_GENERIC>, R_SEMANTICS, Ix>, Payload>>
    for LowerTable<Input>
where
    LowerByTable<Input, LowerTableGeneric<Input>>:
        OpOnce<Tagged<Key<Domain<D_GENERIC>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <LowerByTable<Input, LowerTableGeneric<Input>> as OpOnce<
        Tagged<Key<Domain<D_GENERIC>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(self, prog: Tagged<Key<Domain<D_GENERIC>, R_SEMANTICS, Ix>, Payload>) -> Self::OutVal {
        LowerByTable::<Input, _>::new(lower_table_generic::<Input>()).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition (U0..U5) + constructor
// ─────────────────────────────────────────────────────────────────────────────

/// Lowering table for GenericDomain semantics keys.
///
/// IMPORTANT: this uses a flat tuple encoding since all indices are U0..U5.
pub type LowerTableGeneric<Input> = (
    LowerMapNewTypeNode<Input>,   // U0: IX_MAP_NEWTYPENODE
    LowerMapHlistNode<Input>,     // U1: IX_MAP_HLISTNODE
    LowerMapChildren<Input>,      // U2: IX_MAP_CHILDREN
    LowerFoldNewTypeNodeL<Input>, // U3: IX_FOLD_NEWTYPENODE_L
    LowerFoldChildrenL<Input>,    // U4: IX_FOLD_CHILDREN_L
    LowerFoldHlistNodeL<Input>,   // U5: IX_FOLD_HLISTNODE_L
);

#[inline]
pub const fn lower_table_generic<Input>() -> LowerTableGeneric<Input> {
    (
        LowerMapNewTypeNode::new(),
        LowerMapHlistNode::new(),
        LowerMapChildren::new(),
        LowerFoldNewTypeNodeL::new(),
        LowerFoldChildrenL::new(),
        LowerFoldHlistNodeL::new(),
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// U0: IX_MAP_NEWTYPENODE
// ─────────────────────────────────────────────────────────────────────────────

/// Lowering op for `IX_MAP_NEWTYPENODE` payload.
///
/// Lowered program:
/// - `then(then(unwrap_newtypenode, lowered_op), wrap_newtypenode)`
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerMapNewTypeNode<Input>(PhantomData<fn() -> Input>);

impl<Input> LowerMapNewTypeNode<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, Op> OpOnce<OpLift<Op>> for LowerMapNewTypeNode<Input>
where
    LowerTable<Input>: OpOnce<OpLift<Op>>,
{
    type OutVal = Then<
        Then<UnwrapTagged<IX_NEWTYPENODE>, <LowerTable<Input> as OpOnce<OpLift<Op>>>::OutVal>,
        WrapTagged<IX_NEWTYPENODE>,
    >;

    #[inline]
    fn run(self, op: OpLift<Op>) -> Self::OutVal {
        then(
            then(
                unwrap_tagged::<IX_NEWTYPENODE>(),
                LowerTable::<Input>::new().run(op),
            ),
            wrap_tagged::<IX_NEWTYPENODE>(),
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// U1: IX_MAP_HLISTNODE
// ─────────────────────────────────────────────────────────────────────────────

/// Lowering op for `IX_MAP_HLISTNODE` payload.
///
/// Lowered program:
/// - `then(then(unwrap_hlist, hlist_map_prog(op_lift)), wrap_hlist)`
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerMapHlistNode<Input>(PhantomData<fn() -> Input>);

impl<Input> LowerMapHlistNode<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, Op> OpOnce<OpLift<Op>> for LowerMapHlistNode<Input> {
    type OutVal =
        Then<Then<UnwrapTagged<IX_HLISTNODE>, HlistMapProg<OpLift<Op>>>, WrapTagged<IX_HLISTNODE>>;

    #[inline]
    fn run(self, op: OpLift<Op>) -> Self::OutVal {
        then(
            then(unwrap_tagged::<IX_HLISTNODE>(), hlist_map_prog(op)),
            wrap_tagged::<IX_HLISTNODE>(),
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// U2: IX_MAP_CHILDREN (dispatcher)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerMapChildren<Input>(PhantomData<fn() -> Input>);

impl<Input> LowerMapChildren<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

// NewTypeNode<S>
impl<S, Op> OpOnce<OpLift<Op>> for LowerMapChildren<NewTypeNode<S>>
where
    LowerTable<NewTypeNode<S>>: OpOnce<OpLift<Op>>,
    LowerTable<NewTypeNode<S>>:
        OpOnce<MapNewTypeNodeProg<<LowerTable<NewTypeNode<S>> as OpOnce<OpLift<Op>>>::OutVal>>,
{
    type OutVal = <LowerTable<NewTypeNode<S>> as OpOnce<
        MapNewTypeNodeProg<<LowerTable<NewTypeNode<S>> as OpOnce<OpLift<Op>>>::OutVal>,
    >>::OutVal;

    #[inline]
    fn run(self, op: OpLift<Op>) -> Self::OutVal {
        let lowered_op = LowerTable::<NewTypeNode<S>>::new().run(op);
        let specialized = map_newtypenode_prog(lowered_op);
        LowerTable::<NewTypeNode<S>>::new().run(specialized)
    }
}

// HlistNode<Children>
impl<Children, Op> OpOnce<OpLift<Op>> for LowerMapChildren<HlistNode<Children>>
where
    LowerTable<HlistNode<Children>>: OpOnce<OpLift<Op>>,
    LowerTable<HlistNode<Children>>:
        OpOnce<MapInHlistNodeProg<<LowerTable<HlistNode<Children>> as OpOnce<OpLift<Op>>>::OutVal>>,
{
    type OutVal = <LowerTable<HlistNode<Children>> as OpOnce<
        MapInHlistNodeProg<<LowerTable<HlistNode<Children>> as OpOnce<OpLift<Op>>>::OutVal>,
    >>::OutVal;

    #[inline]
    fn run(self, op: OpLift<Op>) -> Self::OutVal {
        let lowered_op = LowerTable::<HlistNode<Children>>::new().run(op);
        let specialized = map_in_hlist_node_prog(lowered_op);
        LowerTable::<HlistNode<Children>>::new().run(specialized)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Generic fold family (U3..U5)
// ─────────────────────────────────────────────────────────────────────────────

// U3: IX_FOLD_NEWTYPENODE_L
//
// NewTypeNode has no children, so folding returns the accumulator unchanged.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerFoldNewTypeNodeL<Input>(PhantomData<fn() -> Input>);

impl<Input> LowerFoldNewTypeNodeL<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, FProg, Acc> OpOnce<(FProg, Acc)> for LowerFoldNewTypeNodeL<Input> {
    type OutVal = ConstMove<Acc>;

    #[inline]
    fn run(self, (_f_prog, acc): (FProg, Acc)) -> Self::OutVal {
        const_move(acc)
    }
}

// U4: IX_FOLD_CHILDREN_L (dispatcher)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerFoldChildrenL<Input>(PhantomData<fn() -> Input>);

impl<Input> LowerFoldChildrenL<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

// NewTypeNode<S>: lower FoldNewTypeNodeLProg all the way
impl<S, FProg, Acc> OpOnce<(FProg, Acc)> for LowerFoldChildrenL<NewTypeNode<S>>
where
    LowerTable<NewTypeNode<S>>: OpOnce<FoldNewTypeNodeLProg<FProg, Acc>>,
{
    type OutVal = <LowerTable<NewTypeNode<S>> as OpOnce<FoldNewTypeNodeLProg<FProg, Acc>>>::OutVal;

    #[inline]
    fn run(self, (f_prog, acc): (FProg, Acc)) -> Self::OutVal {
        let specialized = fold_newtypenode_l_prog(f_prog, acc);
        LowerTable::<NewTypeNode<S>>::new().run(specialized)
    }
}

impl<Children, FProg, Acc> OpOnce<(FProg, Acc)> for LowerFoldChildrenL<HlistNode<Children>>
where
    LowerFoldHlistNodeL<HlistNode<Children>>: OpOnce<(FProg, Acc)>,
{
    type OutVal = <LowerFoldHlistNodeL<HlistNode<Children>> as OpOnce<(FProg, Acc)>>::OutVal;

    #[inline]
    fn run(self, (f_prog, acc): (FProg, Acc)) -> Self::OutVal {
        LowerFoldHlistNodeL::<HlistNode<Children>>::new().run((f_prog, acc))
    }
}

// NOTE: `IX_FOLD_HLISTNODE_L` lowering is implemented below (U5).
// It is intentionally restricted to step programs of shape `OpLift<Op>` to keep the trait solver stable.

// ------------------------------
// U5: IX_FOLD_HLISTNODE_L
// ------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerFoldHlistNodeL<Input>(PhantomData<fn() -> Input>);

impl<Input> LowerFoldHlistNodeL<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

// NOTE: we intentionally restrict the step program to `OpLift<Op>` here.
// We use AST program keys (hlist_to_segments_l_prog, hlist_compose_balanced_l_prog, apply)
// instead of runtime ops to keep the pipeline pure AST.
impl<Children, Op, Acc> OpOnce<(OpLift<Op>, Acc)> for LowerFoldHlistNodeL<HlistNode<Children>> {
    type OutVal = Then<
        UnwrapTagged<IX_HLISTNODE>,
        Then<
            crate::HlistToSegmentsLProg<OpLift<Op>>,
            Then<crate::HlistComposeBalancedLProg, PartialR<crate::ApplyProg, Acc>>,
        >,
    >;

    #[inline]
    fn run(self, (f_prog, acc): (OpLift<Op>, Acc)) -> Self::OutVal {
        let fold_pipeline = then(
            hlist_to_segments_l_prog(f_prog),
            then(hlist_compose_balanced_l_prog(), partial_r(apply(), acc)),
        );
        then(unwrap_tagged::<IX_HLISTNODE>(), fold_pipeline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Compile;
    use crate::{
        fold_children_l_prog, fold_newtypenode_l_prog, map_newtypenode_prog, op_lift, OpOnce,
        Tagged,
    };

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct Inc;

    impl OpOnce<i32> for Inc {
        type OutVal = i32;
        fn run(self, x: i32) -> Self::OutVal {
            x + 1
        }
    }

    #[test]
    fn compile_closed_v2_lowers_then_reifies_map_newtypenode() {
        type N = Tagged<IX_NEWTYPENODE, i32>;
        let prog = map_newtypenode_prog(op_lift(Inc));
        let out = Compile::<N>::new().run(prog).run(Tagged::new(41));
        assert_eq!(out, Tagged::new(42));
    }

    #[test]
    fn compile_closed_v2_lowers_then_reifies_fold_newtypenode_l_returns_acc() {
        type N = Tagged<IX_NEWTYPENODE, i32>;
        let prog = fold_newtypenode_l_prog(op_lift(Inc), 123_i32);
        let out = Compile::<N>::new().run(prog).run(Tagged::new(999));
        assert_eq!(out, 123_i32);
    }

    #[test]
    fn compile_closed_v2_lowers_then_reifies_fold_children_l_dispatches_on_newtypenode() {
        type N = Tagged<IX_NEWTYPENODE, i32>;
        let prog = fold_children_l_prog(op_lift(Inc), 5_i32);
        let out = Compile::<N>::new().run(prog).run(Tagged::new(999));
        assert_eq!(out, 5_i32);
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct CountL;

    impl<X> OpOnce<(usize, X)> for CountL {
        type OutVal = usize;
        #[inline]
        fn run(self, (acc, _x): (usize, X)) -> Self::OutVal {
            acc + 1
        }
    }

    #[test]
    fn compile_closed_v2_fold_children_l_works_for_hlistnode_arity_12() {
        // 12 elements encoded as: 11-head + (last,)
        type Children = (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, (u8,));
        type H = Tagged<IX_HLISTNODE, Children>;

        let input: H = Tagged::new((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, (12,)));
        let prog = fold_children_l_prog(op_lift(CountL), 0usize);
        let out = Compile::<H>::new().run(prog).run(input);
        assert_eq!(out, 12usize);
    }
}
