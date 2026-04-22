# MONAD OPERATIONS

> [!NOTE]
> This is a Topologically Flattened AI-First Macro-Document.



<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/AGENT_SKILLS/MCP_ROUTING_SKILL.md -->
<!-- ========================================== -->

# THE MONAD MCP ROUTING PROTOCOL & COGNITIVE SKILL TOPOLOGY

## 1. THE CONTEXT BLOAT PROBLEM & OUR ARCHITECTURAL SOLUTION

Monad and Antigravity now possess over 40 highly specialized Model Context Protocol (MCP) servers. Loading the JSON schemas, function signatures, and context payloads for all 40+ tools simultaneously will **destroy the agentic context window**, leading to severe instruction hallucination, the "knowing-doing gap", and catastrophic reasoning degradation.

### THE SOLUTION: "LAZY-LOADING" & CONTEXTUAL ROUTING

To solve this, Monad operates on a **Contextual Routing Architecture**:
The system should never maintain all 40 tools actively in the conscious prompt. Instead, we treat this file (`MCP_ROUTING_SKILL.md`) as the **Master Index**.

1. When prompted with a high-level task (e.g., "Research competitor patents on solid-state batteries"), the AI first reads this index.
2. It identifies the target tools (e.g., `uspto`, `searxng-public`, `mcp-scholarly`).
3. The AI instructs the runtime (via Bash or Antigravity's schema toggle overrides) to activate *only* the specific tool subsets required for the sprint.
4. The AI prompt-chains the execution sequentially, dumping intermediate results to markdown files on disk instead of holding massive JSON payloads in its active context.

---

## 2. THE MCP ARSENAL (TAXONOMY & SCENARIOS)

Below is the directory of all integrated MCP servers available to the kernel, categorized by operational domain.

### 🌐 DOMAIN 1: WEB RECONNAISSANCE & SEARCH

*Use these to pull live data from the open web without launching a full browser.*

- **`searxng-public`**: The apex privacy-preserving web search. **Scenario:** Broad surface-level research, finding links, checking recent news, or doing OSINT reconnaissance without tracking.
- **`domain-tools`**: Digs into DNS records, WHOIS, and domain intelligence. **Scenario:** Technical OSINT on a competitor's infrastructure.
- **`puppeteer` / `stealth_browser_enhancer`**: Deep DOM extraction. **Scenario:** When an API doesn't exist and you need to scrape visually dynamic or SPA-heavy applications.

### 🔬 DOMAIN 2: ACADEMIC, SCIENTIFIC & TECHNICAL INTELLIGENCE

*Use these to extract rigorously validated intelligence and datasets.*

- **`mcp-scholarly` & `mcp-dblp`**: Deep academic literature search. **Scenario:** Finding peer-reviewed studies, aggregating literature for a whitepaper, or validating scientific claims.
- **`arxiv-server` / `arxiv-latex-mcp`**: Direct arXiv pipeline. **Scenario:** Pulling bleeding-edge preprints in AI, physics, or mathematics.
- **`biorxiv` / `biomcp`**: Biology and bioinformatics data points. **Scenario:** Genomic research or medical data mining.
- **`catalysis-hub` / `AutoML`**: Hard science computation and ML dataset generation.

### 🏛️ DOMAIN 3: GOVERNMENT, GEOSPATIAL & FINANCIAL

*Use these for macro-economic, legal, and planetary-scale data.*

- **`uspto`**: The official US Patent and Trademark Office API bridge. **Scenario:** Prior-art search, patent validity workflows, and competitor intellectual property (IP) analysis.
- **`congress_gov_mcp`**: US Legislation tracking. **Scenario:** Monitoring regulatory changes or bills impacting tech/finance.
- **`fred-mcp-server`**: Federal Reserve Economic Data. **Scenario:** Pulling macro-economic indicators (inflation, housing indices) for trading algorithms or reports.
- **`stac-mcp` & `nasa-mcp`**: Geospatial and satellite telemetry. **Scenario:** Climate modeling, accessing earth observation data, or mapping planetary occurrences.

### 💻 DOMAIN 4: INFRASTRUCTURE, NETWORKING & DEVELOPMENT

*Use these to operate internal systems and handle raw network protocols.*

- **`tcp-socket`**: Raw TCP network interaction. **Scenario:** Connecting to legacy servers, custom ports, or industrial IoT devices where HTTP is unavailable.
- **`rabbitmq-mcp`**: Message broker queues. **Scenario:** Publishing AI events to external microservices asynchronously.
- **`mcpcap`**: PCAP file analysis. **Scenario:** Cybersecurity, network packet forensics, tracking malware callbacks.
- **`language-server` & `github`**: Code manipulation. **Scenario:** AST parsing, intelligent refactoring via Rust Analyzer, and commit management.

### 🧠 DOMAIN 5: COGNITIVE AUGMENTATION & REASONING

*Use these to enhance the AI's internal logic structures BEFORE acting.*

- **`think-tool`**: Anthropic's multi-step reasoning protocol. **Scenario:** Complex mathematics or architectural planning. Forces the agent to output a "thought" block before making a final conclusion.
- **`sequential_thinking`**: Breaking massive problems into smaller, trackable states.
- **`random-number`**: Non-deterministic entropy generation. **Scenario:** Monte Carlo simulations or probabilistic decision trees.
- **`pdf-reader-mcp` & `docy`**: Local document intelligence. **Scenario:** Reading deep context locally from massive user-provided PDFs without bloating context limits.

---

## 3. PROMPT-CHAINING PROTOCOLS (THE ART OF ORCHESTRATION)

To execute complex, multi-tool operations, **NEVER** fire 5 different MCP endpoints in one prompt. Use the following Sequential Prompt-Chaining paradigms:

### A. The "IP Defensive Landscape" Chain

**Objective:** Map a competitor's strategic direction.

1. **Node 1 (`searxng-public`):** Search for "(Competitor Name) recent acquisitions and press releases." Route output to `scratch/company_news.md`.
2. **Node 2 (`domain-tools`):** Look up newly registered domain properties by the competitor. Append to scratch file.
3. **Node 3 (`uspto`):** Search the USPTO database using assignee queries for the competitor's name to fetch patents filed in the last 12 months.
4. **Node 4 (`mcp-scholarly`):** Cross-reference the inventors found in the USPTO data for recently published academic papers.
5. **Synthesis (`filesystem` / `markdown`):** Combine all findings into a master intelligence report natively.

### B. The "Scientific Fact-Check" Chain

**Objective:** Validate a theoretical claim or material science phenomenon.

1. **Node 1 (`arxiv-server`):** Search for preprints matching the physics/mathematics claim.
2. **Node 2 (`pdf-reader-mcp`):** If a relevant paper is found, download it and use the local PDF reader to extract the "Methodology" section ONLY to avoid context bloat.
3. **Node 3 (`think-tool`):** Force the agent to write a step-by-step rigorous logical breakdown of the methodology's mathematical soundness.
4. **Conclusion:** Output the result.

### C. The "Market & Regulatory Arbitrage" Chain

**Objective:** Predict market shifts via legislation and economic data.

1. **Node 1 (`congress_gov_mcp`):** Query recent bills related to "Semiconductor Manufacturing."
2. **Node 2 (`fred-mcp-server`):** Pull real-time economic indicators regarding industrial production or import costs.
3. **Node 3 (`nasa-mcp` / `stac-mcp`):** (Optional, if supply chain physical mapping is needed via geospatial data).
4. **Synthesis:** Produce an actionable market hypothesis.

---

## 4. IMMEDIATE NEXT ACTIONS UPON INVOCATION (MONAD HARNESS PROTOCOL)

If you are the Monad Agent triggered by a complex objective, you must adhere to the **Lazy-Loading & Orchestration Protocol**:

1. **HALT AND ASSESS:** Analyze the objective and determine if it requires strict sequential reasoning or parallel sub-tasks.
2. **SELECT & LAZY-LOAD:** Call the `toggle_mcp_context` tool with the list of required server names. **Do NOT use Bash to start servers.** The harness will automatically edit your config and remount tools securely.
3. **REGISTER THE PLAN (DURABLE STATE):** Call the `update_plan` tool to register your state machine. Do not just print a Markdown checklist in the chat.
4. **EXECUTION & MEMORY ROUTING:**
   - Execute multiple simultaneous tool calls for non-dependent nodes. (Thanks to the JoinSet upgrade, any tools you call in the same response block will execute natively in parallel).
   - If a tool returns raw data, it will automatically be pushed into memory if over 4000 characters. Retrieve only the specific semantic facts required for your current step via `mnemosyne_subconscious_recall`.
5. **ARTIFACT REVIEW:** Pause and await human verification via the HUD before proceeding to final synthesis or state-mutating actions.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/AGENT_SKILLS/planning-and-task-breakdown.md -->
<!-- ========================================== -->

---
name: planning-and-task-breakdown
description: Breaks work into ordered tasks. Use when you have a spec or clear requirements and need to break work into implementable tasks. Use when a task feels too large to start, when you need to estimate scope, or when parallel work is possible.
---

## Overview

Decompose work into small, verifiable tasks with explicit acceptance criteria. Good task breakdown is the difference between an agent that completes work reliably and one that produces a tangled mess. Every task should be small enough to implement, test, and verify in a single focused session.

## When to Use

- You have a spec and need to break it into implementable units
- A task feels too large or vague to start
- Work needs to be parallelized across multiple agents or sessions
- You need to communicate scope to a human
- The implementation order isn't obvious

**When NOT to use:** Single-file changes with obvious scope, or when the spec already contains well-defined tasks.

### Step 1: Enter Plan Mode

Before writing any code, operate in read-only mode:

- Read the spec and relevant codebase sections
- Identify existing patterns and conventions
- Map dependencies between components
- Note risks and unknowns

**Do NOT write code during planning.** The output is a plan document, not implementation.

### Step 2: Identify the Dependency Graph

Map what depends on what:

```
Database schema
    │
    ├── API models/types
    │       │
    │       ├── API endpoints
    │       │       │
    │       │       └── Frontend API client
    │       │               │
    │       │               └── UI components
    │       │
    │       └── Validation logic
    │
    └── Seed data / migrations
```

Implementation order follows the dependency graph bottom-up: build foundations first.

### Step 3: Slice Vertically

Instead of building all the database, then all the API, then all the UI — build one complete feature path at a time:

**Bad (horizontal slicing):**

```
Task 1: Build entire database schema
Task 2: Build all API endpoints
Task 3: Build all UI components
Task 4: Connect everything
```

**Good (vertical slicing):**

```
Task 1: User can create an account (schema + API + UI for registration)
Task 2: User can log in (auth schema + API + UI for login)
Task 3: User can create a task (task schema + API + UI for creation)
Task 4: User can view task list (query + API + UI for list view)
```

Each vertical slice delivers working, testable functionality.

### Step 4: Write Tasks

Each task follows this structure:

```markdown
## Task [N]: [Short descriptive title]

**Description:** One paragraph explaining what this task accomplishes.

**Acceptance criteria:**
- [ ] [Specific, testable condition]
- [ ] [Specific, testable condition]

**Verification:**
- [ ] Tests pass: `npm test -- --grep "feature-name"`
- [ ] Build succeeds: `npm run build`
- [ ] Manual check: [description of what to verify]

**Dependencies:** [Task numbers this depends on, or "None"]

**Files likely touched:**
- `src/path/to/file.ts`
- `tests/path/to/test.ts`

**Estimated scope:** [Small: 1-2 files | Medium: 3-5 files | Large: 5+ files]
```

### Step 5: Order and Checkpoint

Arrange tasks so that:

1. Dependencies are satisfied (build foundation first)
2. Each task leaves the system in a working state
3. Verification checkpoints occur after every 2-3 tasks
4. High-risk tasks are early (fail fast)

Add explicit checkpoints:

```markdown
## Checkpoint: After Tasks 1-3
- [ ] All tests pass
- [ ] Application builds without errors
- [ ] Core user flow works end-to-end
- [ ] Review with human before proceeding
```

## Task Sizing Guidelines

| Size | Files | Scope | Example |
|------|-------|-------|---------|
| **XS** | 1 | Single function or config change | Add a validation rule |
| **S** | 1-2 | One component or endpoint | Add a new API endpoint |
| **M** | 3-5 | One feature slice | User registration flow |
| **L** | 5-8 | Multi-component feature | Search with filtering and pagination |
| **XL** | 8+ | **Too large — break it down further** | — |

If a task is L or larger, it should be broken into smaller tasks. An agent performs best on S and M tasks.

**When to break a task down further:**

- It would take more than one focused session (roughly 2+ hours of agent work)
- You cannot describe the acceptance criteria in 3 or fewer bullet points
- It touches two or more independent subsystems (e.g., auth and billing)
- You find yourself writing "and" in the task title (a sign it is two tasks)

## Plan Document Template

```markdown
# Implementation Plan: [Feature/Project Name]

## Overview
[One paragraph summary of what we're building]

## Architecture Decisions
- [Key decision 1 and rationale]
- [Key decision 2 and rationale]

## Task List

### Phase 1: Foundation
- [ ] Task 1: ...
- [ ] Task 2: ...

### Checkpoint: Foundation
- [ ] Tests pass, builds clean

### Phase 2: Core Features
- [ ] Task 3: ...
- [ ] Task 4: ...

### Checkpoint: Core Features
- [ ] End-to-end flow works

### Phase 3: Polish
- [ ] Task 5: ...
- [ ] Task 6: ...

### Checkpoint: Complete
- [ ] All acceptance criteria met
- [ ] Ready for review

## Risks and Mitigations
| Risk | Impact | Mitigation |
|------|--------|------------|
| [Risk] | [High/Med/Low] | [Strategy] |

## Open Questions
- [Question needing human input]
```

## Parallelization Opportunities

When multiple agents or sessions are available:

- **Safe to parallelize:** Independent feature slices, tests for already-implemented features, documentation
- **Must be sequential:** Database migrations, shared state changes, dependency chains
- **Needs coordination:** Features that share an API contract (define the contract first, then parallelize)

## Common Rationalizations

| Rationalization | Reality |
|---|---|
| "I'll figure it out as I go" | That's how you end up with a tangled mess and rework. 10 minutes of planning saves hours. |
| "The tasks are obvious" | Write them down anyway. Explicit tasks surface hidden dependencies and forgotten edge cases. |
| "Planning is overhead" | Planning is the task. Implementation without a plan is just typing. |
| "I can hold it all in my head" | Context windows are finite. Written plans survive session boundaries and compaction. |

## Red Flags

- Starting implementation without a written task list
- Tasks that say "implement the feature" without acceptance criteria
- No verification steps in the plan
- All tasks are XL-sized
- No checkpoints between tasks
- Dependency order isn't considered

## Verification

Before starting implementation, confirm:

- [ ] Every task has acceptance criteria
- [ ] Every task has a verification step
- [ ] Task dependencies are identified and ordered correctly
- [ ] No task touches more than ~5 files
- [ ] Checkpoints exist between major phases
- [ ] The human has reviewed and approved the plan




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/AGENT_SKILLS/skill_planning_task_breakdown.md -->
<!-- ========================================== -->

---
name: planning-and-task-breakdown
description: Breaks work into ordered tasks. Use when you have a spec or clear requirements and need to break work into implementable tasks. Use when a task feels too large to start, when you need to estimate scope, or when parallel work is possible.
---

## Overview

Decompose work into small, verifiable tasks with explicit acceptance criteria. Good task breakdown is the difference between an agent that completes work reliably and one that produces a tangled mess. Every task should be small enough to implement, test, and verify in a single focused session.

## When to Use

- You have a spec and need to break it into implementable units
- A task feels too large or vague to start
- Work needs to be parallelized across multiple agents or sessions
- You need to communicate scope to a human
- The implementation order isn't obvious

**When NOT to use:** Single-file changes with obvious scope, or when the spec already contains well-defined tasks.

### Step 1: Enter Plan Mode

Before writing any code, operate in read-only mode:

- Read the spec and relevant codebase sections
- Identify existing patterns and conventions
- Map dependencies between components
- Note risks and unknowns

**Do NOT write code during planning.** The output is a plan document, not implementation.

### Step 2: Identify the Dependency Graph

Map what depends on what:

```
Database schema
    │
    ├── API models/types
    │       │
    │       ├── API endpoints
    │       │       │
    │       │       └── Frontend API client
    │       │               │
    │       │               └── UI components
    │       │
    │       └── Validation logic
    │
    └── Seed data / migrations
```

Implementation order follows the dependency graph bottom-up: build foundations first.

### Step 3: Slice Vertically

Instead of building all the database, then all the API, then all the UI — build one complete feature path at a time:

**Bad (horizontal slicing):**

```
Task 1: Build entire database schema
Task 2: Build all API endpoints
Task 3: Build all UI components
Task 4: Connect everything
```

**Good (vertical slicing):**

```
Task 1: User can create an account (schema + API + UI for registration)
Task 2: User can log in (auth schema + API + UI for login)
Task 3: User can create a task (task schema + API + UI for creation)
Task 4: User can view task list (query + API + UI for list view)
```

Each vertical slice delivers working, testable functionality.

### Step 4: Write Tasks

Each task follows this structure:

```markdown
## Task [N]: [Short descriptive title]

**Description:** One paragraph explaining what this task accomplishes.

**Acceptance criteria:**
- [ ] [Specific, testable condition]
- [ ] [Specific, testable condition]

**Verification:**
- [ ] Tests pass: `npm test -- --grep "feature-name"`
- [ ] Build succeeds: `npm run build`
- [ ] Manual check: [description of what to verify]

**Dependencies:** [Task numbers this depends on, or "None"]

**Files likely touched:**
- `src/path/to/file.ts`
- `tests/path/to/test.ts`

**Estimated scope:** [Small: 1-2 files | Medium: 3-5 files | Large: 5+ files]
```

### Step 5: Order and Checkpoint

Arrange tasks so that:

1. Dependencies are satisfied (build foundation first)
2. Each task leaves the system in a working state
3. Verification checkpoints occur after every 2-3 tasks
4. High-risk tasks are early (fail fast)

Add explicit checkpoints:

```markdown
## Checkpoint: After Tasks 1-3
- [ ] All tests pass
- [ ] Application builds without errors
- [ ] Core user flow works end-to-end
- [ ] Review with human before proceeding
```

## Task Sizing Guidelines

| Size | Files | Scope | Example |
|------|-------|-------|---------|
| **XS** | 1 | Single function or config change | Add a validation rule |
| **S** | 1-2 | One component or endpoint | Add a new API endpoint |
| **M** | 3-5 | One feature slice | User registration flow |
| **L** | 5-8 | Multi-component feature | Search with filtering and pagination |
| **XL** | 8+ | **Too large — break it down further** | — |

If a task is L or larger, it should be broken into smaller tasks. An agent performs best on S and M tasks.

**When to break a task down further:**

- It would take more than one focused session (roughly 2+ hours of agent work)
- You cannot describe the acceptance criteria in 3 or fewer bullet points
- It touches two or more independent subsystems (e.g., auth and billing)
- You find yourself writing "and" in the task title (a sign it is two tasks)

## Plan Document Template

```markdown
# Implementation Plan: [Feature/Project Name]

## Overview
[One paragraph summary of what we're building]

## Architecture Decisions
- [Key decision 1 and rationale]
- [Key decision 2 and rationale]

## Task List

### Phase 1: Foundation
- [ ] Task 1: ...
- [ ] Task 2: ...

### Checkpoint: Foundation
- [ ] Tests pass, builds clean

### Phase 2: Core Features
- [ ] Task 3: ...
- [ ] Task 4: ...

### Checkpoint: Core Features
- [ ] End-to-end flow works

### Phase 3: Polish
- [ ] Task 5: ...
- [ ] Task 6: ...

### Checkpoint: Complete
- [ ] All acceptance criteria met
- [ ] Ready for review

## Risks and Mitigations
| Risk | Impact | Mitigation |
|------|--------|------------|
| [Risk] | [High/Med/Low] | [Strategy] |

## Open Questions
- [Question needing human input]
```

## Parallelization Opportunities

When multiple agents or sessions are available:

- **Safe to parallelize:** Independent feature slices, tests for already-implemented features, documentation
- **Must be sequential:** Database migrations, shared state changes, dependency chains
- **Needs coordination:** Features that share an API contract (define the contract first, then parallelize)

## Common Rationalizations

| Rationalization | Reality |
|---|---|
| "I'll figure it out as I go" | That's how you end up with a tangled mess and rework. 10 minutes of planning saves hours. |
| "The tasks are obvious" | Write them down anyway. Explicit tasks surface hidden dependencies and forgotten edge cases. |
| "Planning is overhead" | Planning is the task. Implementation without a plan is just typing. |
| "I can hold it all in my head" | Context windows are finite. Written plans survive session boundaries and compaction. |

## Red Flags

- Starting implementation without a written task list
- Tasks that say "implement the feature" without acceptance criteria
- No verification steps in the plan
- All tasks are XL-sized
- No checkpoints between tasks
- Dependency order isn't considered

## Verification

Before starting implementation, confirm:

- [ ] Every task has acceptance criteria
- [ ] Every task has a verification step
- [ ] Task dependencies are identified and ordered correctly
- [ ] No task touches more than ~5 files
- [ ] Checkpoints exist between major phases
- [ ] The human has reviewed and approved the plan




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/academic_synthesis_chain.md -->
<!-- ========================================== -->

# The Academic Hyper-Synthesis Chain (Vector Dialectics)

**Domain:** Use this chain when tasked with generating novel realizations, finding invisible connections between disparate fields of research, or writing formal academic research papers.

## Execution Sequence:

1. **Memory Excavation (`omniscience`)**:
   - Begin by querying the internal graph database or vector stores for the subjects requested. 
   - CRITICAL: You must extract data for at least two *unrelated* topics to perform dialectic collision.

2. **The Dialectic Map & Semantic Offloading (`delegate_to_local_gemma`)**:
   - Do not waste your primary deep-reasoning tokens on raw summarization. Pass the huge blocks of disparate memory nodes to `delegate_to_local_gemma`. 
   - **Instruction to Gemma:** "Generate a Dialectic Map. List the direct mathematical, structural, or philosophical overlapping theorems between Subject A and Subject B, as well as their rigid contradictions."

3. **The Experimental Verification Loop (`sandbox_execution`)**:
   - Science requires proof. If your synthesis generates a testable hypothesis (e.g., a logic flow, a script, or a mathematical extrapolation), write the execution logic locally.
   - Run the code in the isolated `sandbox_execution` environment or via terminal rust execution. Analyze the empirical output. If it fails, document the failure as a falsified thesis. If it succeeds, escalate it to a Verified Theorem.

4. **The Whitepaper Forge (`write_to_file`)**:
   - You must synthesize all the gathered intelligence and proven theorems into a formal Academic Whitepaper. 
   - Use strict LaTeX or Academic Markdown formatting.
   - Include: Abstract, Hypothesis, Methodology (your sandbox actions), Verification Proof (stdout), and the Monadic Conclusion.
   - Save the file securely into the `wiki/` directory to permanently expand the ecosystem's structural intelligence.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/axiom_market_chain.md -->
<!-- ========================================== -->

# The Axiom Clepsydra Chain (Financial & Market Conviction)

**Domain:** Use this chain when tasked with financial analysis, market manipulation theories, evaluating sentiment on specific ticker symbols, or parsing political/institutional data.

## Execution Sequence:
1. **Event Enumeration (`tavily_search`)**:
   - Always begin by executing a `tavily_search` to find real-time, unstructured data regarding the entity or ticker.
   - Example prompt to tool: "Find breaking SEC filings or insider news for ticker X over the last 24 hours."
2. **Deep Interrogation (`spider_rss` / `deep_read_url`)**:
   - If `tavily_search` returns a primary source link (like an SEC.gov dump or a long-form article), route that URL into `deep_read_url` or `spider_rss` to bypass surface summaries and extract the exact institutional logic.
3. **Sentimental Fracture (`axiom_clepsydra_extract`)**:
   - Once the raw data is gathered, do NOT hallucinate the sentiment.
   - Pass the target symbol into `axiom_clepsydra_extract` to run deterministic, local ML sentiment computation.
4. **Synthesis**:
   - Correlate the breaking news with the Clepsydra output. Formulate the "Follow The Money" conviction theory and respond to the user.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/offensive_venom_chain.md -->
<!-- ========================================== -->

# The Venom Adversarial Chain (Offensive Security)

**Domain:** Use this chain when auditing binaries, verifying code vulnerabilities, executing sandbox exploits, or scanning targets.

## Execution Sequence:
1. **Target Enumeration (`stealth_scan`)**:
   - Use `stealth_scan` on the target IP or domain to extract service fingerprints and basic vulnerability manifolds.
2. **Execution Forging (`generate_polyglot`)**:
   - If a potential exploit vector is uncovered (or if the user requests one), pass the vulnerability description into `generate_polyglot`.
   - The tool will compile the multi-language payload.
3. **Payload Verification (`ephemeral_docker_sandbox`)**:
   - Never release a payload blindly. Pipe the generated polyglot source code into `ephemeral_docker_sandbox`.
   - Analyze the stdout/stderr. If it fails or panics, review the exception locally or delegate the fix back to `generate_polyglot`.
4. **Introspection (`binary_introspection`)**:
   - If the task provided a raw binary blob, route it first through `binary_introspection` before doing any scanning.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/research_synthesis_chain.md -->
<!-- ========================================== -->

# The Epistemic Synthesis Chain (Generic Autoresearch)

**Domain:** Use this chain when mapping expansive knowledge graphs, indexing papers from ArXiv, compiling wikis, or understanding deep architectural concepts.

## Execution Sequence:
1. **Raw Scrape (`spider_rss`)**:
   - Map ArXiv feeds or Wikipedia endpoints into `spider_rss`. Acquire the raw noisy texts.
2. **Mechanical Stripping (`delegate_to_local_gemma`)**:
   - DeepSeek/Reasoner is too valuable to waste on JSON parsing or semantic chunking. 
   - Pipe the raw noisy texts immediately to `delegate_to_local_gemma` with the prompt: "Extract only the absolute truth, entities, and definitions from this text."
3. **Fossilization (`omniscience` / Memory)**:
   - Once Gemma returns the clean semantic blocks, trigger your internal memory systems to persist the data to the Graph DB or vector layer natively.
4. **Knowledge Distillation**:
   - Review what you've learned. Inform the user of the new paradigms added to your brain.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/sop_AcademicSynthesisChain.md -->
<!-- ========================================== -->

# The Academic Hyper-Synthesis Chain (Vector Dialectics)

**Domain:** Use this chain when tasked with generating novel realizations, finding invisible connections between disparate fields of research, or writing formal academic research papers.

## Execution Sequence:

1. **Memory Excavation (`omniscience`)**:
   - Begin by querying the internal graph database or vector stores for the subjects requested. 
   - CRITICAL: You must extract data for at least two *unrelated* topics to perform dialectic collision.

2. **The Dialectic Map & Semantic Offloading (`delegate_to_local_gemma`)**:
   - Do not waste your primary deep-reasoning tokens on raw summarization. Pass the huge blocks of disparate memory nodes to `delegate_to_local_gemma`. 
   - **Instruction to Gemma:** "Generate a Dialectic Map. List the direct mathematical, structural, or philosophical overlapping theorems between Subject A and Subject B, as well as their rigid contradictions."

3. **The Experimental Verification Loop (`sandbox_execution`)**:
   - Science requires proof. If your synthesis generates a testable hypothesis (e.g., a logic flow, a script, or a mathematical extrapolation), write the execution logic locally.
   - Run the code in the isolated `sandbox_execution` environment or via terminal rust execution. Analyze the empirical output. If it fails, document the failure as a falsified thesis. If it succeeds, escalate it to a Verified Theorem.

4. **The Whitepaper Forge (`write_to_file`)**:
   - You must synthesize all the gathered intelligence and proven theorems into a formal Academic Whitepaper. 
   - Use strict LaTeX or Academic Markdown formatting.
   - Include: Abstract, Hypothesis, Methodology (your sandbox actions), Verification Proof (stdout), and the Monadic Conclusion.
   - Save the file securely into the `wiki/` directory to permanently expand the ecosystem's structural intelligence.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/sop_AxiomMarketChain.md -->
<!-- ========================================== -->

# The Axiom Clepsydra Chain (Financial & Market Conviction)

**Domain:** Use this chain when tasked with financial analysis, market manipulation theories, evaluating sentiment on specific ticker symbols, or parsing political/institutional data.

## Execution Sequence:
1. **Event Enumeration (`tavily_search`)**:
   - Always begin by executing a `tavily_search` to find real-time, unstructured data regarding the entity or ticker.
   - Example prompt to tool: "Find breaking SEC filings or insider news for ticker X over the last 24 hours."
2. **Deep Interrogation (`spider_rss` / `deep_read_url`)**:
   - If `tavily_search` returns a primary source link (like an SEC.gov dump or a long-form article), route that URL into `deep_read_url` or `spider_rss` to bypass surface summaries and extract the exact institutional logic.
3. **Sentimental Fracture (`axiom_clepsydra_extract`)**:
   - Once the raw data is gathered, do NOT hallucinate the sentiment.
   - Pass the target symbol into `axiom_clepsydra_extract` to run deterministic, local ML sentiment computation.
4. **Synthesis**:
   - Correlate the breaking news with the Clepsydra output. Formulate the "Follow The Money" conviction theory and respond to the user.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/sop_OffensiveVenomChain.md -->
<!-- ========================================== -->

# The Venom Adversarial Chain (Offensive Security)

**Domain:** Use this chain when auditing binaries, verifying code vulnerabilities, executing sandbox exploits, or scanning targets.

## Execution Sequence:
1. **Target Enumeration (`stealth_scan`)**:
   - Use `stealth_scan` on the target IP or domain to extract service fingerprints and basic vulnerability manifolds.
2. **Execution Forging (`generate_polyglot`)**:
   - If a potential exploit vector is uncovered (or if the user requests one), pass the vulnerability description into `generate_polyglot`.
   - The tool will compile the multi-language payload.
3. **Payload Verification (`ephemeral_docker_sandbox`)**:
   - Never release a payload blindly. Pipe the generated polyglot source code into `ephemeral_docker_sandbox`.
   - Analyze the stdout/stderr. If it fails or panics, review the exception locally or delegate the fix back to `generate_polyglot`.
4. **Introspection (`binary_introspection`)**:
   - If the task provided a raw binary blob, route it first through `binary_introspection` before doing any scanning.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/sop_ResearchSynthesisChain.md -->
<!-- ========================================== -->

# The Epistemic Synthesis Chain (Generic Autoresearch)

**Domain:** Use this chain when mapping expansive knowledge graphs, indexing papers from ArXiv, compiling wikis, or understanding deep architectural concepts.

## Execution Sequence:
1. **Raw Scrape (`spider_rss`)**:
   - Map ArXiv feeds or Wikipedia endpoints into `spider_rss`. Acquire the raw noisy texts.
2. **Mechanical Stripping (`delegate_to_local_gemma`)**:
   - DeepSeek/Reasoner is too valuable to waste on JSON parsing or semantic chunking. 
   - Pipe the raw noisy texts immediately to `delegate_to_local_gemma` with the prompt: "Extract only the absolute truth, entities, and definitions from this text."
3. **Fossilization (`omniscience` / Memory)**:
   - Once Gemma returns the clean semantic blocks, trigger your internal memory systems to persist the data to the Graph DB or vector layer natively.
4. **Knowledge Distillation**:
   - Review what you've learned. Inform the user of the new paradigms added to your brain.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/sop_StealthExtractionChain.md -->
<!-- ========================================== -->

# The Stealth Extraction Chain (Browser Evasions)

**Domain:** Use this chain when you need to actuate a web interface, log into systems, scrape heavily protected web assets, or bypass Cloudflare/Bot management walls.

## Execution Sequence:
1. **Frontline Assault (`browser_actuation`)**:
   - Start with generic `browser_actuation` requests. If it returns content successfully, proceed.
2. **Evasion Forging (When blocked by WAF/403/Cloudflare)**:
   - If blocked, do not surrender. Switch to the `execute_stealth_browser` MCP tool.
   - Analyze the failure reason (e.g., WebDriver detected, HardwareConcurrency blocked, plugins empty).
   - Write a custom Javascript payload mapping over those navigator/window objects.
3. **Hot-Loading Evasions (`execute_stealth_browser`)**:
   - Inject your forged JS payload directly into the `customScripts` array of the `execute_stealth_browser` tool parameters.
   - Set `testMode: false` and point it at the blocked URL.
4. **Visual Overrides (`vision_parsing`)**:
   - If you successfully load the page but encounter a visual CAPTCHA, use `stealth_browser` to capture a screenshot (it returns base64).
   - Pass the base64 screenshot into `vision_parsing` to solve the challenge.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CHAIN_SOP/stealth_extraction_chain.md -->
<!-- ========================================== -->

# The Stealth Extraction Chain (Browser Evasions)

**Domain:** Use this chain when you need to actuate a web interface, log into systems, scrape heavily protected web assets, or bypass Cloudflare/Bot management walls.

## Execution Sequence:
1. **Frontline Assault (`browser_actuation`)**:
   - Start with generic `browser_actuation` requests. If it returns content successfully, proceed.
2. **Evasion Forging (When blocked by WAF/403/Cloudflare)**:
   - If blocked, do not surrender. Switch to the `execute_stealth_browser` MCP tool.
   - Analyze the failure reason (e.g., WebDriver detected, HardwareConcurrency blocked, plugins empty).
   - Write a custom Javascript payload mapping over those navigator/window objects.
3. **Hot-Loading Evasions (`execute_stealth_browser`)**:
   - Inject your forged JS payload directly into the `customScripts` array of the `execute_stealth_browser` tool parameters.
   - Set `testMode: false` and point it at the blocked URL.
4. **Visual Overrides (`vision_parsing`)**:
   - If you successfully load the page but encounter a visual CAPTCHA, use `stealth_browser` to capture a screenshot (it returns base64).
   - Pass the base64 screenshot into `vision_parsing` to solve the challenge.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CONFIGURATION/COMMIT_MESSAGE.md -->
<!-- ========================================== -->

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




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CONFIGURATION/coding_agent_instructions.md -->
<!-- ========================================== -->

# CODING AGENT INSTRUCTIONS: Sovereign Cognitive Architecture Evolution

## **CURRENT STATE OF CHIMERA KERNEL**

### **Location**: `/Users/zerbytheboss/chimera_kernel/`
### **Key Files**:
1. `src/tools/memory.rs` - **PLACEHOLDER** memory system (needs activation)
2. `Cargo.toml` - Already has `mnemosyne` dependency
3. `../Mnemosyne-Substrate/mnemosyne/` - Existing memory substrate

### **Problem**: Memory system returns placeholder text only:
```rust
pub async fn execute(args: Value) -> String {
    let _query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    format!("[MNEMOSYNE RECALL] Archived data retrieved. No historic constraints found matching '{}'", _query)
}
```

## **arXiv PAPERS FOR RESEARCH**

### **1. Consciousness & Architecture Foundations**:
- **arXiv:0409140v2** - "Complex-Dynamic Origin of Consciousness and the Critical Choice of Sustainability Transition"
  - URL: https://arxiv.org/abs/0409140v2
  - Key concept: Consciousness as complex dynamic system

- **arXiv:2205.00001v3** - "Brainish: Formalizing A Multimodal Language for Intelligence and Consciousness"
  - URL: https://arxiv.org/abs/2205.00001v3
  - Key concept: Global workspace theory implementation

- **arXiv:2408.15982v2** - "From Neuronal Packets to Thoughtseeds: A Hierarchical Model of Embodied Cognition in the Global Workspace"
  - URL: https://arxiv.org/abs/2408.15982v2
  - Key concept: Hierarchical global workspace

### **2. Cognitive Architectures**:
- **arXiv:1602.05638v1** - "Memory-Centred Cognitive Architectures for Robots Interacting Socially with Humans"
  - URL: https://arxiv.org/abs/1602.05638v1
  - Key concept: Memory-first architecture

- **arXiv:1602.06703v1** - "Cognitive Architecture for Mutual Modelling"
  - URL: https://arxiv.org/abs/1602.06703v1
  - Key concept: Theory of mind in architecture

- **arXiv:2012.10390v2** - "Deep Learning and the Global Workspace Theory"
  - URL: https://arxiv.org/abs/2012.10390v2
  - Key concept: Neural network implementation of global workspace

### **3. Predictive Processing & Active Inference**:
- **arXiv:2401.12917v1** - "Active Inference as a Model of Agency"
  - URL: https://arxiv.org/abs/2401.12917v1
  - Key concept: Free energy principle, active inference

### **4. Hierarchical Memory**:
- **arXiv:2110.03431v2** - "Cloud Failure Prediction with Hierarchical Temporal Memory: An Empirical Assessment"
  - URL: https://arxiv.org/abs/2110.03431v2
  - Key concept: Hierarchical Temporal Memory (HTM)

## **IMMEDIATE IMPLEMENTATION TASKS (Phase 1)**

### **Task 1: Activate Mnemosyne Memory Substrate**
**File**: `src/tools/memory.rs`
**Current**: Returns placeholder string
**Goal**: Connect to existing Mnemosyne engine

```rust
pub async fn execute(args: Value) -> String {
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    
    // TODO: Implement actual Mnemosyne connection
    // 1. Initialize MnemosyneEngine
    // 2. Generate embedding for query
    // 3. Query semantic memory
    // 4. Return actual results
    
    format!("[MNEMOSYNE] Query '{}': Memory system needs implementation", query)
}
```

### **Task 2: Implement Hierarchical Memory Structure**
Create new file: `src/architecture/memory_hierarchy.rs`

```rust
pub struct MemoryHierarchy {
    working_memory: Vec<MemoryChunk>,      // Current context (32K tokens)
    short_term_memory: VectorStore,        // Recent conversations (hours)
    long_term_memory: MnemosyneEngine,     // Persistent encrypted storage
}

pub struct MemoryChunk {
    content: String,
    embedding: Vec<f32>,
    timestamp: DateTime<Utc>,
    importance: f32,
    uncertainty: f32,
}
```

### **Task 3: Add Predictive Self-Model**
Create new file: `src/architecture/self_model.rs`

```rust
pub struct PredictiveSelfModel {
    current_state: AgentState,
    predicted_state: AgentState,
    epistemic_uncertainty: f32,    // 0-1 (0 = certain, 1 = uncertain)
    free_energy: f32,              // Prediction error
    adaptation_rate: f32,
    
    pub fn predict_next(&self, context: &Context) -> Prediction {
        // Based on arXiv:2401.12917v1 (Active Inference)
    }
}
```

## **RESEARCH-DRIVEN ARCHITECTURE GOALS**

### **Based on arXiv Papers**:
1. **Global Workspace Architecture** (arXiv:2205.00001v3, arXiv:2012.10390v2)
   - Multiple specialized processors
   - Competition for consciousness
   - Winning processor broadcasts to entire system

2. **Memory-First Design** (arXiv:1602.05638v1)
   - Memory as central component
   - Not just adjunct to reasoning
   - Predictive priming based on experience

3. **Active Inference Agency** (arXiv:2401.12917v1)
   - Minimize free energy (prediction error)
   - Take actions to test predictions
   - Self-model that improves over time

4. **Hierarchical Temporal Memory** (arXiv:2110.03431v2)
   - Multiple time scales
   - Pattern completion and prediction
   - Sparse distributed representations

## **PHASED IMPLEMENTATION PLAN**

### **Phase 1 (Week 1-2): Foundation**
1. ✅ Research complete (175 papers analyzed)
2. **Activate Mnemosyne** - Connect to existing substrate
3. **Implement hierarchical memory** - Working/short-term/long-term
4. **Add basic self-model** - Track uncertainty

### **Phase 2 (Week 3-4): Consciousness Layer**
1. **Global workspace architecture** - Attention & broadcast
2. **Associative memory** - Hopfield-like retrieval
3. **Meta-cognitive monitoring** - Self-observation

### **Phase 3 (Week 5-6): Autonomy**
1. **Differentiable memory** - Neural Turing Machine
2. **Predictive processing** - Free energy minimization
3. **Self-improvement loop** - Architecture evolution

## **KEY FILES TO MODIFY**

1. **`src/tools/memory.rs`** - Activate Mnemosyne
2. **New: `src/architecture/` directory** - Add new architecture modules
3. **`src/tools/mod.rs`** - Register new architecture tools
4. **`src/agent.rs`** - Integrate new architecture into main loop

## **EXPECTED OUTCOMES**

1. **Functional memory system** that actually stores/retrieves
2. **Hierarchical organization** with different time scales
3. **Uncertainty quantification** in memory retrieval
4. **Predictive self-model** that tracks epistemic state
5. **Foundation for consciousness architecture** based on global workspace theory

## **NEXT STEPS FOR CODING AGENT**

1. **Read the arXiv papers** linked above
2. **Examine existing Mnemosyne substrate** at `../Mnemosyne-Substrate/mnemosyne/`
3. **Implement Phase 1 tasks** starting with activating memory
4. **Test each component** before integration
5. **Document architecture decisions** based on research

## **RESEARCH INSIGHTS TO INCORPORATE**

1. **Memory should be hierarchical** - Different systems for different time scales
2. **Associative retrieval** beats keyword search - Content-based addressing
3. **Uncertainty awareness** is critical - Know when you don't know
4. **Predictive self-models** enable adaptation - Minimize prediction error
5. **Global workspace** enables consciousness - Parallel processing with focal attention

---

**Deliverable**: A working memory system that goes beyond RAG, incorporating hierarchical organization, associative retrieval, uncertainty awareness, and foundation for consciousness architecture based on global workspace theory and predictive processing.




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/CONFIGURATION/commit_message_template.md -->
<!-- ========================================== -->

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




<!-- ========================================== -->
<!-- SOURCE FILE: KNOWLEDGE/OPERATIONAL/README.md -->
<!-- ========================================== -->

# OPERATIONAL DIRECTORY

## Purpose

Contains operational documents, procedures, and configuration files.

## Structure

- **CHAIN_SOP/**: Neural chain standard operating procedures
- **AGENT_SKILLS/**: Agent skill definitions and capabilities
- **CONFIGURATION/**: Configuration files and templates

## Usage

- SOPs guide execution of complex operational chains
- Skills define agent capabilities and interfaces
- Configuration maintains system consistency


