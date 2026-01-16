# Flat vs encoded indexing for HList-encoded tuples

This directory contains tuple/HList "get-at" primitives used by multiple subsystems.
There are **two different (and intentional) indexing semantics**:

## 1) Encoded indexing (`HlistGetAt`)

`HlistGetAt<Ix>` indexes into the **chunked tuple encoding** directly.

For arity > 11, the encoding is:

- `(E0, .., E10, Tail)`

Where the **12th field** (`Ix == U11`) is the *tail pointer slot*.

So:

- `Ix < U11` selects an element from the head chunk (`U0..U10`)
- `Ix == U11` selects the `Tail` value itself
- `Ix > U11` recurses into the tail at `Ix - U12`

This is the semantics used by representation-level code that needs direct access to the chunk boundary.

## 2) Flat (logical) indexing (`HlistFlatGetAt`)

`HlistFlatGetAt<Ix>` treats the same representation as a **single flat logical list**.

For arity > 11:

- `Ix < U11` selects from the head chunk (`U0..U10`)
- `Ix >= U11` recurses into the tail at `Ix - U11`

Notably:

- `Ix == U11` selects the **first element inside the tail** (i.e. the 12th element overall)
- the tail pointer slot is **never** exposed as an element

This is the semantics that "feels like normal indexing" for large tuples.

## Why we keep both

Some structural recursion patterns over the chunked encoding need to treat the tail as a *whole sub-structure*
(i.e. "get the tail tuple/table and recurse"), which requires being able to select the **tail pointer slot**.

So we keep both:

- `HlistGetAt`: representation/encoding primitive (tail-aware)
- `HlistFlatGetAt`: user-facing/semantic primitive ("flat" indexing)


