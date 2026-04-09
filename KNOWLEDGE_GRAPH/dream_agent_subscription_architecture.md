# Dream-Agent Subscription Architecture Specification

## Architectural Overview

The Cerebrospinal Fluid model requires agents to subscribe to the SYSTEM.DREAM topic to receive synthesized insights from the Sensory Drift subsystem. Currently, the MessageBus exists as the circulatory system, Sensory Drift generates dreams, but agents lack the synaptic connections to receive and process them.

## Current State Analysis

### Existing Components ✅

1. **MessageBus** (`src/architecture/message_bus.rs`):
   - Complete pub/sub implementation
   - Supports topics including "SYSTEM.DREAM"
   - Methods: `subscribe()`, `unsubscribe()`, `publish()`, `receive()`, `has_messages()`

2. **Sensory Drift** (`src/architecture/sensory_drift.rs`):
   - Hourly dream synthesis via Oracle (local Gemma)
   - Publishes to "SYSTEM.DREAM" topic
   - TTL: 3600 seconds, Priority: 200

3. **Agent Trait** (`src/architecture/agent_trait.rs`):
   - Basic agent interface with `execute_task()`, `health_check()`, `status()`
   - BaseAgent implementation
   - Missing message handling methods

4. **Specialized Agents** (`src/architecture/specialized_agents.rs`):
   - 12 agent factory functions
   - Creates BaseAgent instances with capability sets
   - No subscription or dream processing logic

5. **MultiAgentKernel** (`src/architecture/multi_agent_kernel.rs`):
   - Spawns sensory drift dream cycle
   - Missing subscription wiring

## Required Modifications

### Priority 1: Agent Trait Extension

**File: `src/architecture/agent_trait.rs`**

```rust
// Add imports at top
use std::sync::Arc;
use super::message_bus::{MessageBus, Message};

// Extend Agent trait
#[async_trait]
pub trait Agent: Send + Sync {
    // Existing methods...
    
    // New methods for message handling
    async fn subscribe_to_topics(&self, message_bus: Arc<MessageBus>) -> anyhow::Result<()>;
    async fn handle_message(&mut self, message: Message) -> anyhow::Result<()>;
    async fn process_dream(&mut self, dream_content: &str) -> anyhow::Result<()>; // Optional helper
}

// Extend BaseAgent implementation
impl BaseAgent {
    pub async fn default_subscribe_to_topics(&self, message_bus: Arc<MessageBus>) -> anyhow::Result<()> {
        // Base agents subscribe to generic topics
        message_bus.subscribe(self.id(), "SYSTEM.HEALTH").await?;
        message_bus.subscribe(self.id(), "SYSTEM.STATUS").await?;
        Ok(())
    }
    
    pub async fn default_handle_message(&mut self, message: Message) -> anyhow::Result<()> {
        // Default message handling (can be overridden)
        match message.topic.as_str() {
            "SYSTEM.HEALTH" => {
                // Handle health check requests
                if self.health_check().await {
                    // Acknowledge health
                }
            }
            "SYSTEM.STATUS" => {
                // Handle status requests
            }
            _ => {
                // Ignore unknown topics
            }
        }
        Ok(())
    }
}
```

### Priority 2: Specialized Agent Extensions

**File: `src/architecture/specialized_agents.rs`**

Create new structs that wrap BaseAgent and add dream processing:

```rust
pub struct ReasoningAgent {
    base: BaseAgent,
    dream_insights: Vec<String>,
    hypothesis_buffer: Vec<String>,
}

pub struct ResearchAgent {
    base: BaseAgent,
    search_queue: VecDeque<String>,
    validation_results: HashMap<String, bool>,
}

pub struct TradingAgent {
    base: BaseAgent,
    risk_adjustments: Vec<f64>,
    market_hypotheses: Vec<String>,
}

pub struct ContextManagementAgent {
    base: BaseAgent,
    dream_archive: Vec<DreamRecord>,
    relevance_scores: HashMap<Uuid, f32>,
}

pub struct SystemManagementAgent {
    base: BaseAgent,
    dream_quality_metrics: HashMap<Uuid, DreamMetrics>,
    system_load_history: Vec<SystemLoad>,
}
```

Each specialized agent should:
1. Subscribe to SYSTEM.DREAM topic
2. Implement `handle_message()` with dream processing logic
3. Override `subscribe_to_topics()` for agent-specific subscriptions
4. Add agent-specific dream reaction methods

### Priority 3: Kernel Subscription Wiring

**File: `src/architecture/multi_agent_kernel.rs`**

Add subscription initialization:

```rust
impl MultiAgentKernel {
    // ... existing code ...
    
    pub async fn initialize_subscriptions(&self) -> anyhow::Result<()> {
        let agents = {
            let registry = self.registry.read().await;
            // Need to add get_all_agents_mut() to AgentRegistry
            // For now, we'll work with IDs and agent factory
            registry.all_agent_ids().await
        };
        
        // Initialize subscriptions for each agent
        // This requires agent registry to support mutable agent access
        // Or agents to implement Arc<Mutex<dyn Agent>> pattern
        
        Ok(())
    }
}
```

## Architectural Challenges

1. **Mutable Agent Access**: AgentRegistry stores `Box<dyn Agent>` which complicates mutable access for message handling.

2. **Async Trait Constraints**: Need `async-trait` crate for async methods in traits.

3. **Backward Compatibility**: Must maintain all existing tests.

4. **Message Loop Integration**: Agents need background tasks to process incoming messages.

## Recommended Implementation Strategy

### Phase 1: Trait Extension (Non-breaking)
1. Add new async trait methods with default empty implementations
2. Update BaseAgent with default implementations
3. Verify all tests still pass

### Phase 2: Specialized Agent Creation
1. Create new agent structs that implement enhanced Agent trait
2. Factory functions return new specialized agents
3. Maintain old factory functions for backward compatibility

### Phase 3: Subscription Wiring
1. Add subscription initialization to kernel
2. Implement message processing loops
3. Add integration tests

### Phase 4: Dream Processing Logic
1. Implement agent-specific dream reactions
2. Add dream-to-task conversion
3. Add comprehensive testing

## Testing Requirements

**New Integration Tests:**
1. Dream subscription and delivery
2. Agent message processing
3. Multi-agent dream coordination
4. Dream-inspired task creation

**Preserve Existing Tests:**
- All 18 current tests must continue to pass

## Success Metrics

1. ✅ All existing tests pass
2. ✅ Agents receive SYSTEM.DREAM messages
3. ✅ Agents process dreams based on capabilities
4. ✅ Dream insights convert to actionable tasks
5. ✅ Cerebrospinal Fluid model fully operational