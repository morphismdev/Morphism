## `kit` Module

### Purpose

`kit` is the **toolbox layer** of the showcase crate.

It contains the minimal vocabulary for building and manipulating morphisms in two "modes":

- **Semantics**: runnable operations, expressed as `OpTy` (type-level) and `OpOnce` (value-level).
- **Syntax (reflected programs)**: reusable, tag-identified program nodes, expressed as `Tagged<IX_*, Payload>`.

In short: `base/` defines primitive *domains* (bool/num/order/arithmetic), while `kit/` defines the *machinery* used
to build programs and run/interpret them.

### Scope

#### Owns

- **Kernel operation traits**:
  - `OpTy`, `OpOnce`, and their `Out` aliases (`OpTyOut`, `OpOnceOut`)
- **Tagging primitives for reflected programs**:
  - `Domain`, `Key`, `Tagged`
- **Syntax tokens / payload markers**:
  - `NullaryToken`, `UnitToken`
- **Core combinators** (each typically has both a reflected program node and a runnable op):
  - composition (`then` / `ThenOp`)
  - identity (`id` / `IdOp`)
  - constants (`const_move`, `ConstCOp`, `ConstMOp`)
  - tuple ops (`fst`, `snd`, `fanout`, `bimap`)
  - application (`apply` / `ApplyOp`)
  - partial application (`partial_l`, `partial_r`)
  - tagged helpers (`wrap_tagged`, `unwrap_tagged`, `TagWith`, `ReTag`)
- **Registry**:
  - the single source of truth for domain codes and keys (`D_*`, `R_*`, `IX_*`)
  - guard tests that keep the key-space clean
- **Bridges and introspection**:
  - `op_lift(op)`: bridge from user-provided runnable ops to reflected syntax
  - `KeyOf`, `PayloadOf`: projections over `Tagged`

#### Does not contain

- Primitive domains (those live in `base/`).
- Compiler behavior (lowering/reification/dispatch), except for the minimal registry/key-space needed by compilation.
- Large "application DSL" layers; this crate keeps `kit` small and composable.

### Contents

- `op.rs`: morphism kernel traits (`OpTy`, `OpOnce`) and `Out` aliases.
- `tag.rs`: tagging primitives (`Domain`, `Key`, `Tagged`) used to represent reflected program nodes.
- `tokens.rs`: tiny payload markers (`NullaryToken`, `UnitToken`).
- `combinators/`: core building-block combinators (syntax nodes + runnable semantics).
- `registry/`: identity allocation table (`D_*`, `R_*`, `IX_*`) + guard tests.
- `op_lift.rs`: `op_lift(op)` bridge into reflected syntax (`OpLift<Op>`).
- `introspect.rs`: pure projection ops over `Tagged` (`KeyOf`, `PayloadOf`).

### The two layers: semantics vs syntax

A recurring pattern in `kit` is that a concept exists in both layers:

- **Syntax**: a reflected program node is a `Tagged<IX_*, Payload>`.
  - Example: `Then<F, G> = Tagged<IX_THEN, (F, G)>`
- **Semantics**: a runnable op implements `OpTy` and/or `OpOnce`.
  - Example: `ThenOp<F, G>` implements composition.

This split is intentional:
- Syntax nodes are used to build "programs" that can be lowered/reified/compiled.
- Semantic ops are used for direct evaluation and for defining domain behavior.

### Invariants

- **Keep `kit` dependency-light**: it should remain the stable foundation for the crate.
- **Keys are explicit**: every reflected node uses an `IX_*` key from `kit/registry/`.
- **Registry stays clean**: no dead keys, no ghost owners (enforced by unit tests in `kit/registry/mod.rs`).
- **Small, composable combinators**: prefer a tiny set of orthogonal primitives over a sprawling DSL.

### Module-specific rules

- If something is "just an op" (pure semantics), it does not need a reflected key.
- If something is reflected as `Tagged<IX_*, ...>`, it must have a corresponding `IX_*` in the registry.
- Don't add keys "just in case"; keys are a commitment to identity and discoverability.

