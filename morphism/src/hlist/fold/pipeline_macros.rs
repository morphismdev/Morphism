//! Shared macro implementations for `fold_l` and `fold_r` pipeline modules.
//!
//! These are invoked from the per-direction pipeline modules to generate the concrete
//! types while keeping the logic single-sourced.

// ─────────────────────────────────────────────────────────────────────────────
// Helpers (module-scope) — avoid nested macro_rules! inside macro expansions
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! __tuple_compose_pairs_impls {
    ($pair_ty:ident, $pair_val:ident; $( ( $( $T:ident $t:ident ),+ ) );+ $(;)? ) => {
        $(
            impl<$( $T ),+> crate::OpTy<( $( $T, )+ )> for TupleComposePairs {
                type OutTy = __tuple_compose_pairs_impls!(@out_ty $pair_ty [] [ $( $T $t ),+ ]);
            }

            impl<$( $T ),+> crate::OpOnce<( $( $T, )+ )> for TupleComposePairs {
                type OutVal = __tuple_compose_pairs_impls!(@out_ty $pair_ty [] [ $( $T $t ),+ ]);

                #[inline]
                fn run(self, ( $( $t, )+ ): ( $( $T, )+ )) -> Self::OutVal {
                    __tuple_compose_pairs_impls!(@out_val $pair_val [] [ $( $T $t ),+ ])
                }
            }
        )+
    };

    // ---- type-level builder: (pair(T0,T1), pair(T2,T3), ..., last?)
    (@out_ty $pair_ty:ident [$($acc:tt)*] [ $A:ident $a:ident, $B:ident $b:ident $(, $restT:ident $restv:ident )* ]) => {
        __tuple_compose_pairs_impls!(
            @out_ty
            $pair_ty
            [$($acc)* $pair_ty!($A, $B),]
            [ $( $restT $restv ),* ]
        )
    };
    (@out_ty $pair_ty:ident [$($acc:tt)*] [ $LastT:ident $lastv:ident ]) => {
        ( $($acc)* $LastT, )
    };
    (@out_ty $pair_ty:ident [$($acc:tt)*] [ ]) => {
        ( $($acc)* )
    };

    // ---- value-level builder: (pair_val(t0,t1), ..., last?)
    (@out_val $pair_val:ident [$($acc:tt)*] [ $A:ident $a:ident, $B:ident $b:ident $(, $restT:ident $restv:ident )* ]) => {
        __tuple_compose_pairs_impls!(
            @out_val
            $pair_val
            [$($acc)* $pair_val!($a, $b),]
            [ $( $restT $restv ),* ]
        )
    };
    (@out_val $pair_val:ident [$($acc:tt)*] [ $LastT:ident $lastv:ident ]) => {
        ( $($acc)* $lastv, )
    };
    (@out_val $pair_val:ident [$($acc:tt)*] [ ]) => {
        ( $($acc)* )
    };
}

macro_rules! __hlist_pairs_tail_flat_impls {
    ($pair_ty:ident, $pair_val:ident; $( ( $( $B:ident $b:ident ),+ ) );+ $(;)? ) => {
        $(
            impl<
                    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,
                    $( $B, )+
                > crate::OpTy<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>
                for HlistComposePairs
            {
                type OutTy = __hlist_pairs_tail_flat_impls!(
                    @out_ty
                    $pair_ty
                    []
                    [A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, $( $B $b ),+ ]
                );
            }

            impl<
                    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,
                    $( $B, )+
                > crate::OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>
                for HlistComposePairs
            {
                type OutVal = __hlist_pairs_tail_flat_impls!(
                    @out_ty
                    $pair_ty
                    []
                    [A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, $( $B $b ),+ ]
                );

                #[inline]
                fn run(
                    self,
                    args: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ )),
                ) -> Self::OutVal {
                    let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, tail) = args;
                    let ( $( $b, )+ ) = tail;

                    __hlist_pairs_tail_flat_impls!(
                        @out_val
                        $pair_val
                        []
                        [A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, $( $B $b ),+ ]
                    )
                }
            }
        )+
    };

    // ---- type-level builder: (pair(X0,X1), pair(X2,X3), ..., last?)
    (@out_ty $pair_ty:ident [$($acc:tt)*] [ $X:ident $x:ident, $Y:ident $y:ident $(, $restT:ident $restv:ident )* ]) => {
        __hlist_pairs_tail_flat_impls!(
            @out_ty
            $pair_ty
            [$($acc)* $pair_ty!($X, $Y),]
            [ $( $restT $restv ),* ]
        )
    };
    (@out_ty $pair_ty:ident [$($acc:tt)*] [ $LastT:ident $lastv:ident ]) => {
        ( $($acc)* $LastT, )
    };
    (@out_ty $pair_ty:ident [$($acc:tt)*] [ ]) => {
        ( $($acc)* )
    };

    // ---- value-level builder: (pair_val(x0,x1), ..., last?)
    (@out_val $pair_val:ident [$($acc:tt)*] [ $X:ident $x:ident, $Y:ident $y:ident $(, $restT:ident $restv:ident )* ]) => {
        __hlist_pairs_tail_flat_impls!(
            @out_val
            $pair_val
            [$($acc)* $pair_val!($x, $y),]
            [ $( $restT $restv ),* ]
        )
    };
    (@out_val $pair_val:ident [$($acc:tt)*] [ $LastT:ident $lastv:ident ]) => {
        ( $($acc)* $lastv, )
    };
    (@out_val $pair_val:ident [$($acc:tt)*] [ ]) => {
        ( $($acc)* )
    };
}

macro_rules! __delegate_flat_hlist_pairs {
    ($($T:ident),+ $(,)?) => {
        impl<$($T,)+> crate::OpTy<($($T,)+)> for HlistComposePairs
        where
            TupleComposePairs: crate::OpTy<($($T,)+)>,
        {
            type OutTy = <TupleComposePairs as crate::OpTy<($($T,)+)>>::OutTy;
        }

        impl<$($T,)+> crate::OpOnce<($($T,)+)> for HlistComposePairs
        where
            TupleComposePairs: crate::OpOnce<($($T,)+)>,
        {
            type OutVal = <TupleComposePairs as crate::OpOnce<($($T,)+)>>::OutVal;

            #[inline]
            fn run(self, args: ($($T,)+)) -> Self::OutVal {
                TupleComposePairs.run(args)
            }
        }
    };
}

macro_rules! __impl_balanced_tuple {
    ($($T:ident),+ $(,)?) => {
        impl<$($T,)+> crate::OpTy<($($T,)+)> for TupleComposeBalanced
        where
            super::compose_pairs::TupleComposePairs: crate::OpTy<($($T,)+)>,
            TupleComposeBalanced: crate::OpTy<
                <super::compose_pairs::TupleComposePairs as crate::OpTy<($($T,)+)>>::OutTy
            >,
        {
            type OutTy = <TupleComposeBalanced as crate::OpTy<
                <super::compose_pairs::TupleComposePairs as crate::OpTy<($($T,)+)>>::OutTy
            >>::OutTy;
        }

        impl<$($T,)+> crate::OpOnce<($($T,)+)> for TupleComposeBalanced
        where
            super::compose_pairs::TupleComposePairs: crate::OpOnce<($($T,)+)>,
            TupleComposeBalanced: crate::OpOnce<
                <super::compose_pairs::TupleComposePairs as crate::OpOnce<($($T,)+)>>::OutVal
            >,
        {
            type OutVal = <TupleComposeBalanced as crate::OpOnce<
                <super::compose_pairs::TupleComposePairs as crate::OpOnce<($($T,)+)>>::OutVal
            >>::OutVal;

            #[inline]
            fn run(self, args: ($($T,)+)) -> Self::OutVal {
                let layer = super::compose_pairs::TupleComposePairs.run(args);
                TupleComposeBalanced.run(layer)
            }
        }
    };
}

macro_rules! __delegate_tuple_balanced_hlist {
    ($($T:ident),+ $(,)?) => {
        impl<$($T,)+> crate::OpTy<($($T,)+)> for HlistComposeBalanced {
            type OutTy = <TupleComposeBalanced as crate::OpTy<($($T,)+)>>::OutTy;
        }

        impl<$($T,)+> crate::OpOnce<($($T,)+)> for HlistComposeBalanced {
            type OutVal = <TupleComposeBalanced as crate::OpOnce<($($T,)+)>>::OutVal;

            #[inline]
            fn run(self, args: ($($T,)+)) -> Self::OutVal {
                TupleComposeBalanced.run(args)
            }
        }
    };
}

macro_rules! __hlist_balanced_tail_flat_impls {
    ($( ( $( $B:ident $b:ident ),+ ) );+ $(;)?) => {
        $(
            impl<
                    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,
                    $( $B, )+
                > crate::OpTy<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>
                for HlistComposeBalanced
            where
                super::compose_pairs::HlistComposePairs:
                    crate::OpTy<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>,
                HlistComposeBalanced: crate::OpTy<
                    <super::compose_pairs::HlistComposePairs as crate::OpTy<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>>::OutTy
                >,
            {
                type OutTy = <HlistComposeBalanced as crate::OpTy<
                    <super::compose_pairs::HlistComposePairs as crate::OpTy<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>>::OutTy
                >>::OutTy;
            }

            impl<
                    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,
                    $( $B, )+
                > crate::OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>
                for HlistComposeBalanced
            where
                super::compose_pairs::HlistComposePairs:
                    crate::OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>,
                HlistComposeBalanced: crate::OpOnce<
                    <super::compose_pairs::HlistComposePairs as crate::OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>>::OutVal
                >,
            {
                type OutVal = <HlistComposeBalanced as crate::OpOnce<
                    <super::compose_pairs::HlistComposePairs as crate::OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>>::OutVal
                >>::OutVal;

                #[inline]
                fn run(
                    self,
                    args: (
                        A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,
                        ( $( $B, )+ ),
                    ),
                ) -> Self::OutVal {
                    let layer = super::compose_pairs::HlistComposePairs.run(args);
                    HlistComposeBalanced.run(layer)
                }
            }
        )+
    };
}

macro_rules! __map_small_to_segments_impls {
    ($Seg:ident; $( ( $( $A:ident $a:ident ),+ ) );+ $(;)? ) => {
        $(
            impl<F, $( $A, )+> crate::OpTy<( $( $A, )+ )> for HlistToSegments<F> {
                type OutTy = ( $( $Seg<F, ($A,)>, )+ );
            }

            impl<F, $( $A, )+> crate::OpOnce<( $( $A, )+ )> for HlistToSegments<F>
            where
                F: Clone,
            {
                type OutVal = ( $( $Seg<F, ($A,)>, )+ );

                #[inline]
                fn run(self, ( $( $a, )+ ): ( $( $A, )+ )) -> Self::OutVal {
                    let f = self.0;
                    ( $( $Seg::new(f.clone(), ($a,)), )+ )
                }
            }
        )+
    };
}

macro_rules! __map_hlist_head_tail_small_impls {
    ($Seg:ident; $( ( $( $B:ident $b:ident ),+ ) );+ $(;)? ) => {
        $(
            impl<
                    F,
                    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,
                    $( $B, )+
                > crate::OpTy<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>
                for HlistToSegments<F>
            {
                type OutTy = (
                    $Seg<F, (A0,)>,
                    $Seg<F, (A1,)>,
                    $Seg<F, (A2,)>,
                    $Seg<F, (A3,)>,
                    $Seg<F, (A4,)>,
                    $Seg<F, (A5,)>,
                    $Seg<F, (A6,)>,
                    $Seg<F, (A7,)>,
                    $Seg<F, (A8,)>,
                    $Seg<F, (A9,)>,
                    $Seg<F, (A10,)>,
                    ( $( $Seg<F, ($B,)>, )+ ),
                );
            }

            impl<
                    F,
                    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,
                    $( $B, )+
                > crate::OpOnce<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, ( $( $B, )+ ))>
                for HlistToSegments<F>
            where
                F: Clone,
            {
                type OutVal = (
                    $Seg<F, (A0,)>,
                    $Seg<F, (A1,)>,
                    $Seg<F, (A2,)>,
                    $Seg<F, (A3,)>,
                    $Seg<F, (A4,)>,
                    $Seg<F, (A5,)>,
                    $Seg<F, (A6,)>,
                    $Seg<F, (A7,)>,
                    $Seg<F, (A8,)>,
                    $Seg<F, (A9,)>,
                    $Seg<F, (A10,)>,
                    ( $( $Seg<F, ($B,)>, )+ ),
                );

                #[inline]
                fn run(
                    self,
                    args: (
                        A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,
                        ( $( $B, )+ ),
                    ),
                ) -> Self::OutVal {
                    let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, tail) = args;
                    let ( $( $b, )+ ) = tail;
                    let f = self.0;
                    (
                        $Seg::new(f.clone(), (a0,)),
                        $Seg::new(f.clone(), (a1,)),
                        $Seg::new(f.clone(), (a2,)),
                        $Seg::new(f.clone(), (a3,)),
                        $Seg::new(f.clone(), (a4,)),
                        $Seg::new(f.clone(), (a5,)),
                        $Seg::new(f.clone(), (a6,)),
                        $Seg::new(f.clone(), (a7,)),
                        $Seg::new(f.clone(), (a8,)),
                        $Seg::new(f.clone(), (a9,)),
                        $Seg::new(f.clone(), (a10,)),
                        ( $( $Seg::new(f.clone(), ($b,)), )+ ),
                    )
                }
            }
        )+
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// Public macros used by the per-direction pipeline modules
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! define_compose_pairs {
    ($pair_ty:ident, $pair_val:ident $(,)?) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct TupleComposePairs;

        __tuple_compose_pairs_impls! {
            $pair_ty, $pair_val;
            (T0 t0);
            (T0 t0, T1 t1);
            (T0 t0, T1 t1, T2 t2);
            (T0 t0, T1 t1, T2 t2, T3 t3);
            (T0 t0, T1 t1, T2 t2, T3 t3, T4 t4);
            (T0 t0, T1 t1, T2 t2, T3 t3, T4 t4, T5 t5);
            (T0 t0, T1 t1, T2 t2, T3 t3, T4 t4, T5 t5, T6 t6);
            (T0 t0, T1 t1, T2 t2, T3 t3, T4 t4, T5 t5, T6 t6, T7 t7);
            (T0 t0, T1 t1, T2 t2, T3 t3, T4 t4, T5 t5, T6 t6, T7 t7, T8 t8);
            (T0 t0, T1 t1, T2 t2, T3 t3, T4 t4, T5 t5, T6 t6, T7 t7, T8 t8, T9 t9);
            (T0 t0, T1 t1, T2 t2, T3 t3, T4 t4, T5 t5, T6 t6, T7 t7, T8 t8, T9 t9, T10 t10);
        }

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct HlistComposePairs;

        __delegate_flat_hlist_pairs!(T0);
        __delegate_flat_hlist_pairs!(T0, T1);
        __delegate_flat_hlist_pairs!(T0, T1, T2);
        __delegate_flat_hlist_pairs!(T0, T1, T2, T3);
        __delegate_flat_hlist_pairs!(T0, T1, T2, T3, T4);
        __delegate_flat_hlist_pairs!(T0, T1, T2, T3, T4, T5);
        __delegate_flat_hlist_pairs!(T0, T1, T2, T3, T4, T5, T6);
        __delegate_flat_hlist_pairs!(T0, T1, T2, T3, T4, T5, T6, T7);
        __delegate_flat_hlist_pairs!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
        __delegate_flat_hlist_pairs!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
        __delegate_flat_hlist_pairs!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

        // Tail is arity 12: (B0..B10, Tail2)
        impl<
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
                B0,
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                Tail2,
            >
            crate::OpTy<(
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
                (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
            )> for HlistComposePairs
        {
            type OutTy = (
                $pair_ty!(A0, A1),
                $pair_ty!(A2, A3),
                $pair_ty!(A4, A5),
                $pair_ty!(A6, A7),
                $pair_ty!(A8, A9),
                $pair_ty!(A10, B0),
                $pair_ty!(B1, B2),
                $pair_ty!(B3, B4),
                $pair_ty!(B5, B6),
                $pair_ty!(B7, B8),
                $pair_ty!(B9, B10),
                Tail2,
            );
        }

        impl<
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
                B0,
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                Tail2,
            >
            crate::OpOnce<(
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
                (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
            )> for HlistComposePairs
        {
            type OutVal = (
                $pair_ty!(A0, A1),
                $pair_ty!(A2, A3),
                $pair_ty!(A4, A5),
                $pair_ty!(A6, A7),
                $pair_ty!(A8, A9),
                $pair_ty!(A10, B0),
                $pair_ty!(B1, B2),
                $pair_ty!(B3, B4),
                $pair_ty!(B5, B6),
                $pair_ty!(B7, B8),
                $pair_ty!(B9, B10),
                Tail2,
            );

            #[inline]
            fn run(
                self,
                args: (
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
                    (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
                ),
            ) -> Self::OutVal {
                let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, tail) = args;
                let (b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, tail2) = tail;

                (
                    $pair_val!(a0, a1),
                    $pair_val!(a2, a3),
                    $pair_val!(a4, a5),
                    $pair_val!(a6, a7),
                    $pair_val!(a8, a9),
                    $pair_val!(a10, b0),
                    $pair_val!(b1, b2),
                    $pair_val!(b3, b4),
                    $pair_val!(b5, b6),
                    $pair_val!(b7, b8),
                    $pair_val!(b9, b10),
                    tail2,
                )
            }
        }

        __hlist_pairs_tail_flat_impls! {
            $pair_ty, $pair_val;
            (B0 b0);
            (B0 b0, B1 b1);
            (B0 b0, B1 b1, B2 b2);
            (B0 b0, B1 b1, B2 b2, B3 b3);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8, B9 b9);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8, B9 b9, B10 b10);
        }
    };
}

macro_rules! define_compose_balanced {
    ($pair_ty:ident, $pair_val:ident $(,)?) => {
        use crate::{OpOnce, OpTy};

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct TupleComposeBalanced;

        impl<T0> OpTy<(T0,)> for TupleComposeBalanced {
            type OutTy = T0;
        }
        impl<T0> OpOnce<(T0,)> for TupleComposeBalanced {
            type OutVal = T0;
            #[inline]
            fn run(self, (t0,): (T0,)) -> Self::OutVal {
                t0
            }
        }

        impl<T0, T1> OpTy<(T0, T1)> for TupleComposeBalanced {
            type OutTy = $pair_ty!(T0, T1);
        }
        impl<T0, T1> OpOnce<(T0, T1)> for TupleComposeBalanced {
            type OutVal = $pair_ty!(T0, T1);
            #[inline]
            fn run(self, (t0, t1): (T0, T1)) -> Self::OutVal {
                $pair_val!(t0, t1)
            }
        }

        __impl_balanced_tuple!(T0, T1, T2);
        __impl_balanced_tuple!(T0, T1, T2, T3);
        __impl_balanced_tuple!(T0, T1, T2, T3, T4);
        __impl_balanced_tuple!(T0, T1, T2, T3, T4, T5);
        __impl_balanced_tuple!(T0, T1, T2, T3, T4, T5, T6);
        __impl_balanced_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
        __impl_balanced_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
        __impl_balanced_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
        __impl_balanced_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct HlistComposeBalanced;

        impl<T0> OpTy<(T0,)> for HlistComposeBalanced {
            type OutTy = <TupleComposeBalanced as OpTy<(T0,)>>::OutTy;
        }
        impl<T0> OpOnce<(T0,)> for HlistComposeBalanced {
            type OutVal = <TupleComposeBalanced as OpOnce<(T0,)>>::OutVal;
            #[inline]
            fn run(self, args: (T0,)) -> Self::OutVal {
                TupleComposeBalanced.run(args)
            }
        }

        impl<T0, T1> OpTy<(T0, T1)> for HlistComposeBalanced {
            type OutTy = <TupleComposeBalanced as OpTy<(T0, T1)>>::OutTy;
        }
        impl<T0, T1> OpOnce<(T0, T1)> for HlistComposeBalanced {
            type OutVal = <TupleComposeBalanced as OpOnce<(T0, T1)>>::OutVal;
            #[inline]
            fn run(self, args: (T0, T1)) -> Self::OutVal {
                TupleComposeBalanced.run(args)
            }
        }

        __delegate_tuple_balanced_hlist!(T0, T1, T2);
        __delegate_tuple_balanced_hlist!(T0, T1, T2, T3);
        __delegate_tuple_balanced_hlist!(T0, T1, T2, T3, T4);
        __delegate_tuple_balanced_hlist!(T0, T1, T2, T3, T4, T5);
        __delegate_tuple_balanced_hlist!(T0, T1, T2, T3, T4, T5, T6);
        __delegate_tuple_balanced_hlist!(T0, T1, T2, T3, T4, T5, T6, T7);
        __delegate_tuple_balanced_hlist!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
        __delegate_tuple_balanced_hlist!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
        __delegate_tuple_balanced_hlist!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

        // Tail is a "flat tuple" with arity 1..=11: (B0..B10)
        //
        // This corresponds to overall lengths 12..=22 elements in the hlist encoding:
        // (A0..A10, (B0..Bn)). One layer of pairing reduces the structure to <= 11 elements,
        // which then uses the tuple-backed balanced composition impls above.
        __hlist_balanced_tail_flat_impls! {
            (B0 b0);
            (B0 b0, B1 b1);
            (B0 b0, B1 b1, B2 b2);
            (B0 b0, B1 b1, B2 b2, B3 b3);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8, B9 b9);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8, B9 b9, B10 b10);
        }

        // Tail is a chunked hlist tail of arity 12: (B0..B10, Tail2).
        //
        // This corresponds to overall length >= 23 elements:
        // (A0..A10, (B0..B10, Tail2)).
        //
        // Optimization: run *two* pairwise layers per recursion step to reduce the amount of
        // type-level recursion needed to "drain" the nested 11-chunk tail encoding.
        impl<
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
                B0,
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                Tail2,
            >
            OpTy<(
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
                (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
            )> for HlistComposeBalanced
        where
            super::compose_pairs::HlistComposePairs: OpTy<(
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
                (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
            )>,
            super::compose_pairs::HlistComposePairs: OpTy<
                <super::compose_pairs::HlistComposePairs as OpTy<(
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
                    (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
                )>>::OutTy,
            >,
            HlistComposeBalanced: OpTy<
                <super::compose_pairs::HlistComposePairs as OpTy<
                    <super::compose_pairs::HlistComposePairs as OpTy<(
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
                        (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
                    )>>::OutTy,
                >>::OutTy,
            >,
        {
            type OutTy = <HlistComposeBalanced as OpTy<
                <super::compose_pairs::HlistComposePairs as OpTy<
                    <super::compose_pairs::HlistComposePairs as OpTy<(
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
                        (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
                    )>>::OutTy,
                >>::OutTy,
            >>::OutTy;
        }

        impl<
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
                B0,
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                Tail2,
            >
            OpOnce<(
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
                (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
            )> for HlistComposeBalanced
        where
            super::compose_pairs::HlistComposePairs: OpOnce<(
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
                (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
            )>,
            super::compose_pairs::HlistComposePairs: OpOnce<
                <super::compose_pairs::HlistComposePairs as OpOnce<(
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
                    (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
                )>>::OutVal,
            >,
            HlistComposeBalanced: OpOnce<
                <super::compose_pairs::HlistComposePairs as OpOnce<
                    <super::compose_pairs::HlistComposePairs as OpOnce<(
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
                        (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
                    )>>::OutVal,
                >>::OutVal,
            >,
        {
            type OutVal = <HlistComposeBalanced as OpOnce<
                <super::compose_pairs::HlistComposePairs as OpOnce<
                    <super::compose_pairs::HlistComposePairs as OpOnce<(
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
                        (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
                    )>>::OutVal,
                >>::OutVal,
            >>::OutVal;

            #[inline]
            fn run(
                self,
                args: (
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
                    (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
                ),
            ) -> Self::OutVal {
                let layer1 = super::compose_pairs::HlistComposePairs.run(args);
                let layer2 = super::compose_pairs::HlistComposePairs.run(layer1);
                HlistComposeBalanced.run(layer2)
            }
        }
    };
}

macro_rules! define_map_to_segments {
    ($Seg:ident $(,)?) => {
        use super::super::runtime::$Seg;
        use crate::{OpOnce, OpTy, OpTyOut};

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct HlistToSegments<F>(pub F);

        impl<F> HlistToSegments<F> {
            #[inline]
            pub const fn new(f: F) -> Self {
                Self(f)
            }
        }

        __map_small_to_segments_impls! {
            $Seg;
            (A0 a0);
            (A0 a0, A1 a1);
            (A0 a0, A1 a1, A2 a2);
            (A0 a0, A1 a1, A2 a2, A3 a3);
            (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4);
            (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5);
            (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6);
            (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7);
            (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8);
            (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9);
            (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10);
        }

        // HList tail with arity <= 11: flatten directly into segments (no recursion).
        __map_hlist_head_tail_small_impls! {
            $Seg;
            (B0 b0);
            (B0 b0, B1 b1);
            (B0 b0, B1 b1, B2 b2);
            (B0 b0, B1 b1, B2 b2, B3 b3);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8, B9 b9);
            (B0 b0, B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8, B9 b9, B10 b10);
        }

        // HList tail with arity 12: consume *two* full 11-element chunks at a time.
        //
        //   (A0..A10, (B0..B10, Tail2))  ->  (Seg(A0)..Seg(A10), Seg(B0)..Seg(B10), map(Tail2))
        //
        // This reduces the recursion depth of the mapping stage by ~2x for long hlists.
        impl<
                F,
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
                B0,
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                Tail2,
            >
            OpTy<(
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
                (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
            )> for HlistToSegments<F>
        where
            HlistToSegments<F>: OpTy<Tail2>,
        {
            type OutTy = (
                $Seg<F, (A0,)>,
                $Seg<F, (A1,)>,
                $Seg<F, (A2,)>,
                $Seg<F, (A3,)>,
                $Seg<F, (A4,)>,
                $Seg<F, (A5,)>,
                $Seg<F, (A6,)>,
                $Seg<F, (A7,)>,
                $Seg<F, (A8,)>,
                $Seg<F, (A9,)>,
                $Seg<F, (A10,)>,
                (
                    $Seg<F, (B0,)>,
                    $Seg<F, (B1,)>,
                    $Seg<F, (B2,)>,
                    $Seg<F, (B3,)>,
                    $Seg<F, (B4,)>,
                    $Seg<F, (B5,)>,
                    $Seg<F, (B6,)>,
                    $Seg<F, (B7,)>,
                    $Seg<F, (B8,)>,
                    $Seg<F, (B9,)>,
                    $Seg<F, (B10,)>,
                    OpTyOut<HlistToSegments<F>, Tail2>,
                ),
            );
        }

        impl<
                F,
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
                B0,
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                Tail2,
            >
            OpOnce<(
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
                (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
            )> for HlistToSegments<F>
        where
            F: Clone,
            HlistToSegments<F>: OpOnce<Tail2>,
        {
            type OutVal = (
                $Seg<F, (A0,)>,
                $Seg<F, (A1,)>,
                $Seg<F, (A2,)>,
                $Seg<F, (A3,)>,
                $Seg<F, (A4,)>,
                $Seg<F, (A5,)>,
                $Seg<F, (A6,)>,
                $Seg<F, (A7,)>,
                $Seg<F, (A8,)>,
                $Seg<F, (A9,)>,
                $Seg<F, (A10,)>,
                (
                    $Seg<F, (B0,)>,
                    $Seg<F, (B1,)>,
                    $Seg<F, (B2,)>,
                    $Seg<F, (B3,)>,
                    $Seg<F, (B4,)>,
                    $Seg<F, (B5,)>,
                    $Seg<F, (B6,)>,
                    $Seg<F, (B7,)>,
                    $Seg<F, (B8,)>,
                    $Seg<F, (B9,)>,
                    $Seg<F, (B10,)>,
                    <HlistToSegments<F> as OpOnce<Tail2>>::OutVal,
                ),
            );

            #[inline]
            fn run(
                self,
                args: (
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
                    (B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, Tail2),
                ),
            ) -> Self::OutVal {
                let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, tail) = args;
                let (b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, tail2) = tail;
                let f = self.0;
                (
                    $Seg::new(f.clone(), (a0,)),
                    $Seg::new(f.clone(), (a1,)),
                    $Seg::new(f.clone(), (a2,)),
                    $Seg::new(f.clone(), (a3,)),
                    $Seg::new(f.clone(), (a4,)),
                    $Seg::new(f.clone(), (a5,)),
                    $Seg::new(f.clone(), (a6,)),
                    $Seg::new(f.clone(), (a7,)),
                    $Seg::new(f.clone(), (a8,)),
                    $Seg::new(f.clone(), (a9,)),
                    $Seg::new(f.clone(), (a10,)),
                    (
                        $Seg::new(f.clone(), (b0,)),
                        $Seg::new(f.clone(), (b1,)),
                        $Seg::new(f.clone(), (b2,)),
                        $Seg::new(f.clone(), (b3,)),
                        $Seg::new(f.clone(), (b4,)),
                        $Seg::new(f.clone(), (b5,)),
                        $Seg::new(f.clone(), (b6,)),
                        $Seg::new(f.clone(), (b7,)),
                        $Seg::new(f.clone(), (b8,)),
                        $Seg::new(f.clone(), (b9,)),
                        $Seg::new(f.clone(), (b10,)),
                        HlistToSegments(f).run(tail2),
                    ),
                )
            }
        }
    };
}
