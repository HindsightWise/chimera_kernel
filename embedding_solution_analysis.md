# EMBEDDING SOLUTION FOR MNEMOSYNE INTEGRATION

## Current State Analysis:

### 1. **Mnemosyne Substrate Status**:
- ✅ **Exists at**: `../Mnemosyne-Substrate/mnemosyne/`
- ✅ **Compiled library**: `target/release/libmemcore.dylib`
- ✅ **Python environment**: Has torch and sentence-transformers
- ❌ **Not installed as Python module**: Can't `import mnemosyne`

### 2. **Embedding Generation Options**:

#### **Option A: Lightweight Local Embedding (Recommended for Phase 1)**
```rust
// In src/tools/memory.rs
pub async fn execute(args: Value) -> String {
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    
    // Phase 1: Simple deterministic embedding
    let dummy_embedding = vec![0.1; 384]; // 384-dim zero vector
    
    // TODO: Connect to Mnemosyne engine
    // For now, return placeholder with plan
    format!("[MNEMOSYNE PROTOTYPE] Query: '{}' | Embedding: {} dimensions | Status: Memory system activation in progress", 
            query, dummy_embedding.len())
}
```

#### **Option B: Call Python Embedding Service**
- **Pro**: Uses existing sentence-transformers in Mnemosyne venv
- **Con**: Requires inter-process communication
- **Implementation**: Spawn Python subprocess or HTTP server

#### **Option C: Build Lightweight Rust Embedder**
- **Pro**: Pure Rust, no Python dependency
- **Con**: Need to implement/port embedding model
- **Models**: `rust-bert`, `candle`, or simple TF-IDF

### 3. **Recommended Phased Approach**:

#### **Phase 1 (Immediate): Dummy Embeddings + Architecture**
```rust
struct HierarchicalMemory {
    working_memory: Vec<MemoryChunk>,
    short_term_cache: HashMap<String, MemoryChunk>,
    // long_term: Will connect to Mnemosyne
}

// Use deterministic dummy embeddings
fn dummy_embedding(text: &str) -> Vec<f32> {
    // Simple hash-based deterministic vector
    vec![0.1; 384]
}
```

#### **Phase 2 (Short-term): Connect to Mnemosyne Python**
- Spawn Python subprocess from Mnemosyne venv
- Send text, receive embeddings
- Store in LanceDB via Mnemosyne engine

#### **Phase 3 (Long-term): Native Rust Embedder**
- Integrate `rust-bert` or `candle` for embeddings
- Pure Rust pipeline
- Better performance, no Python dependency

### 4. **Immediate Action Plan**:

1. **Test Mnemosyne Python integration**:
```bash
cd ../Mnemosyne-Substrate/mnemosyne
./venv/bin/python3 -c "from sentence_transformers import SentenceTransformer; model = SentenceTransformer('all-MiniLM-L6-v2'); print('Model works')"
```

2. **Implement Phase 1 in `src/tools/memory.rs`**:
   - Add hierarchical memory structure
   - Use dummy embeddings
   - Prepare interface for real Mnemosyne

3. **Create embedding bridge** (Phase 2):
   - Rust → Python subprocess communication
   - JSON over stdin/stdout or temporary files

### 5. **Technical Details**:

#### **Embedding Dimensions**: 384 (all-MiniLM-L6-v2)
#### **Normalization**: Cosine similarity expects unit vectors
#### **Storage**: LanceDB expects `Vec<f32>`
#### **Query**: Mnemosyne expects JSON string of `Vec<f32>`

#### **Example JSON for Mnemosyne**:
```json
[0.1, 0.2, 0.3, ..., 0.384]  // 384 floats
```

### 6. **Decision**:

**Start with Option A (dummy embeddings)** for Phase 1 because:
1. Gets architecture in place immediately
2. Allows testing memory hierarchy
3. Doesn't block on Python integration issues
4. We can swap dummy for real embeddings later

**Phase 1 delivers**:
- Working memory system architecture
- Hierarchical structure (working/short-term/long-term)
- Interface ready for real embeddings
- Foundation for consciousness architecture
