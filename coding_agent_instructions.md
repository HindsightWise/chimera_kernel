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
