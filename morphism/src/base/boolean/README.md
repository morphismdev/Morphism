## `base::boolean` Module

### Purpose

Defines the boolean domain for the **showcase crate**: reflectable boolean values (`True`, `False`) and a minimal set
of boolean operations implemented as `OpTy` morphisms.

This module is intentionally small and stable; it exists to support the rest of the showcase DSL with the most basic
boolean building blocks.

### Scope

#### Owns

- Boolean domain syntax (reflectable values):
  - `True`, `False`
- Boolean operations (type-level morphisms):
  - `Not`
  - `And<Rhs>`
  - `Or<Rhs>`
  - `Xor<Rhs>`
  - `AndNot<Rhs>` (A & !B shape)
- Boolean elimination primitives:
  - `Select`, `If`, `IfApply`, `IfConst`, `AssertTrue`

#### Does not contain

- Higher-order predicate combinators unless there is a concrete consumer.
- Runtime/value-level boolean evaluation (`OpOnce`) unless it becomes necessary.
- Compiler behavior (lowering/reification/dispatch).
- Reflected program nodes for boolean ops: boolean **ops** remain unreflected semantics only.

### Contents

- `mod.rs`: boolean domain syntax (`True`, `False`) + re-exports of `ops` and `elim`.
- `ops.rs`: boolean operations (`Not`, `And`, `Or`, `Xor`, `AndNot`).
- `elim.rs`: boolean elimination (`Select`, `If`, `IfApply`, `IfConst`, `AssertTrue`).

### Invariants

- `True` and `False` remain reflectable domain values via tagging.
- Ops are pure type-level morphisms (`OpTy`) and are grouped in `ops.rs`.
- Boolean operations are **unreflected** (no registry keys or program nodes).
- Keep the surface area small and used.

### Module-specific rules

- New boolean ops go into `ops.rs`.
- Higher-order logic belongs outside this module unless it is proven foundational.
