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

## 4. IMMEDIATE NEXT ACTIONS UPON INVOCATION

If you are an Agent reading this file because a user triggered a complex objective:
1. **HALT AND ASSESS:** Do not perform any execution yet.
2. **SELECT THE CHAIN:** Identify which 3-4 MCP tools from taxonomy listed above are required.
3. **WRITE A LOCAL PLAN:** Output a 5-step checklist using your Boolean Task Taxonomy (`- [ ]`) natively in the chat.
4. **EXECUTE NODE 1:** Perform the very first tool call, save the stdout natively to a `.md` file, and await the user HUD to continue.
