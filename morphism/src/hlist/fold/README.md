# HList fold pipeline (current implementation)

This folder implements **balanced-spine folds** over the showcase crate's fixed-arity "HList tuple encoding".

At a high level, the current implementation (`fold_l`) follows the pipeline:

1. **Map elements → segment transformers** (linear)
2. **Pairwise compose transformers** (one "layer")
3. **Repeat pairwise composition until one transformer remains** (balanced, \(O(\log n)\) layers)
4. **Apply final transformer to the initial accumulator**

> Note: `fold_r` is not implemented in `morphism` yet. If/when it's added, it will mirror this pipeline with reversed step ordering (`(Elem, Acc) -> AccNext` instead of `(Acc, Elem) -> AccNext`).

---

## Fixed-arity HList encoding (important constraint)

This implementation does **not** use a true recursive list type. Instead it uses a *tuple-based encoding*:

- If the “list length” is \(\le 11\): it’s just a plain tuple `(E0, E1, ..)`.
- If the length is \(> 11\): it’s encoded as:

`(E0, E1, .., E10, Tail)`

where `Tail` is another tuple representing the remaining elements (again using the same rule).

This is why the code treats “arity == 12” as “11-head + tail”.

---

## Pipeline stages (where the code lives)

### 1) Map elements → segment transformers (**linear**)

- `fold_l/pipeline/map_to_segments.rs`: `HlistToSegments<F>` producing `SegmentFoldL<F, (Elem,)>`

Each element `x` becomes a transformer `Acc -> AccNext`:

- fold-left: `SegmentFoldL::new(f, (x,))`

**Important limitation:** this mapping stage is linear in the number of elements (and chunks). It does not provide “infinite arity for free”: you still construct a large nested tuple value/type, and the mapper walks it linearly.

**Why the depth optimizations exist:** the pipeline includes specialized steps (e.g. consuming two chunks per recursion step) to reduce trait recursion depth and keep the demo usable under practical `recursion_limit` settings. These optimizations preserve the same fixed-arity encoding while improving headroom for future compiler passes and structural queries.

**Depth optimization (2-chunk step):** for long hlists encoded as nested `11-head + tail` tuples,
the mapper has a specialized “2-chunk” step for the shape:

`(A0..A10, (B0..B10, Tail2))`

In that case it consumes **22 elements per recursion step** (two full chunks) and produces a
structure-preserving output:

`(Seg(A0)..Seg(A10), (Seg(B0)..Seg(B10), map(Tail2)))`

This keeps the same fixed-arity hlist encoding while reducing trait recursion depth in the
mapping stage by ~2×.

### 2) Pairwise compose ("one layer")

- `fold_l/pipeline/compose_pairs.rs`: `TupleComposePairs` + `HlistComposePairs`

This converts a sequence:

`(T0, T1, T2, T3, ...)`

into:

`(pair(T0,T1), pair(T2,T3), ...)` (carry last if odd)

The `HlistComposePairs` form also knows how to pair **across the chunk boundary**:
it will build the boundary pair from the last head element and the first tail element.

### 3) Balanced composition ("repeat until 1")

- `fold_l/pipeline/compose_balanced.rs`: `HlistComposeBalanced`

This repeatedly applies pairwise layers, yielding **log-depth** parenthesization (much smaller types than a naive left-associated chain).

**Depth optimization (2-layer step for chunked tails):** for long hlists with a chunked tail
`(A0..A10, (B0..B10, Tail2))`, `HlistComposeBalanced` performs **two pairwise layers per recursion
step** (i.e. `pairs(pairs(x))` before recursing) to reduce the amount of type-level recursion
needed to “drain” the nested chunk encoding.

### 4) Apply to accumulator

The final “apply” step is the generic op:

- `ApplyOp` combinator (from `kit/combinators/apply.rs`) with behavior `(Op, Arg) -> Op.run(Arg)`

---

## Worked example: 20 elements (shows the balancing)

We’ll trace **structure**, not concrete numeric behavior.

Assume we have 20 elements:

`e0, e1, ..., e19`

### Step A: Encode as the fixed-arity HList tuple

20 elements become:

`(e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, (e11, e12, e13, e14, e15, e16, e17, e18, e19))`

That’s “11-head + 9-tail”.

### Step B: Map each element to a segment transformer (linear)

After mapping, we have transformers:

`S0, S1, ..., S19`

where each `Si` is:

- fold-left: `Si = SegmentFoldL::new(f, (ei,))`

### Step C: First pairwise layer (includes the head/tail boundary)

Pairing adjacent elements in order (and pairing across the boundary `(S10, S11)`):

- `P0  = pair(S0,  S1)`
- `P1  = pair(S2,  S3)`
- `P2  = pair(S4,  S5)`
- `P3  = pair(S6,  S7)`
- `P4  = pair(S8,  S9)`
- `P5  = pair(S10, S11)`  ← boundary pair (head + first tail)
- `P6  = pair(S12, S13)`
- `P7  = pair(S14, S15)`
- `P8  = pair(S16, S17)`
- `P9  = pair(S18, S19)`

Now we have **10 transformers**: `(P0, P1, ..., P9)` (no tail needed anymore since 10 ≤ 11).

### Step D: Keep pairing until one transformer remains (balanced)

Layer 2 (10 → 5):

- `Q0 = pair(P0, P1)`
- `Q1 = pair(P2, P3)`
- `Q2 = pair(P4, P5)`
- `Q3 = pair(P6, P7)`
- `Q4 = pair(P8, P9)`

Layer 3 (5 → 3, carries the last):

- `R0 = pair(Q0, Q1)`
- `R1 = pair(Q2, Q3)`
- `R2 = Q4` (carried)

Layer 4 (3 → 2, carries the last):

- `U0 = pair(R0, R1)`
- `U1 = R2` (carried)

Layer 5 (2 → 1):

- `Final = pair(U0, U1)`

So the overall “shape” is a balanced tree built out of repeated adjacent pairing.

### What is `pair(·,·)` exactly?

For fold-left: `pair(A,B) = ThenOp::new(A, B)`

> (If `fold_r` is added later, it will use mirrored composition order: `pair(A,B) = ThenOp::new(B, A)`.)

