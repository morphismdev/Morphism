## HList (tuple-based product plumbing)

This module provides the showcase's **product-shape runtime and program nodes**.
It treats Rust tuples as an "HList encoding" so we can build generic operations
(map, fold, indexing) over heterogeneous products.

This is a pragmatic encoding designed to work with Rust's tuple trait limits and
keep compile times sane.

---

## Fixed-arity encoding (the core constraint)

We do not use a recursive list type. Instead we encode long products using
a chunked tuple strategy:

- If the "list length" is `<= 11`: it is just a plain tuple `(E0, E1, ..)`.
- If the length is `> 11`: it is encoded as:

`(E0, E1, .., E10, Tail)`

where `Tail` is another tuple using the same rule.

You will see this show up as:
- "arity < 12 => Tail = NullaryToken"
- "arity == 12 => (11 head, Tail)"

This is the format used across `map`, `push_back`, `get_at`, and fold pipelines.

---

## Program nodes vs runtime ops

This folder contains both:
- **Program nodes**: `Tagged<IX_*, Payload>` terms (AST) that are compiled through the closed compiler
- **Runtime ops**: `OpOnce` implementations that execute directly

Examples:
- `HlistMapProg<StepProg>` is an AST node (a kernel runtime-bridge key).
- `HlistMap<Op>` is the runtime implementation used after reify.

---

## What lives where

- `map.rs`
  - `HlistMapProg` / `hlist_map_prog` (AST node)
  - `HlistMap<Op>` (runtime map, implemented via the fold_l pipeline)

- `hlist_push_back.rs`
  - `HlistPushBackProg` / `hlist_push_back_prog` (AST node)
  - `HlistPushBack` (runtime op)

- `get_at/`
  - "get element at index" utilities for tuples and the chunked HList encoding
  - includes notes about the flattened representation and boundary behavior

- `fold/`
  - balanced fold pipeline implementation
  - see `fold/README.md` for the full explanation and cost model

---

## Why this exists in the showcase

The main "real" motivation is tuple ergonomics:
users can work with tuples directly, while the system internally treats them as
heterogeneous product shapes with a predictable encoding and a closed compiler path.

