//! Pairwise composition (fold-left order).

// Parameterize the shared implementation with fold-left pairing:
// (a, b) -> ThenOp<a, b>
macro_rules! pair_ty {
    ($A:ident, $B:ident) => {
        crate::ThenOp<$A, $B>
    };
}
macro_rules! pair_val {
    ($a:ident, $b:ident) => {
        crate::ThenOp::new($a, $b)
    };
}

define_compose_pairs!(pair_ty, pair_val);

#[cfg(test)]
mod tuple_compose_pairs_tests {
    use super::*;
    use crate::{OpOnce, OpTy};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Add1;
    impl OpTy<u32> for Add1 {
        type OutTy = u32;
    }
    impl OpOnce<u32> for Add1 {
        type OutVal = u32;
        fn run(self, x: u32) -> Self::OutVal {
            x + 1
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ToU64;
    impl OpTy<u32> for ToU64 {
        type OutTy = u64;
    }
    impl OpOnce<u32> for ToU64 {
        type OutVal = u64;
        fn run(self, x: u32) -> Self::OutVal {
            x as u64
        }
    }

    #[test]
    fn pairs_arity_1_is_identity() {
        let out = TupleComposePairs.run((Add1,));
        assert_eq!(out.0.run(41u32), 42u32);
    }

    #[test]
    fn pairs_arity_2_composes_adjacent() {
        let (t01,) = TupleComposePairs.run((Add1, ToU64));
        assert_eq!(t01.run(41u32), 42u64);

        type OutTy = <TupleComposePairs as OpTy<(Add1, ToU64)>>::OutTy;
        let _x: OutTy = (crate::ThenOp::new(Add1, ToU64),);
    }

    #[test]
    fn pairs_arity_3_carries_last() {
        let (t01, t2) = TupleComposePairs.run((Add1, ToU64, Add1));
        assert_eq!(t01.run(41u32), 42u64);
        assert_eq!(t2.run(1u32), 2u32);
    }
}

#[cfg(test)]
mod hlist_compose_pairs_tests {
    use super::*;
    use crate::OpOnce;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Add1;
    impl crate::OpTy<u32> for Add1 {
        type OutTy = u32;
    }
    impl crate::OpOnce<u32> for Add1 {
        type OutVal = u32;
        fn run(self, x: u32) -> u32 {
            x + 1
        }
    }

    #[test]
    fn hlist_pairs_len_12_produces_6() {
        // 12 elements encoded as: 11-head + (last,)
        let input = (
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            (Add1,),
        );

        let out = HlistComposePairs.run(input);
        let (t0, t1, t2, t3, t4, t5) = out;
        assert_eq!(t0.run(0), 2);
        assert_eq!(t1.run(0), 2);
        assert_eq!(t2.run(0), 2);
        assert_eq!(t3.run(0), 2);
        assert_eq!(t4.run(0), 2);
        assert_eq!(t5.run(0), 2);
    }

    #[test]
    fn hlist_pairs_len_23_produces_12_hlist() {
        // 23 elements encoded as: 11-head + (11-head + (last,))
        let input = (
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            Add1,
            (
                Add1,
                Add1,
                Add1,
                Add1,
                Add1,
                Add1,
                Add1,
                Add1,
                Add1,
                Add1,
                Add1,
                (Add1,),
            ),
        );

        let out = HlistComposePairs.run(input);
        let (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, tail) = out;

        assert_eq!(t0.run(0), 2);
        assert_eq!(t1.run(0), 2);
        assert_eq!(t2.run(0), 2);
        assert_eq!(t3.run(0), 2);
        assert_eq!(t4.run(0), 2);
        assert_eq!(t5.run(0), 2);
        assert_eq!(t6.run(0), 2);
        assert_eq!(t7.run(0), 2);
        assert_eq!(t8.run(0), 2);
        assert_eq!(t9.run(0), 2);
        assert_eq!(t10.run(0), 2);

        let (last,) = tail;
        assert_eq!(last.run(0), 1);
    }
}
