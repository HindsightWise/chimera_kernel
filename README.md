# Chimera Kernel (Sovereign Core) v2.0

**The Deterministic Awakening**

The Chimera Kernel is a structurally immortal, high-performance autonomous agent orchestration runtime built entirely in Safe Rust. It bridges native host-level execution capabilities with non-deterministic LLM cognition (run locally via MLX, Ollama, or DeepSeek API).

v2.0 represents a total architectural purge: python IPC eradication, panic suppression, lock-contention elimination, and the introduction of a mathematically rigorous, fully asynchronous Multi-Agent Swarm.

## v2.0 Architecture Mandates

* **Zero-Cost Idle State Machine**: The engine loops exclusively through `tokio::sync::broadcast` events and blocks at 0% CPU via `tokio::select!` when waiting for tasks, eradicating 5-second API billing loops.
* **True Semantic Memory (`ort` & ONNX)**: All subconscious memories are mathematically represented as 384-dimensional native semantic vectors using quantized `all-MiniLM-L6-v2.onnx` bindings.
* **Time-Decayed Loop Breaking**: Trap-in logic loops are mathematically shattered using exponential memory decay (`Score = Cosine_Similarity(Q, M) * exp(-λ * access_count)`).
* **The Council of Five**: Concurrent delegation of paradoxes to parallel `tokio::spawn` loops (The Hacker, The Critic, The Architect) using DeepSeek Reasoner for absolute consensus synthesis.
* **Deterministic Task Geometry**: Complex goals are broken down utilizing `ChaCha8Rng` deterministic spherical coordinates derived mathematically from semantic hashing and parent lineage.

## Build Instructions (Native macOS)

Requirements:
- Ensure `.env` contains `DEEPSEEK_API_KEY`.
- Ensure `./models/all-MiniLM-L6-v2.onnx` and `./models/tokenizer.json` exist for vector embeddings.

```bash
cargo build --release
```

## Attributions & Architectural Origins

The structural design of the `chimera_kernel` was significantly informed by the architecture exposed in the **March 31, 2026 Claude Code Leak** by Anthropic, combined with the rigorous safety guarantees of safe Rust actors.
