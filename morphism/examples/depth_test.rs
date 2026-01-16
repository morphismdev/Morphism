//! Runtime compilation depth stress test (value-level).
//!
//! Goal:
//! - keep a **compile-time stress** shape (very deep tuple type)
//! - but also run the **largest runtime fold** we can under `recursion_limit = 128`
//!
//! Note: for the runtime run, we construct the payload via `mem::zeroed()`.
//! This is safe here because the payload is a nested tuple of `u8` (all bit-patterns valid).

#![recursion_limit = "128"]

use morphism::{fold_children_l_prog, op_lift, Compile, HlistNode, OpOnce, OpTy, Tagged};

macro_rules! chunk11_ty {
    ($t:ty, $tail:ty) => {
        ($t, $t, $t, $t, $t, $t, $t, $t, $t, $t, $t, $tail)
    };
}
macro_rules! chunk33_ty {
    ($t:ty, $tail:ty) => {
        chunk11_ty!($t, chunk11_ty!($t, chunk11_ty!($t, $tail)))
    };
}
macro_rules! chunk99_ty {
    ($t:ty, $tail:ty) => {
        chunk33_ty!($t, chunk33_ty!($t, chunk33_ty!($t, $tail)))
    };
}

// --- Types ---
pub type Tail0 = (u8,);
pub type Tail1 = chunk99_ty!(u8, Tail0);
pub type Tail2 = chunk99_ty!(u8, Tail1);
pub type Tail3 = chunk99_ty!(u8, Tail2);
pub type Tail4 = chunk99_ty!(u8, Tail3);
pub type Tail5 = chunk99_ty!(u8, Tail4);
pub type Tail6 = chunk99_ty!(u8, Tail5);
pub type Tail7 = chunk99_ty!(u8, Tail6);
pub type Tail8 = chunk99_ty!(u8, Tail7);
pub type Tail9 = chunk99_ty!(u8, Tail8);
pub type Tail10 = chunk99_ty!(u8, Tail9);
pub type Tail11 = chunk99_ty!(u8, Tail10);
pub type Tail12 = chunk99_ty!(u8, Tail11);
pub type Tail13 = chunk99_ty!(u8, Tail12);
pub type Tail14 = chunk99_ty!(u8, Tail13);
pub type Tail15 = chunk99_ty!(u8, Tail14);

mod config {
    use super::*;
    pub type BigChildren = Tail13; // 1+99⋅13=1288 elements
    pub type BigFoldProg = HlistNode<BigChildren>;
}

mod ops {
    use super::*;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct CountL;

    impl<S> OpTy<(usize, S)> for CountL {
        type OutTy = usize;
    }

    // Left fold step: (acc, elem) -> acc
    impl<S> OpOnce<(usize, S)> for CountL {
        type OutVal = usize;

        #[inline]
        fn run(self, (acc, _elem): (usize, S)) -> Self::OutVal {
            acc + 1
        }
    }
}

fn main() {
    use config::*;
    use ops::*;

    // --- Compile-time stress (do not run) ---
    // Force the closed compiler to lower+reify for a very deep tuple type.
    let prog_compile_only = fold_children_l_prog(op_lift(CountL), 0usize);
    let _compiled_big = Compile::<BigFoldProg>::new().run(prog_compile_only);

    // --- Runtime run (largest that fits under recursion_limit=128) ---
    type RtChildren = Tail12;
    type RtProg = HlistNode<RtChildren>;
    const RT_LEN: usize = 1 + 99 * 12;

    let prog_rt = fold_children_l_prog(op_lift(CountL), 0usize);
    let compiled_rt = Compile::<RtProg>::new().run(prog_rt);
    let input_rt: RtProg = Tagged::new(unsafe { core::mem::zeroed::<RtChildren>() });
    let out = compiled_rt.run(input_rt);
    assert_eq!(out, RT_LEN);
}