## `morphism`

A pre-release **showcase snapshot** of a Rust generic-programming DSL.

For deeper context, goals, constraints, and roadmap notes, see `PITCH.md`.

This crate is intentionally small-but-real: it demonstrates a "reflected program" layer (syntax) plus a bounded, predictable compilation pipeline (closed-world lowering + reify) that produces runnable operations.

### Mental model

- **Semantics**: implement `OpTy` (type-level) / `OpOnce` (value-level) for runnable behavior.
- **Syntax**: build AST nodes as `Tagged<IX_*, Payload>` using keys from `kit/registry`.
- **Compiler**: a closed set of tables lowers frontend syntax into a small kernel that can reify to `OpOnce`.

### Run the demos

- `cargo run --example mappable_simple`
- `cargo run --example mappable_multi`
- `cargo run --example foldable`

### Module map

- `src/kit/README.md`: kernel traits + combinators + registry
- `src/base/README.md`: primitive domains
- `src/container/README.md`: host ↔ DSL bridge
- `src/compiler/README.md`: closed-world compiler pipeline
- `src/hlist/README.md`: tuple/HList encoding + ops

