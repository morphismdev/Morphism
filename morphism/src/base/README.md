## `base` Module

### Purpose

`base` is the primitive-domains layer of the **showcase crate**.

It provides a small set of foundational domains (boolean, numeric, order) plus a tiny arithmetic layer that the rest
of the crate builds on.

The morphism kernel traits (`OpTy`, `OpOnce`) and most program-construction utilities live in `kit/`.

This module is intended to be *boring and durable*: it changes slowly and only contains primitives that are actively used.

### Scope

#### Owns

- Primitive domains used pervasively:
  - Boolean domain: `True`, `False` plus boolean ops + elimination.
  - Numeric domain: type-level naturals (`UTerm` / `UInt<B, T>`), common aliases (`U0..U64`), and numeric predicates.
  - Order domain: equality and relational comparisons for naturals.
- Core arithmetic over naturals:
  - `Add<Rhs>` and `Sub<Rhs>` (saturating subtraction).

#### Does not contain

- Morphism kernel traits (`OpTy`, `OpOnce`) — those live in `kit/op.rs`.
- Program construction tools (combinators, tokens, registry, tagging) — those live in `kit/`.
- Compiler behavior (lowering, reification, dispatch).
- Data-structure/container machinery (`hlist/`, `container/`, etc).
- "Maybe useful later" helpers that are not used.

### Contents

`base/` is organized into small domain modules:

- **Domain modules**:
  - `boolean/`: reflectable boolean values (`True`, `False`) + boolean ops (`ops.rs`) + elimination (`elim.rs`).
  - `num/`: natural number representation (`nat.rs`), small aliases (`aliases.rs`), numeric predicates (`predicates.rs`).
  - `order/`: semantics-only comparisons for naturals (no reflected program nodes).
- **Arithmetic module**:
  - `arithmetic/`: type-level arithmetic ops (`Add`, `Sub`) over `base::num` naturals.

- `mod.rs`: re-exports a curated "base prelude" (`base::*`) for convenient access to these primitives.

Reading entrypoints:
- For nat syntax and notation: `base/num/README.md`.
- For arithmetic ops: `base/arithmetic/README.md`.
- For boolean elimination/control-flow primitives: `base/boolean/README.md`.

### Invariants

- **Small surface area**: `base` should remain easy to scan and hard to bloat.
- **High reuse**: anything added to `base` must be used by multiple subsystems (or be a direct primitive representation required by existing code).
- **No speculative utilities**: if it isn't used yet, it doesn't belong here.
- **Domain focus**: `base` focuses on primitive domain semantics, not program construction machinery.
- **Primitive domains only**: domain-specific "application DSLs" do not belong here.

### Module-specific rules

- When a feature is "higher-order" (e.g. predicate combinators), it does not enter `base` until there is a concrete consumer and a clear home.
- Prefer adding new functionality outside `base` first; promote into `base` only when it proves foundational.
