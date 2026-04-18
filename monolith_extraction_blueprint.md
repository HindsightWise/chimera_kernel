# Engineering Handoff: Monolith Decoupling (Phase 6 / code-simplify)

## The Core Problem State
During Phase 6 (`/code-simplify`), an attempt was made to extract **17 inline submodules** deeply nested inside two massive monolithic "God Files":
- `src/cognitive_loop.rs` (3,181 lines, 12 modules)
- `src/core_identity.rs` (1,617 lines, 5 modules)

**Why the Automation Failed:**
The extracted `target_file` size vastly exceeded human cognitive and context window limits ("The Rule of 500"). To automate the extraction, an external Python regex/string-matching script was executed to seek `pub mod { ... }` blocks and count opening/closing brackets `{}`. 
However, Rust syntax is notoriously hostile to pure string-matching. Edge cases inside the files—specifically `json!({ ... })` macros and multi-line raw strings (`r#"..."#`)—broke the bracket counting heuristic, resulting in catastrophic bracket merging that stripped the AST logic incorrectly.

**The Golden Rule:** You cannot safely rewrite thousands of lines of Rust utilizing primitive string replacement regex.

---

## The Target Architectural State
The goal is to transition from inline modules to Idiomatic Rust directory structures. The execution sequence must result in the following file tree layout natively resolving Rust boundaries:

### Target `src/cognitive_loop/` Boundary
Create a new directory named `src/cognitive_loop/` and extract the inline blocks from `src/cognitive_loop.rs` into independent files.
The `src/cognitive_loop.rs` file should eventually be shrunk to 12 lines resembling:
```rust
pub mod agent;
pub mod multi_agent_kernel;
pub mod task_manager;
pub mod task_decomposer;
pub mod message_bus;
pub mod agent_trait;
pub mod agent_coordinator;
pub mod agent_registry;
pub mod auto_dream;
pub mod dependency_graph;
pub mod plugins;
pub mod presentation_layer;
```

### Target `src/core_identity/` Boundary
Create a new directory named `src/core_identity/` and extract the 5 inline blocks from `src/core_identity.rs`.
The `src/core_identity.rs` file should eventually resemble:
```rust
pub mod specialized_agents;
pub mod duality;
pub mod kinematics;
pub mod self_model;
pub mod xenoactualization;
```

---

## How the Next Agent Must Solve This (Two Valid Paths)

Since primitive Python parsing failed, the next agent must rely on absolute code boundary exactitude. They have two execution tracks depending on their tooling constraints.

### Execution Path 1: The `syn`-crate AST Extractor (Recommended for AI)
Instead of guessing bracket boundaries with Python, the agent should write a dedicated, temporary **Rust binary** utilizing the `syn` and `quote` crates to parse the physical Rust Abstract Syntax Tree (AST), ensuring 100% mathematical precision.

**Steps for the Agent:**
1. Generate a temporary cargo binary crate: `cargo new --bin ast_splitter`.
2. Add dependencies: `cargo add syn --features full` and `cargo add quote`.
3. Write Rust code that iterates over the `syn::ItemMod` elements of `cognitive_loop.rs`.
4. If an `ItemMod` has content (an inline module), the script pulls the `quote!` token stream for its inner items, writes that payload to `cognitive_loop/<name>.rs`, and modifies the original `ItemMod` to remove its `{ ... }` brackets.
5. Compile and run the `ast_splitter` directly against the codebase.
6. Verify via `cargo check -p monad` and then format with `cargo fmt`.

### Execution Path 2: Granular Line-Boundary Slicing (The Painful Route)
If compiling a robust `syn` binary is blocked by environment constraints, the agent must perform the extraction manually, strictly leveraging exact line numbers via bash tools without generic substitution logic.

**Steps for the Agent:**
1. **Locate Absolute Boundaries**: Utilize absolute grep (`git grep -n "pub mod agent {"`) to locate the exact starting line. 
2. Because Python regex fails to find the ending bracket `}`, the agent must carefully examine the file visually (via `sed -n 'X,Yp'` or `view_file`) by jumping to the end of the module to find the closing bracket location securely.
3. **Shatter via Awk/Head/Tail**: Use strict exact-line slicing. 
    - E.g. `sed -n '120,400p' src/cognitive_loop.rs > src/cognitive_loop/agent.rs`
    - Make sure to remove the outer `pub mod agent {` and final `}` from the newly created `agent.rs`.
4. Replace lines 120-400 in the primary `cognitive_loop.rs` with `pub mod agent;\n`.
5. Run `cargo check` after **every single module extraction**. Do not batch them. If one module breaks scope, the agent knows exactly which boundary failed.

---

## Final Validation Criteria for Handoff
No matter which path the next agent chooses, the execution is NOT VERIFIED until:
1. `cargo check -p monad` passes strictly (proving no local `self::` or `super::` macro leakages occurred from altering inline module proximity).
2. The sizes of `src/cognitive_loop.rs` and `core_identity.rs` strictly fall underneath the 500-line warning threshold.
