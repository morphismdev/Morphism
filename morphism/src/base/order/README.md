## `base::order` Module

### Purpose

`base::order` defines comparison operations for type-level naturals in the **showcase crate**.
It provides runnable semantic operations (`OpTy` implementations) for equality and relational comparisons.

This module is foundational: comparisons are used for type-level control flow and numeric reasoning across the showcase.

### Scope

#### Owns

- **Comparison semantics** (runnable ops):
  - `equality.rs`: `NumEqOp<Rhs>` (equality)
  - `relational.rs`: `LeOp<Rhs>`, `LtOp<Rhs>`, `GeOp<Rhs>`, `GtOp<Rhs>`
  - `cmp.rs`: internal comparison machinery for natural numbers (`CmpBit`, `CmpNat`)

#### Does not contain

- Compiler behavior.
- Registry allocation (`IX_*` keys) or reflected program nodes.
- Numeric representations (those live in `base::num`).
- Boolean elimination (those live in `base::boolean`).

### Contents

- `mod.rs`: re-exports of semantic modules.
- `equality.rs`: equality comparison operations (`NumEqOp`).
- `relational.rs`: relational comparison operations (`LeOp`, `LtOp`, `GeOp`, `GtOp`).
- `cmp.rs`: internal comparison algorithms for natural numbers (`CmpBit`, `CmpNat`).

### Invariants

- **Order operations are unreflected**: all comparison operations are pure semantic ops (`OpTy`).
- **Semantics in sibling files**: runnable ops are organized by category.
- **Natural number focus**: internal machinery focuses on natural number comparisons.

### Module-specific rules

- Add new comparison ops as `OpTy` implementations in the appropriate semantic file.
- Keep internal comparison algorithms in `cmp.rs` unless a public API is required.
