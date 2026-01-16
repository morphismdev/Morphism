## Container (Rust ↔ DSL bridge)

This module is the **bridge between Rust values** and the DSL’s program world.

It does two things:

- Defines how a Rust "host type" participates in the DSL (via `IntoContainer` / `FromContainer`)
- Provides `Container<T, Prog>`, a lazy builder that stores an input value plus a program pipeline, and can compile+run it through the closed compiler

---

## Core idea: host ↔ container isomorphism

A host type `T` is viewed as a **tagged node** with:

- a **shape tag**: `T::Tag`
- a **payload**: `T::Payload`

But *the public bridge type is not `Tagged`* — the bridge type is:

- `IntoContainer`: `T -> Container<T, Id>`

Inside `Container`, the host is represented as a `Tagged<T::Tag, T::Payload>` plus the current program.

This gives you:
- a stable "host-facing" API (`T`, `Container<T, _>`)
- an explicit internal representation (`Tagged<Tag, Payload>`)
- a place to attach program pipelines (`Prog`)

---

## The three key pieces

### 1) `IntoContainer`
`IntoContainer` defines how to convert a concrete Rust value into a container builder:

- `T -> Container<T, Id>`

The resulting `Container` holds the tagged representation internally, and starts with the identity program.

### 2) `FromContainer`
`FromContainer` defines how to rebuild the host wrapper when the payload type changes.

This is used by `Container::collect`, which:
- compiles+runs the current program
- produces `Tagged<T::Tag, OutPayload>`
- reconstructs the host via `T::from_container(out_payload)` into `T::Rewrap<OutPayload>`

### 3) `Container<T, Prog>`
`Container` is a lazy pipeline builder:

- stores input as `Tagged<T::Tag, T::Payload>`
- stores a program `Prog` (default is identity)
- supports pipeline composition (`then`)
- supports running (`run`)
- supports collecting back into the host (`collect`, `collect_into`, `reinterpret_into`)

It also provides convenience program stages like:
- `map(op)` (map over children)
- `fold_children_l(op, acc)` (fold over children)

---

## Tuple bridge (why tuples "just work" in the showcase)

`std_tuple_bridge` implements the container isomorphism for Rust tuples by treating them as **HList nodes**:

- tuples implement `IntoContainer<Tag = IX_HLISTNODE, Payload = Tuple>`
- tuples implement `FromContainer` where `Rewrap<NewPayload> = NewPayload`

That "rewrap = payload" rule means: when a tuple participates in a program, we treat the tuple as a *structural product shape*, and `collect` returns the new payload (often another tuple).

---

## Mental model

- `IntoContainer` says: "this Rust value has a DSL shape and payload"
- `Container` says: "here is that value, plus a program to run on it"
- the compiler says: "given `Input = Tagged<Tag, Payload>`, compile `Prog` into a runnable `OpOnce<Input>`"
- `FromContainer` says: "take the output payload and rebuild the Rust wrapper"

This is the main "bridge surface" that lets user-space types hook into the system.

