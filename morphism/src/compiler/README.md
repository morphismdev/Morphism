## Compiler (Showcase)

This module presents a **closed-world compiler** for the showcase crate. It takes
program AST values and produces runnable `OpOnce` steps that are guaranteed to
work for a specific input type.

---

## Pipeline (what exists here)

`Program AST` → `LowerTable<Input>` → `ReifyTable<Input>` → `OpOnce<Input>`

The pipeline is deliberately **closed**: if a program contains a key that is not
supported by the closed tables, it fails to type-check. This keeps the demo
focused and prevents partially-lowered programs from slipping through.

---

## Entry points

- `Compile<Input>`: closed-world compiler entrypoint
- `compile::<Input>(prog)`: convenience wrapper
- `EvalProg<Prog>`: runtime bridge for executing a program as a step

---

## Kernel (in this showcase)

The kernel here is intentionally small and limited to what the showcase needs.
It is defined by whatever can reach reify in `compiler/reify/kernel/*`.

In this crate, the reify surface covers:
- **Combinators** (`compiler/reify/kernel/combinators.rs`)
- **Tagged** (`compiler/reify/kernel/tagged.rs`)
- **Op lift** (`compiler/reify/kernel/op.rs`)
- **HList** (`compiler/reify/kernel/hlist.rs`)
- **HList fold** (`compiler/reify/kernel/hlist_fold.rs`)

Anything outside this set must lower away before reify.

---

## `OpLift` is an explicit boundary

`IX_OP_LIFT` is the escape hatch that embeds runtime ops into a program.
It is treated as opaque: no inspection and no rewriting across it.

This is a **user-lift only** bridge in the design: the compiler itself does not
generate `IX_OP_LIFT` during lowering.

---

## `EvalProg` (runtime bridge)

`EvalProg<Prog>` adapts a program value into a runtime `OpOnce` step by compiling
it on-demand under the argument type. This lets runtime engines (like HList fold
machinery) accept program-defined steps without introducing a separate backend.

**Cost note**: `EvalProg` introduces inner compilation. Keep step programs small.

---

## Where to look

- `compile.rs`: pipeline entrypoint (`Compile`, `compile`)
- `lower/*`: lowering routers + tables
- `reify/*`: reify routers + tables
- `table.rs`: closed-table plumbing

