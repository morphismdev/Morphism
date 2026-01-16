use crate::{NullaryToken, OpOnce, OpTy};

// Runtime HList ops live here (`map.rs`).
// Public AST program nodes are `HlistMapProg` in that module.

pub mod map;
pub use map::*;

mod fold;
pub use fold::*;

pub mod get_at;
pub use get_at::*;

mod hlist_push_back;
pub use hlist_push_back::*;

use crate::{Tagged, IX_HLISTNODE};

pub type HlistNode<S> = Tagged<IX_HLISTNODE, S>;

/// Split an "Hlist tuple" under the fixed-arity assumptions:
/// - arity < 12  => Tail = NullaryToken
/// - arity == 12 => last element is Tail
///
/// This is both:
/// - a **type-level** splitter (via `OpTy<T>`)
/// - a **runtime** splitter (via `OpOnce<T>`)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Hlist;

macro_rules! impl_hlist_split_no_tail {
    ( $( $A:ident : $a:ident ),+ $(,)? ) => {
        impl<$( $A, )+> OpTy<( $( $A, )+ )> for Hlist {
            type OutTy = ( ( $( $A, )+ ), NullaryToken );
        }

        impl<$( $A, )+> OpOnce<( $( $A, )+ )> for Hlist {
            type OutVal = ( ( $( $A, )+ ), NullaryToken );

            #[inline]
            fn run(self, args: ( $( $A, )+ )) -> Self::OutVal {
                (args, NullaryToken)
            }
        }
    };
}

// arity 1..=11 => Tail = NullaryToken
impl_hlist_split_no_tail!(A0: a0);
impl_hlist_split_no_tail!(A0: a0, A1: a1);
impl_hlist_split_no_tail!(A0: a0, A1: a1, A2: a2);
impl_hlist_split_no_tail!(A0: a0, A1: a1, A2: a2, A3: a3);
impl_hlist_split_no_tail!(A0: a0, A1: a1, A2: a2, A3: a3, A4: a4);
impl_hlist_split_no_tail!(A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5);
impl_hlist_split_no_tail!(A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5, A6: a6);
impl_hlist_split_no_tail!(A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5, A6: a6, A7: a7);
impl_hlist_split_no_tail!(A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5, A6: a6, A7: a7, A8: a8);
impl_hlist_split_no_tail!(A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5, A6: a6, A7: a7, A8: a8, A9: a9);
impl_hlist_split_no_tail!(A0: a0, A1: a1, A2: a2, A3: a3, A4: a4, A5: a5, A6: a6, A7: a7, A8: a8, A9: a9, A10: a10);

// arity 12 => last element is Tail
impl<A, B, C, D, E, F, G, H, I, J, K, Tail> OpTy<(A, B, C, D, E, F, G, H, I, J, K, Tail)>
    for Hlist
{
    type OutTy = ((A, B, C, D, E, F, G, H, I, J, K), Tail);
}

impl<A, B, C, D, E, F, G, H, I, J, K, Tail> OpOnce<(A, B, C, D, E, F, G, H, I, J, K, Tail)>
    for Hlist
{
    type OutVal = ((A, B, C, D, E, F, G, H, I, J, K), Tail);

    #[inline]
    fn run(self, args: (A, B, C, D, E, F, G, H, I, J, K, Tail)) -> Self::OutVal {
        let (a, b, c, d, e, f, g, h, i, j, k, tail) = args;
        ((a, b, c, d, e, f, g, h, i, j, k), tail)
    }
}
