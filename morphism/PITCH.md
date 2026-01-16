## Morphism — Pitch (working draft)

Morphism is an experimental generic-programming project for Rust.
This repository contains **`morphism`**: a small, closed-world snapshot that demonstrates the core idea (reflected programs + lowering + reify into runnable ops).

---

## What Morphism is

Morphism is an **experimental generic programming framework** for Rust.

It provides:
- a reflected DSL for describing structure-aware transformations
- a closed compilation pipeline (lowering → kernel → reify) that turns programs into runnable operations
- an architecture that aims to keep compilation/execution behavior predictable under heavy generic usage

### Current status (this snapshot)

**Today, `morphism` supports:**
- product shapes (Rust tuples treated as an HList encoding)
- mapping over immediate children
- fold-left over immediate children
- a closed-world compiler (unsupported program keys fail to type-check)
- an "opaque/atomic" boundary (`NewTypeNode`) to opt out of structural traversal

**Today, `morphism` does not support (yet):**
- sum types / enums (no coproduct story in this crate)
- `dyn` / trait-object driven APIs
- recursive traversals over deep recursive host structures ("traversable containers")
- fold-right (`fold_r`) in the HList fold pipeline

---

## Motivation

Generic programming addresses recurring pain points in the Rust ecosystem:

- expressing transformations over *structure* (not just over a single type parameter)
- writing reusable logic over products/sums without duplicating implementations
- bridging “nice user-facing types” to a uniform internal representation
- scaling generic power without losing compile-time predictability or debuggability

Today there are many partial approaches (macros, derives, codegen, trait patterns, bespoke frameworks). What’s missing is a cohesive "all-in" foundation that tries to solve the problem as a system.

Morphism treats **generic programming** as a first‑class design target: build structure-aware programs that remain explicit, composable, and inspectable.

---

## What “all-in” means (and why it matters)

“All‑in” here means **low‑level, end‑to‑end, and standalone**:

- a DSL for generic programs (not just a set of traits)
- explicit separation of:
  - **syntax** (reflected program terms)
  - **semantics** (runnable operations)
- a real compiler pipeline:
  - **lowering** from higher-level terms into a smaller kernel
  - **reify** into runnable operations
- a commitment to maintainability as a first-order concern

This is not a typical Rust approach. The reason is practical: the moment you attempt a low-level end-to-end design, Rust’s missing pieces become *impossible to ignore*—and that impacts:
- how “complete” the framework can feel at the bottom layer
- how much **cognitive load** the design imposes on maintainers
- what sort of ergonomics you can realistically offer without language support (e.g., variadic generics)

Going end-to-end is useful because it forces clarity:
- where the language is strong
- where the model fits naturally
- where the constraints are fundamental vs incidental
- and what “language features” would actually buy us something measurable

---

## Key ideas (current mental model)

The current project is organized around:

- **Structure as a programmable object**
  - Representing structure in a way that can be transformed and annotated
- **Reflected programs**
  - Programs are values (terms) you can compose before compiling
- **Lowering and kernelization**
  - Move from rich frontend terms → smaller kernel terms with well-defined meaning
- **Multiple interpretations**
  - The same structure/program can generate different “meanings” (executions/interpretations)
- **Predictability**
  - Prefer closed or bounded compilation surfaces when possible to keep behavior debuggable

(Concrete mechanics live in code; this section is just the conceptual frame.)

---

## Scope

This project is intentionally scoped to **`Sized` types** for the foreseeable future.
There is no `dyn` / trait-object story yet, and that is a conscious choice while the core model stabilizes.

### In scope

- **Structure-aware programming over `Sized` types**
- A low-level, explicit model for generic programs (syntax + compiler + reify)
- **Iterators as applicatives**, not as traversable containers (for now)
  - target shape:
    `(iterable1, iterable2, ...).into_iter().map(...)`
    without macro expansion
- Better control over:
  - type-level error messages (where possible)
  - compile-time “timing / cost” visibility and predictability
- Practical groundwork for areas like:
  - variadic generics-style abstractions (and a concrete argument for what’s missing)
  - SoA (structure-of-arrays) collections
  - codegen-adjacent systems that need structural transforms

### Out of scope (for now)

- `dyn`/trait-object based APIs
- iterator traversal over recursive structures (“iterator of structure” / traversable containers)
- TCO-style “stack-safe recursion APIs” for deep recursive structures

### Arity expectations (tuples / HLists)

Even though the current plumbing can stress **large arities** (e.g. deep tuple tests), the near-term focus is **small, practical tuple arities**—what Rust tuples are meant to be used for in real programs.

Large-arity stress tests serve a specific purpose: they measure **headroom** while we adjust the lowering boundary.

In practice, the supported arity is a **moving ceiling**:
- the more obligations we push into the kernel (more “fully lowered” behavior),
  the more compile-time pressure we introduce → **lower maximum arity**
- keeping some building blocks opaque can preserve arity ceilings → **higher stress ceilings**

Example (illustrative): fully lowering the fold pipeline into kernel combinators can drop usable arity from ~1288 down to ~500. This is expected: the design prioritizes **maintainable lowering** over extreme arity.

---

## Why this isn’t “just traits” (design pressure)

A purely trait-based approach can work for many local problems, but it tends to break down when you want:
- reflected programs as values
- compilation stages (lowering boundaries)
- predictable “closed surfaces”
- multiple interpretations of the same structure/program
- scalable reuse across products/sums/newtypes without macro explosion

The point of Morphism is to build a system where those concepts are *native*, not bolted on.

---

## Roadmap (near term)

This crate is a **showcase snapshot**. These items reflect the next steps on the main branch (not all are present here yet).

### Next
- Sum types support (not in this showcase yet)
- Migrate/consolidate tests; publish a coherent module DAG
- Centralize pattern-matching / case analysis machinery

### After that
- Stabilize core frontends (e.g. `Mappable`)
- Introduce `OpRef` / `OpMut` (beyond `OpOnce`)
- Add round-trip examples: `T -> into_container -> transform -> collect -> T`
- Add derive support (reduce boilerplate, especially tuple-related)
- Write guides/tutorials
---
