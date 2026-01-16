## `registry` Module

### Purpose

The `registry` module is the single source of truth for type-level identity allocation in the **showcase crate**.
It defines domain codes (`D_*`), role codes (`R_*`), and keys (`IX_*`) used to tag reflected program nodes and to drive dispatch.

### Scope

#### Owns

- Domain codes: `D_*`
- Role codes: `R_*` (currently `R_SYNTAX`, `R_SEMANTICS`)
- Domain types: `*Domain = Domain<D_*>`
- Keys: `IX_* = Key<*Domain, R_*, U*>`
- Collision guards that detect domain code/key collisions

#### Does not contain

- Semantic implementations (ops) for the operations identified by keys
- Compiler logic (lowering, reification)
- Pattern matching / case analysis logic
- Any code architecture mirroring (the registry does not encode the crate's internal module layout)

### Contents

- `mod.rs`
  - Re-exports `codes`, `roles`, and all domain files.
  - Contains collision guard tests and registry hygiene tests.
- `codes.rs`
  - Defines all domain codes (`D_*`).
- `roles.rs`
  - Defines all role codes (`R_*`).
- Domain key files (flat list; one file per domain or grouping):
  - `bool.rs`: boolean syntax keys (`IX_TRUE`, `IX_FALSE`)
  - `combinators.rs`: combinator semantics keys (`IX_ID`, `IX_THEN`, `IX_PARTIAL_L`, `IX_PARTIAL_R`, `IX_FST`, `IX_SND`, `IX_FANOUT`, `IX_CONST_MOVE`, `IX_BIMAP`, `IX_APPLY`)
  - `generic.rs`: generic operations (`IX_MAP_NEWTYPENODE`, `IX_MAP_HLISTNODE`, `IX_MAP_CHILDREN`, `IX_FOLD_NEWTYPENODE_L`, `IX_FOLD_CHILDREN_L`, `IX_FOLD_HLISTNODE_L`)
  - `hlist.rs`: hlist syntax and semantics (`IX_HLISTNODE`, `IX_HLIST_PUSH_BACK`)
  - `hlist_fold.rs`: hlist fold pipeline primitives (`IX_HLIST_TO_SEGMENTS_L`, `IX_HLIST_COMPOSE_BALANCED_L`, `IX_HLIST_MAP`)
  - `new_type_node.rs`: newtype node syntax (`IX_NEWTYPENODE`)
  - `op.rs`: operation lift bridge (`IX_OP_LIFT`)
  - `tagged.rs`: tagged type operations (`IX_WRAP_TAGGED`, `IX_UNWRAP_TAGGED`)

### Invariants

- **Domain codes are globally unique and contiguous**: each `D_*` in `codes.rs` is unique and assigned to a contiguous block starting from `U0`.
- **Role codes are defined in `roles.rs`**: currently `R_SYNTAX` (U0) and `R_SEMANTICS` (U1).
- **Keys are globally unique**: `IX_*` types must not collide across `(Domain, Role)` pairs.
- **Contiguous indices**: within each `(Domain, Role)` pair, key indices are contiguous starting from `U0` and must not contain gaps.
- **Index ordering**: keys within each domain file are declared in strict increasing index order per role to support fast scanning and discovery.
- **Role semantics**: 
  - `R_SYNTAX`: structural AST nodes (e.g., `IX_HLISTNODE`, `IX_NEWTYPENODE`, `IX_TRUE`, `IX_FALSE`)
  - `R_SEMANTICS`: operations/ops (e.g., combinators, generic ops, `IX_OP_LIFT`)
- **Flat structure**: this module remains flat to support fast discovery of domain codes and keys.
- **Architecture-independent**: the registry does not mirror internal code architecture and is not reorganized to reflect internal refactors.
- **No dead keys (enforced)**: every `IX_*` declared in `kit/registry/` must be referenced somewhere outside `kit/registry/` (program owner, compiler dispatch, etc). This is enforced by a unit test in `kit/registry/mod.rs`.
- **No ghost owners (enforced)**: every `Tagged<IX_EXAMPLE, ...>` program owner must have a corresponding `IX_EXAMPLE` key declared in `kit/registry/`. This is enforced by a unit test in `kit/registry/mod.rs`.

### Enforced guard tests (how they work)

The following checks run in `registry/mod.rs` under `#[cfg(test)]`:

- **Dead key check** (`no_dead_registry_keys`)
  - Walk `src/kit/registry/**/*.rs` and collect all declared keys by scanning for lines starting with `pub type IX_`.
  - Walk `src/**/*.rs` and `tests/**/*.rs`, excluding `src/kit/registry/`.
  - For each collected `IX_*`, scan those non-registry files for an identifier-token occurrence.
  - Fail if any key is never referenced outside `kit/registry/`.

- **Ghost owner check** (`all_tagged_ix_owners_have_registry_keys`)
  - Walk `src/**/*.rs` and `tests/**/*.rs`, excluding `src/kit/registry/`.
  - Scan for `Tagged<IX_...` and `Tagged::<IX_...` occurrences and collect the `IX_...` tokens.
  - Fail if any such `IX_*` is missing from `src/kit/registry/`.

These tests are intentionally lightweight lexical scans (not a Rust parser). They are designed to match
the crate's conventions and keep the registry clean during rapid refactors.

### Module-specific rules

- The file structure is engineered for fast discovery of domain codes and keys.
- Domains and keys are named and grouped to support lookup, not to match the rest of the crate structure.
- When updating indices, remove sparsity and enforce contiguous indices within each `(Domain, Role)` pair.

### File Styling

Registry domain files use mandatory separator banners to organize keys.

**Tier 1 separators (`*`): Role sections**

Every file must contain Tier 1 banners for each role it defines:
- Files with `R_SYNTAX` keys must have a Tier 1 banner for the syntax section.
- Files with `R_SEMANTICS` keys must have a Tier 1 banner for the semantics section.

Format:
```rust
// ****************************************************************************
// Syntax keys (R_SYNTAX): <description>
// ****************************************************************************
```

```rust
// ****************************************************************************
// Semantics keys (R_SEMANTICS): <description>
// ****************************************************************************
```

**Tier 2 separators (`─`): Subgroups within a role**

Every Tier 1 section must contain one or more Tier 2 banners to group related keys.

Format:
```rust
// ─────────────────────────────────────────────────────────────────────────────
// <Subgroup description>
// ─────────────────────────────────────────────────────────────────────────────
```

**Index policy**

Within each `(Domain, Role)` pair, indices are contiguous starting from `U0` and incrementing by 1. No gaps are allowed.
