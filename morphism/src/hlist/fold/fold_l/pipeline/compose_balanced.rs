//! Balanced composition (fold-left order).

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

define_compose_balanced!(pair_ty, pair_val);

#[cfg(test)]
mod tuple_compose_balanced_tests {
    use super::*;
    use crate::{OpOnce, OpTy};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ToU64;
    impl OpTy<u32> for ToU64 {
        type OutTy = u64;
    }
    impl OpOnce<u32> for ToU64 {
        type OutVal = u64;
        fn run(self, x: u32) -> u64 {
            x as u64
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Add1U64;
    impl OpTy<u64> for Add1U64 {
        type OutTy = u64;
    }
    impl OpOnce<u64> for Add1U64 {
        type OutVal = u64;
        fn run(self, x: u64) -> u64 {
            x + 1
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ToU128;
    impl OpTy<u64> for ToU128 {
        type OutTy = u128;
    }
    impl OpOnce<u64> for ToU128 {
        type OutVal = u128;
        fn run(self, x: u64) -> u128 {
            x as u128
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Add1U128;
    impl OpTy<u128> for Add1U128 {
        type OutTy = u128;
    }
    impl OpOnce<u128> for Add1U128 {
        type OutVal = u128;
        fn run(self, x: u128) -> u128 {
            x + 1
        }
    }

    #[test]
    fn balanced_arity_4_matches_sequential() {
        let t_bal = TupleComposeBalanced.run((ToU64, Add1U64, ToU128, Add1U128));
        let t_seq = crate::ThenOp::new(
            ToU64,
            crate::ThenOp::new(Add1U64, crate::ThenOp::new(ToU128, Add1U128)),
        );

        assert_eq!(t_bal.run(41u32), t_seq.run(41u32));
        assert_eq!(t_bal.run(41u32), 43u128);
    }
}

#[cfg(test)]
mod hlist_compose_balanced_tests {
    use super::*;

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
    fn balanced_hlist_23_add1() {
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

        let t = HlistComposeBalanced.run(input);
        assert_eq!(t.run(0u32), 23u32);
    }
}
