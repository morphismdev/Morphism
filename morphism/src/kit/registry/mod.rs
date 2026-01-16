mod codes;
pub use codes::*;

mod roles;
pub use roles::*;

pub mod bool;
pub use bool::*;

pub mod combinators;
pub use combinators::*;

mod new_type_node;
pub use new_type_node::*;

mod hlist;
pub use hlist::*;

mod hlist_fold;
pub use hlist_fold::*;

mod op;
pub use op::*;

mod tagged;
pub use tagged::*;

mod generic;
pub use generic::*;

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    use crate::{IX_HLISTNODE, IX_TRUE};

    /// Collision guard: ensures critical keys from different domains are distinct.
    ///
    /// This test catches domain code collisions (like the OpDomain bug we fixed).
    /// If two keys from different domains have the same TypeId, they're type-equal
    /// and dispatch/pattern matching will be ambiguous.
    #[test]
    fn key_collision_guard() {
        // Different domains, same index (U0) - should still be distinct
        assert_ne!(
            TypeId::of::<IX_TRUE>(),
            TypeId::of::<IX_HLISTNODE>(),
            "IX_TRUE (BoolDomain) and IX_HLISTNODE (HlistDomain) must be distinct"
        );
    }

    // =============================================================================
    // Registry guard tests (Phase 4.D)
    // =============================================================================
    //
    // Goal: keep the registry a clean "identity allocation table" by preventing drift:
    //
    // - No dead keys: every `IX_*` declared under `src/kit/registry/` must be referenced
    //   somewhere outside `registry/` (a real owner or consumer).
    //
    // - No ghost owners: every `Tagged<IX_*, ...>` program owner must have a corresponding
    //   `IX_*` declaration in `src/kit/registry/`.
    //
    // Why tests (instead of convention only)?
    // - The registry is easy to pollute during refactors.
    // - Stale keys are hard to notice and create “false architecture” (looks supported, isn't).
    // - We want to keep indices/key-space small and intentional until a key is actually used.
    //
    // How these tests work (step-by-step):
    //
    // 1) Collect declared keys:
    //    - Walk `src/kit/registry/**/*.rs`
    //    - Parse lines that look like `pub type IX_FOO = ...`
    //    - Collect `IX_FOO` into a set.
    //
    // 2) Collect reference candidates:
    //    - Walk `src/**/*.rs` and `tests/**/*.rs`
    //    - Exclude `src/kit/registry/` itself.
    //
    // 3) Dead key check (`no_dead_registry_keys`):
    //    - For each `IX_*` declared in registry, scan non-registry Rust files for identifier
    //      occurrences of that exact token.
    //    - Fail if any are not found.
    //
    // 4) Ghost owner check (`all_tagged_ix_owners_have_registry_keys`):
    //    - Scan non-registry Rust files for `Tagged<IX_*` and `Tagged::<IX_*`.
    //    - Fail if any extracted `IX_*` is not declared in registry.
    //
    // Notes / limitations:
    // - This is intentionally a lightweight lexical scan (no Rust parsing).
    // - It may miss extremely exotic formatting, but it is robust for the crate's style.
    // - It can be fooled by strings/comments in edge cases; we tolerate this because the
    //   ergonomics payoff is high and the codebase uses `IX_*` mostly as type tokens.
    // - The failure output is a curated "to-fix list" (add an owner/use-site or remove key).
    //
    fn walk_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(rd) = fs::read_dir(dir) else {
            return;
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_rs_files(&path, out);
            } else if path.extension().and_then(|x| x.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }

    /// Return true if `ident` appears in `haystack` as a standalone identifier token.
    ///
    /// This intentionally uses ASCII identifier boundaries (`[A-Za-z0-9_]`) because `IX_*`
    /// keys are ASCII and the crate uses them as type-level identifiers.
    fn contains_ident(haystack: &str, ident: &str) -> bool {
        let mut from = 0;
        while let Some(rel) = haystack[from..].find(ident) {
            let start = from + rel;
            let end = start + ident.len();

            let left_ok = start == 0
                || !haystack[..start]
                    .chars()
                    .next_back()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
            let right_ok = end == haystack.len()
                || !haystack[end..]
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');

            if left_ok && right_ok {
                return true;
            }
            from = end;
        }
        false
    }

    /// Parse all `IX_*` keys declared in `src/registry/**/*.rs` by scanning for `pub type IX_*`.
    fn parse_registry_ix_keys(registry_dir: &Path) -> BTreeSet<String> {
        let mut keys = BTreeSet::new();

        let mut files = Vec::new();
        walk_rs_files(registry_dir, &mut files);

        for file in files {
            let Ok(src) = fs::read_to_string(&file) else {
                continue;
            };
            for line in src.lines() {
                let line = line.trim_start();
                let Some(rest) = line.strip_prefix("pub type ") else {
                    continue;
                };
                let Some(tok) = rest.split_whitespace().next() else {
                    continue;
                };
                if tok.starts_with("IX_") {
                    keys.insert(tok.to_string());
                }
            }
        }

        keys
    }

    /// Extract `IX_*` tokens from `Tagged<IX_*, ...>` / `Tagged::<IX_*, ...>` occurrences.
    ///
    /// This is used to ensure that every `Tagged<IX_*, ...>` owner has a corresponding
    /// `IX_*` declaration in the registry.
    fn parse_tagged_ix_owners(non_registry_files: &[PathBuf]) -> BTreeSet<String> {
        let mut used = BTreeSet::new();

        for file in non_registry_files {
            let Ok(src) = fs::read_to_string(file) else {
                continue;
            };

            // Parse patterns like:
            // - Tagged<IX_FOO, ...>
            // - Tagged::<IX_FOO, ...>
            for needle in ["Tagged<", "Tagged::<"] {
                let mut from = 0;
                while let Some(rel) = src[from..].find(needle) {
                    let start = from + rel + needle.len();
                    let rest = &src[start..];

                    // Skip whitespace
                    let ws = rest
                        .char_indices()
                        .take_while(|(_, c)| c.is_whitespace())
                        .last()
                        .map(|(i, c)| i + c.len_utf8())
                        .unwrap_or(0);
                    let rest = &rest[ws..];

                    // Read token until comma / whitespace / '>' / ':'
                    let token_len = rest
                        .char_indices()
                        .take_while(|(_, c)| {
                            !c.is_whitespace() && *c != ',' && *c != '>' && *c != ':'
                        })
                        .last()
                        .map(|(i, c)| i + c.len_utf8())
                        .unwrap_or(0);

                    if token_len > 0 {
                        let token = &rest[..token_len];
                        if token.starts_with("IX_") {
                            used.insert(token.to_string());
                        }
                    }

                    from = start;
                }
            }
        }

        used
    }

    /// Guard: every `IX_*` declared in the registry must be referenced outside `registry/`.
    ///
    /// A key is considered "referenced" if it appears as an identifier token in any Rust
    /// file under `src/` or `tests/`, excluding the `src/kit/registry/` directory.
    #[test]
    fn no_dead_registry_keys() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let src_dir = manifest_dir.join("src");
        let registry_dir = src_dir.join("kit").join("registry");

        let registry_keys = parse_registry_ix_keys(&registry_dir);

        let mut files = Vec::new();
        walk_rs_files(&src_dir, &mut files);
        walk_rs_files(&manifest_dir.join("tests"), &mut files);

        // Exclude registry itself.
        let non_registry_files: Vec<PathBuf> = files
            .into_iter()
            .filter(|p| !p.starts_with(&registry_dir))
            .collect();

        let mut dead = Vec::new();
        for key in &registry_keys {
            let mut used = false;
            for file in &non_registry_files {
                let Ok(src) = fs::read_to_string(file) else {
                    continue;
                };
                if contains_ident(&src, key) {
                    used = true;
                    break;
                }
            }
            if !used {
                dead.push(key.clone());
            }
        }

        assert!(
            dead.is_empty(),
            "Dead registry keys (defined but never referenced outside `registry/`):\n- {}",
            dead.join("\n- ")
        );
    }

    /// Guard: every `Tagged<IX_*, ...>` owner must have `IX_*` declared in the registry.
    #[test]
    fn all_tagged_ix_owners_have_registry_keys() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let src_dir = manifest_dir.join("src");
        let registry_dir = src_dir.join("kit").join("registry");

        let registry_keys = parse_registry_ix_keys(&registry_dir);

        let mut files = Vec::new();
        walk_rs_files(&src_dir, &mut files);
        walk_rs_files(&manifest_dir.join("tests"), &mut files);

        // Exclude registry itself.
        let non_registry_files: Vec<PathBuf> = files
            .into_iter()
            .filter(|p| !p.starts_with(&registry_dir))
            .collect();

        let tagged_used = parse_tagged_ix_owners(&non_registry_files);

        let mut missing = Vec::new();
        for key in tagged_used {
            if !registry_keys.contains(&key) {
                missing.push(key);
            }
        }

        assert!(
            missing.is_empty(),
            "Found `Tagged<IX_*, ...>` owners with missing registry keys:\n- {}",
            missing.join("\n- ")
        );
    }
}
