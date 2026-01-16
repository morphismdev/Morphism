## NewType (opaque / one-shot nodes)

This module defines the **opaque (atomic) node** used by the generic machinery.

By default, product shapes (notably Rust tuples viewed as HLists) are treated as
**structural nodes**: operations like "map over children" run *per element*.

`NewTypeNode<T>` is the opt-out: it forces `T` to be treated as **one unit**.

---

## What problem this solves

Sometimes you have a composite value (like a tuple) that you want to participate
in the algebra as a **single element**, not as a collection of children.

Examples:
- treat a tuple as one field/value
- prevent element-wise mapping/folding
- establish an intentional "atomic boundary" inside a larger structure

---

## What `NewTypeNode` means

`NewTypeNode<T>` represents an **opaque/atomic value** in the generic representation.

- It has a shape tag (`IX_NEWTYPENODE`)
- Its payload is the entire value `T`
- It has **no children** as far as "children traversal" is concerned

---

## One-shot behavior

### Mapping
Mapping over a `NewTypeNode<T>` applies the operation to the **whole payload** `T`,
not to any internal structure inside `T`.

In other words: wrapping turns "element-wise map" into "one-shot map".

### Folding over children
Folding over children of a `NewTypeNode<_>` returns the accumulator unchanged,
because an opaque node has **zero children**.

This is the "atomic boundary" behavior: traversal does not enter the payload.

---

## Program constructors in this module

This module exposes program-level (AST) constructors that lower through the closed compiler:

- `MapNewTypeNodeProg`: map over the inner payload (one-shot)
- `FoldNewTypeNodeLProg`: fold over children (no-op; returns `acc`)

These are syntax terms (`Tagged<IX_*, Payload>`) and are meant to be composed into programs.
Most user-facing APIs build these programs for you.

---

## How users usually trigger this

In the showcase examples, a host wrapper can implement `IntoContainer` with
`Tag = IX_NEWTYPENODE` to force its payload to be treated as atomic.

That wrapper is the "escape hatch" when the default behavior (tuples as structural
products) is not what you want.

