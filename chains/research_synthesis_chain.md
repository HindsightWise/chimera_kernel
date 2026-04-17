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
