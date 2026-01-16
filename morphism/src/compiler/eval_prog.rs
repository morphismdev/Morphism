use crate::{Compile, OpOnce, OpTy};

/// Bridge: execute a **program AST** as a runtime op by compiling it on-demand.
///
/// `EvalProg<Prog>` adapts a program value into an `OpOnce` step, enabling runtime engines
/// (like the HList fold/map/zip-with machinery) to accept program-defined steps while remaining
/// runtime-`OpOnce` driven.
///
/// **How it works**: implements `OpOnce<Args>` by compiling the payload program under `Args`
/// using the closed-world compiler, then running the compiled result.
///
/// **When to use**: when you have a program value and need an `OpOnce` step *right now*
/// (typically as a parameter to fold-based runtime engines).
///
/// **Cost**: introduces inner compilation (the program is compiled under the argument type
/// at runtime). Keep step programs small to avoid compile-time blowups.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvalProg<Prog>(pub Prog);

impl<Prog> EvalProg<Prog> {
    #[inline]
    pub const fn new(prog: Prog) -> Self {
        Self(prog)
    }

    #[inline]
    pub fn into_inner(self) -> Prog {
        self.0
    }
}

impl<Args, Prog> OpTy<Args> for EvalProg<Prog>
where
    Compile<Args>: OpOnce<Prog>,
    <Compile<Args> as OpOnce<Prog>>::OutVal: OpOnce<Args>,
{
    type OutTy = <<Compile<Args> as OpOnce<Prog>>::OutVal as OpOnce<Args>>::OutVal;
}

impl<Args, Prog> OpOnce<Args> for EvalProg<Prog>
where
    Compile<Args>: OpOnce<Prog>,
    <Compile<Args> as OpOnce<Prog>>::OutVal: OpOnce<Args>,
{
    type OutVal = <<Compile<Args> as OpOnce<Prog>>::OutVal as OpOnce<Args>>::OutVal;

    #[inline]
    fn run(self, args: Args) -> Self::OutVal {
        Compile::<Args>::new().run(self.0).run(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{id, op_lift, then, OpOnce};

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct Inc;

    impl OpOnce<i32> for Inc {
        type OutVal = i32;
        fn run(self, x: i32) -> Self::OutVal {
            x + 1
        }
    }

    #[test]
    fn eval_prog_runs_op_lift() {
        let step = EvalProg::new(op_lift(Inc));
        assert_eq!(step.run(41), 42);
    }

    #[test]
    fn eval_prog_runs_composed_program() {
        let prog = then(id(), op_lift(Inc));
        let step = EvalProg::new(prog);
        assert_eq!(step.run(41), 42);
    }
}
