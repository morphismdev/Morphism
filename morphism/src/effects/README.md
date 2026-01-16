## Effects (high-level programs over children)

This module defines **program constructors** for "do something to a node's immediate children".

These are **syntax terms** (AST nodes) represented as `Tagged<IX_*, Payload>`. They are not runtime APIs.
They exist to be composed into a program and then compiled through the closed compiler.

Most users won't import this module directly — they'll use `Container` / `api::*` helpers which build these
programs under the hood.

---

## What "effects" means here

An "effect" is a **structural operation on children**, such as:

- map a step over each child
- fold-left over children

This is the layer where those operations are represented as **first-class program values**.

---

## The main programs

### 1) `MapChildrenProg` (dispatcher)

`MapChildrenProg<OpLift>` represents: "map this step over the children".

- It is a **dispatcher**: during closed lowering, it selects the correct lowering based on the input shape
  (e.g. `NewTypeNode` vs `HlistNode`).

### 2) `FoldChildrenLProg` (dispatcher)

`FoldChildrenLProg<FProg, Acc>` represents: "fold-left over children using step program `FProg`, starting at `Acc`".

- Also a **dispatcher**: lowering chooses the correct implementation based on input shape.

---

## Wrapper-aware `HlistNode` operations

The `effects::hlist_node` submodule provides "wrapper-aware" variants for `HlistNode<Children>`:

- **In** variants: unwrap → transform children → wrap
  - preserves the `HlistNode<...>` wrapper
- **Into** variants: unwrap → transform children → return raw result
  - eliminates the wrapper (returns the accumulator/result directly)

This distinction exists because sometimes you want to preserve the node wrapper as part of the host type,
and sometimes you want to consume it and return a plain value.

---

## Where these programs are implemented

- These `Tagged<IX_*, _>` programs must **lower away** into kernel programs before reify.
- Dispatch and specialization are handled in the closed lowering tables (see `compiler/lower/*`).

Rule of thumb:
- If you see a program key in `effects/`, it's a **frontend syntax term**.
- If you see a key in `compiler/reify/kernel/*`, it's in the **kernel** and can reify directly.

---

## How this connects to the user API

- `Container::map(op)` builds a program using these constructors (after lifting `op` into syntax via `OpLift`)
- `Container::fold_children_l(op, acc)` builds a fold program similarly
- `api::MappablePlan` / `api::Foldable` provide Rust-first convenience wrappers over the same machinery

