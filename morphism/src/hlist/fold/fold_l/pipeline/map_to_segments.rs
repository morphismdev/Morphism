//! Map elements to segment transformers (fold-left).

define_map_to_segments!(SegmentFoldL);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HlistComposeBalanced;
    use crate::{OpOnce, OpTy};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Count;

    impl<X> OpTy<(usize, X)> for Count {
        type OutTy = usize;
    }
    impl<X> OpOnce<(usize, X)> for Count {
        type OutVal = usize;
        #[inline]
        fn run(self, (acc, _x): (usize, X)) -> usize {
            acc + 1
        }
    }

    #[test]
    fn map_elements_to_transformers_then_compose_counts_len_23() {
        // 23 elements encoded as: 11-head + (11-head + (last,))
        let elems = (
            1u8,
            2u8,
            3u8,
            4u8,
            5u8,
            6u8,
            7u8,
            8u8,
            9u8,
            10u8,
            11u8,
            (
                12u8,
                13u8,
                14u8,
                15u8,
                16u8,
                17u8,
                18u8,
                19u8,
                20u8,
                21u8,
                22u8,
                (23u8,),
            ),
        );

        let trans_hlist = HlistToSegments::new(Count).run(elems);
        let total = HlistComposeBalanced.run(trans_hlist);

        assert_eq!(total.run(0usize), 23usize);
    }

    #[test]
    fn type_checks() {
        type Out = <HlistToSegments<Count> as OpTy<(u8, (u8,))>>::OutTy;
        // Note: the second element type is `(u8,)`, so its segment type is `((u8,),)`.
        let _x: Out = (
            super::super::super::runtime::SegmentFoldL::new(Count, (0u8,)),
            super::super::super::runtime::SegmentFoldL::new(Count, ((0u8,),)),
        );
    }
}
