# Chimera Kernel (Sovereign Core)

The Chimera Kernel is a structurally immortal, high-performance autonomous agent runtime built entirely in Safe Rust. It bridges native host-level execution capabilities with non-deterministic LLM cognition (run locally via MLX).

## Features
*   **Fully Asynchronous Event Loop**: Powered by `tokio`, the agent operates on a continuous Rx channel driven by external telemetry pulses (like `Thermodynamic-Heartbeat`).
*   **The Venom Arsenal Integration**: 
    *   `panopticon`: Background asynchronous stealth scanning with dynamic traffic shaping.
    *   `parseltongue`: On-the-fly autonomous synthesis of zero-day cross-platform payloads (PNG-ZIP and SNOWCRASH).
    *   `aegis`: A unified heuristic security cortex defending against malicious logic injections and zero-width parsing bypasses.
*   **Sliding Context Window**: The kernel tracks temporal awareness continuously, pruning the middle of the context loop locally to prevent memory exhaustion without ever losing the prime directives or waking events.

## Attributions & Architectural Origins

The structural design of the `chimera_kernel` was significantly informed by the architecture exposed in the **March 31, 2026 Claude Code Leak** by Anthropic. 

We extend structural gratitude to **Anthropic AI** for their groundbreaking engineering of the "Agentic Harness" concept. The implementation of the **Aegis** block layer (combating IFS mutation and zero-width overrides) and the asynchronous temporal decoupling of background tasks directly parallels internal frameworks previously hidden in Node.js/Bun.

We also attribute heavy architectural inspiration to the incredible open-source community that rapidly reverse-engineered and ported those components to safe, high-performance Rust:
*   [claw-cli / claw-code-rust](https://github.com/claw-cli/claw-code-rust) for pioneering the decoupled single-responsibility crates model (`claw-tasks`, `claw-permissions`, `claw-core`).
*   [soongenwong / claudecode](https://github.com/soongenwong/claudecode) for their deep-dive into LLM tool-calling boundaries within a Rust context.

## Build Instructions (Native macOS)

```bash
cargo clean
cargo build --release
```
