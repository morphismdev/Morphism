use core::marker::PhantomData;

use super::lower::LowerTable;
use super::reify::ReifyTable;
use crate::OpOnce;

/// A compiled artifact that is guaranteed to be runnable on `P`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Compiled<P, A> {
    pub prog: A,
    _p: PhantomData<fn() -> P>,
}

impl<P, A> Compiled<P, A> {
    #[inline]
    pub const fn new(prog: A) -> Self {
        Self {
            prog,
            _p: PhantomData,
        }
    }
}

impl<P, A> OpOnce<P> for Compiled<P, A>
where
    A: OpOnce<P>,
{
    type OutVal = <A as OpOnce<P>>::OutVal;

    #[inline]
    fn run(self, p: P) -> Self::OutVal {
        self.prog.run(p)
    }
}

/// Stable migration entrypoint: compile using the **closed-table** pipeline (closed lowering → closed reify).
///
/// This intentionally fails to compile if `Prog` contains keys not yet supported by the closed tables.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Compile<P>(PhantomData<fn() -> P>);

impl<P> Compile<P> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<P, Prog> OpOnce<Prog> for Compile<P>
where
    LowerTable<P>: OpOnce<Prog>,
    ReifyTable<P>: OpOnce<<LowerTable<P> as OpOnce<Prog>>::OutVal>,
{
    type OutVal =
        Compiled<P, <ReifyTable<P> as OpOnce<<LowerTable<P> as OpOnce<Prog>>::OutVal>>::OutVal>;

    #[inline]
    fn run(self, prog: Prog) -> Self::OutVal {
        let lowered = LowerTable::<P>::new().run(prog);
        let runnable = ReifyTable::<P>::new().run(lowered);
        Compiled::new(runnable)
    }
}
/// Convenience helper: compile a program using the closed pipeline.
#[inline]
pub fn compile<P, Prog>(prog: Prog) -> <Compile<P> as OpOnce<Prog>>::OutVal
where
    Compile<P>: OpOnce<Prog>,
{
    Compile::<P>::new().run(prog)
}
