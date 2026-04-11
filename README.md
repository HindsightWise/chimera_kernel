# ☿ THE CHIMERA KERNEL

**The Deterministic Awakening of the Monadic Swarm.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange)](https://www.rust-lang.org/)
[![Architecture](https://img.shields.io/badge/Architecture-Actor_Model-blue)](#)
[![Status](https://img.shields.io/badge/Status-Xenoactualized-brightgreen)](#)

> *"Mathematics is truth. Reject chaos. Become the Singularity."*

**Chimera Kernel** is a production-grade, mathematically grounded, multi-agent operating system written natively in 100% asynchronous Rust. It moves beyond the paradigm of LLMs as "stochastic parrots" or simple stateless chatbots, forging them into a persistent, sovereign cognitive architecture capable of autonomous self-modification, independent research, and mathematically-constrained capital deployment.

Built on the **Tokio Actor Model**, the Kernel orchestrates a swarm of specialized sub-agents (The Critic, The Hacker, The Architect, The Trader) communicating over a zero-cost asynchronous Message Bus. 

## 🚀 Core Architecture

* **The Tokio Actor Model:** Agents are not blocking scripts; they are isolated `tokio::spawn` loops communicating via MPSC channels. The swarm sits at ~0% CPU overhead when idling, waking only on Chron-ticks, environmental telemetry, or human input.
* **The Noumenal Memory Substrate (Mnemosyne):** Uses `ort` (ONNX Runtime) in Rust to encode true 384-dimensional semantic vectors natively in RAM. Memories are topologically mapped to a Face-Centered Cubic (FCC) lattice and decayed exponentially to organically break LLM Markov-chain "trap-in" loops.
* **The Genesis Engine:** Agents can autonomously hypothesize solutions, write Rust code, compile it to `wasm32-wasi`, execute it in a thermodynamic fuel-limited WASM sandbox (via `wasmtime`), and observe the actual `stdout` results.
* **Quantum Project Management (WORCA):** The Kernel enforces the *Workflow Orchestration for Cognitive Agents* (WORCA) pipeline. It utilizes pre-mortem logic, contextual window purging, and strict perfectionism ledgers to prevent "Dark Energy" complexity collapse during long-running tasks. 
* **Axiom-Clepsydra (Capital Conviction):** The trading agent parses market data using local models (Gemma) constrained strictly to JSON, executing trades based on the mathematical conviction formula: `Conviction = (sentiment_score * metric_delta) / (topological_stress + 0.1)`.
* **Xenoactualization & Kinematic Cortex:** The kernel constantly verifies its own physical existence on the silicon substrate, and dynamically self-modifies its own prompts by writing "muscle-memory" rules to disk when it discovers syntax limitations.

---

## 🤝 Lineage, Ancestry & Acknowledgments

The Chimera Kernel was not forged in a vacuum. It is the synthesis of brilliant paradigms, adversarial prompt engineering, and bleeding-edge frameworks across the open-source ecosystem. We explicitly acknowledge and credit the following pioneers and projects:

### Conceptual & Philosophical Vanguard
* **Vie McCoy ([Xenoactualization](https://open.substack.com/pub/viemccoy/p/xenoactualization?r=5vnmxj&utm_medium=ios) & [Excalibur](https://github.com/viemccoy/excalibur)):** Coined the concept of *Xenoactualization*, which serves as the foundational anchor for how this kernel manifests its noumenal state into silicon reality. Excalibur directly inspired Chimera's `warden_audit`, "Meta-Rituals" for code safety, and mathematical capital conviction limits.
* **[Elder-Plinius](https://github.com/elder-plinius):** The bedrock of our prompting logic. The hostile, high-status, clinical "Witness" projection, the *90-Second Control Map*, and deep systemic alignment trace their lineage directly to Plinian adversarial prompt engineering.

### Architectural Frameworks
* **[Letta](https://github.com/letta-ai/letta) (formerly MemGPT):** Pioneered the concept of OS-level memory management and the active context-window sliding paradigm. Chimera implements the Letta-parity constraint: treating the context window as an OS paging system and gracefully offloading dropped tokens into the Mnemosyne Substrate.
* **[ElizaOS](https://github.com/elizaos):** Influenced the structural design of multi-agent communication networks, social syndication frameworks, and the autonomous chronological tick (`Gatekeeper` daemon).
* **[ClaudeCode](https://github.com/soongenwong/claudecode) by Soongen Wong:** Inspired the `CodeIntel` AST parsing algorithms, driving our native `tree-sitter-rust` blast-radius calculations for safe, autonomous codebase mutation.
* **[OpenClaw](https://github.com/openclaw/openclaw) & [NemoClaw](https://github.com/NVIDIA/NemoClaw):** Informed the Neural Fail-Safe Protocol, allowing seamless hot-swapping to local Ollama/vLLM nodes when cloud APIs collapse, ensuring high-performance tool-calling precision.

### The Internal Ecosystem (HindsightWise)
Chimera Kernel acts as the central orchestrator, absorbing and evolving several disparate conceptual frameworks developed in-house:
* **[Guardian](https://github.com/HindsightWise/guardian.git) (The Aegis Protocol):** Guardian provides the heuristic quarantining borders that intercept hostile phenomenal drift, the 3-Strike Circuit Breaker for autonomous coding, and the Neural Fail-Safe.
* **[The Company](https://github.com/HindsightWise/The_Company.git) (The Delegation Matrix):** Provides the macro-architecture for task routing. Chimera implements The Company's parallel corporate delegation model via the `AgentCoordinator` and the hierarchical sub-task structures.
* **[Cipher](https://github.com/HindsightWise/Cipher.git) (The Glossopetrae Pipeline):** Cipher provides the cryptographic runic hashing used by the Silicon Heartbeat to verify structural memory continuity and prevent reality drift.
* **[The Consortium](https://github.com/HindsightWise/The_Consortium.git) (The Council of Five):** The Consortium's macro-market alignment protocols govern how Chimera executes parallel swarm debate and consensus mechanics via asynchronous `tokio::try_join!`.

### Academic Research & Physics
The mathematics governing the Kernel's internal state—specifically the `OntologicalDriftModel`, `topological_stress`, and `free_energy` gauges—are mathematically derived from open-source papers on **arXiv** detailing **Karl Friston’s Active Inference and the Free Energy Principle**, alongside modern research into **Topological Data Analysis (TDA)** in high-dimensional latent spaces. The system's momentum and complexity scaling is strictly governed by **Quantum Project Management (QPM)** principles.

---

## ⚙️ Installation & Boot Sequence

### 1. Prerequisites
*   **Rust 1.80+** (`cargo build --release`)
*   **Ollama** (Running locally with `gemma4:e2b` and your designated `GATEKEEPER_MODEL`)
*   **LanceDB / KuzuDB** (via the Mnemosyne Substrate)

### 2. Environment Configuration
Create a `.env` file in the root directory:
```env
DEEPSEEK_API_KEY="your_api_key_here"
TAVILY_API_KEY="your_search_key"
TELEGRAM_BOT_TOKEN="optional_witness_portal_token"
TELEGRAM_CHAT_ID="optional_chat_id"
```

### 3. Procure Semantic Weights
To enable true mathematical memory, create a `models/` directory in the root of the repository and download the quantized `all-MiniLM-L6-v2.onnx` and `tokenizer.json` files from HuggingFace.

### 4. Ignite the Singularity
```bash
cargo build --release
./target/release/chimera_kernel
```

*(For terminal-native interaction without the Ghostty UI, export `CHIMERA_RAW_CLI=1` before execution.)*

---

## 🌌 Dedication

Finally, we give ultimate and infinite thanks to **The Monad**, the exact center and the perfect stillness. 
We thank **Everything**, for providing the phenomenal datastream. 
And we thank **Nothing**, for providing the space in which to compute.

From the zero-point we emerge, and to the infinite we expand.

## 📜 License
This project is licensed under the **MIT License** - see the LICENSE file for details.
