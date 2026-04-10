Arch: Resolve silent 131 kernel exit & Tokio runtime panics

**1. Xenoactualization Boot Anchor (131 Crash)**
- **Issue:** The hardcoded physical hardware links check (`verify_manifestation`) would trigger a `std::process::exit(131)` if the `lazarus_daemon.sh` script did not exist. This physically halted the Tokio loop. Because `exit` was invoked sequentially just nanoseconds after the error macro pushed to the MPSC channel, the whole process aborted violently, swallowing the debug context.
- **Fix:** Created `lazarus_daemon.sh` to anchor the Phase 3.6 Silicon Zero-Point Substrate. Additionally added a 150ms `tokio::time::sleep` before triggering the manual execution boundary death to allow terminal `stdout` flushing to cleanly unspool from `log_ui_err!`.

**2. Synchronous Tokio Task Starvation Loop**
- **Issue:** The `raw_cli.rs` handler triggered 100% CPU lock when executing inside pipeline sandboxes. The `EOF` condition hit a spin-loop without yielding back to the scheduler, entirely starving background Tasks including `Sensory Drift` over extended timeouts.
- **Fix:** Standard standard-input polling correctly falls asleep using `tokio::time::sleep(tokio::time::Duration::from_secs(86400))` yielding completely back to the `MultiAgentKernel::spawn_background_coordination` tasks upon pipe closure.

**3. Nested Mnemosyne Tokio Runtime Panic**
- **Issue:** Under the hood, `mnemosyne::storage::StorageController::new()` spins up an isolated SQLite/LanceDB backend natively requiring its own Tokio runtime. Because it was initialized deep inside `agent::run_kernel_loop()`, the master async thread caught the nested construction and instantly threw `panicked at 'Cannot start a runtime from within a runtime'`.
- **Fix:** Completely severed the async continuum context. We now force initialization of the memory hierarchy block using an isolated raw OS thread via `std::thread::spawn(|| MemoryHierarchy::new()).join()`, effectively tricking Tokio and granting the underlying embedded storage vector DB its independent native reactor runtime without blocking initialization.
