use crate::IX_HLIST_PUSH_BACK;
use crate::{NullaryToken, OpOnce, Tagged};

/// Push a single element onto the end of an **HList-encoded tuple**
/// (chunked at 11 with a recursive tail).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HlistPushBack;

/// Push-back program: reflectable syntax node.
pub type HlistPushBackProg = Tagged<IX_HLIST_PUSH_BACK, NullaryToken>;

/// Construct a push-back program node.
#[inline]
pub const fn hlist_push_back_prog() -> HlistPushBackProg {
    Tagged::new(NullaryToken)
}

/// Internal helper op: captures the accumulator in `Self`, takes only `E` as input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PushBackAcc<Acc>(Acc);

// ─────────────
// Base + small: self-contained (no TuplePushBack)
// ─────────────

impl<E> OpOnce<E> for PushBackAcc<()> {
    type OutVal = (E,);

    #[inline]
    fn run(self, e: E) -> Self::OutVal {
        let () = self.0;
        (e,)
    }
}

macro_rules! impl_small {
    ($( ($($A:ident : $a:ident),+ ) ),+ $(,)?) => {$(
        impl<E, $($A,)+> OpOnce<E> for PushBackAcc<( $($A,)+ )> {
            type OutVal = ( $($A,)+ E, );

            #[inline]
            fn run(self, e: E) -> Self::OutVal {
                let ( $($a,)+ ) = self.0;
                ( $($a,)+ e, )
            }
        }
    )+};
}

impl_small!(
    (A0: a0),
    (A0: a0, A1: a1),
    (A0: a0, A1: a1, A2: a2),
    (A0: a0, A1: a1, A2: a2, A3: a3),
    (A0: a0, A1: a1, A2: a2, A3: a3, A4: a4),
    (A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5),
    (A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5, A6: a6),
    (A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5, A6: a6, A7: a7),
    (A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5, A6: a6, A7: a7, A8: a8),
    (A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5, A6: a6, A7: a7, A8: a8, A9: a9),
);

// Exactly 11: start tail (custom, no TuplePushBack)
impl<E, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> OpOnce<E>
    for PushBackAcc<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10)>
{
    type OutVal = (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, (E,));
    #[inline]
    fn run(self, e: E) -> Self::OutVal {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10) = self.0;
        (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, (e,))
    }
}

// Tail case: recurse structurally on Tail via PushBackAcc<Tail>: OpOnce<E>
impl<E, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail> OpOnce<E>
    for PushBackAcc<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail)>
where
    PushBackAcc<Tail>: OpOnce<E>,
{
    type OutVal = (
        A0,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        <PushBackAcc<Tail> as OpOnce<E>>::OutVal,
    );

    #[inline]
    fn run(self, e: E) -> Self::OutVal {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, tail) = self.0;
        let tail_next = PushBackAcc(tail).run(e);
        (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, tail_next)
    }
}

// Public op: (Acc, E) -> AccOut, implemented via PushBackAcc(acc).run(e)
impl<Acc, E> OpOnce<(Acc, E)> for HlistPushBack
where
    PushBackAcc<Acc>: OpOnce<E>,
{
    type OutVal = <PushBackAcc<Acc> as OpOnce<E>>::OutVal;

    #[inline]
    fn run(self, (acc, e): (Acc, E)) -> Self::OutVal {
        PushBackAcc(acc).run(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OpOnce;

    #[test]
    fn push_back_small_self_contained() {
        let out = HlistPushBack.run(((), 0u8));
        assert_eq!(out, (0u8,));

        let out = HlistPushBack.run(((1u16,), 2u32));
        assert_eq!(out, (1u16, 2u32));

        let out = HlistPushBack.run(((1u8, 2u16), 3u32));
        assert_eq!(out, (1u8, 2u16, 3u32));
    }

    #[test]
    fn push_back_into_len_12_recurses_on_tail() {
        let acc = (
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
            (12u8,),
        );
        let out = HlistPushBack.run((acc, 13u8));
        assert_eq!(out, (1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, (12u8, 13u8,)));
    }
}
