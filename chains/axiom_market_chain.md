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
