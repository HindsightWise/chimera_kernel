# Telemetry Spam Vector Analysis

## Investigation Date: Phase 5.0 Resumption
## File: chimera_state.log
## Size: 8.0MB, 122,895 lines

## Key Findings

### 1. Primary Spam Vector: `log_state!` Macro
**Location**: `src/agent.rs`, line 59
**Behavior**: Writes to `chimera_state.log` file unconditionally
**Impact**: 
- Every tool invocation logged as `[OUROBOROS TRIGGER]`
- Every tool return logged as `[TOOL RETURN]`
- Every monad actualization logged
- Unbounded file growth (8MB with 122k lines)

### 2. Secondary Spam Vector: Telegram Integration
**Location**: `src/agent.rs`, lines 336-343
**Behavior**: When `tg_config` provided, spawns async task to send messages
**Impact**: Every monad output sent to external Telegram service
**Risk**: External service spam, privacy concerns

### 3. Tertiary Spam Vector: UI Channel Pipeline
**Location**: `src/lib.rs`, `UI_LOG_TX` global channel
**Behavior**: All `log_ui!`, `log_ui_err!`, `log_verbose!` calls go through UI channel
**Impact**: Potential UI congestion, though less critical than file/network spam

### 4. Existing Mitigation Infrastructure
**`VERBOSE_MODE`**: Static boolean in `src/lib.rs`, line 19
**`log_verbose!` macro**: Respects `VERBOSE_MODE` (lines 22-33 in `src/lib.rs`)
**Current Usage**: `log_verbose!` used in `agent.rs` but `log_state!` ignores verbosity

## Code Flow Analysis

### Logging Pipeline:
1. **Tool Invocation**: 
   - `log_verbose!` call (respects VERBOSE_MODE)
   - `log_state!` call (ignores verbosity, writes to file)

2. **Tool Return**:
   - `log_verbose!` call (respects VERBOSE_MODE)  
   - `log_state!` call (ignores verbosity, writes to file)

3. **Monad Output**:
   - `log_ui!` call (goes to UI channel)
   - `log_state!` call (ignores verbosity, writes to file)
   - Telegram broadcast (if configured)

### Configuration Missing:
- No environment variables for log level control
- No log rotation mechanism
- No Telegram broadcast filtering

## Recommended Fixes

### Priority 1: Fix `log_state!` Macro
- Add verbosity check using `VERBOSE_MODE`
- Or implement log level filtering
- Consider moving to `log_verbose!` pattern

### Priority 2: Telegram Broadcast Controls
- Add rate limiting (e.g., max 1 message per 30 seconds)
- Add importance filtering (only send important messages)
- Make configurable via environment variable

### Priority 3: Log Rotation
- Implement size-based rotation for `chimera_state.log`
- Consider time-based rotation (daily logs)
- Add compression for old logs

### Priority 4: Configuration System
- Add `CHIMERA_LOG_LEVEL` environment variable
- Add `CHIMERA_TELEGRAM_ENABLED` with filtering options
- Add `CHIMERA_LOG_MAX_SIZE` for file rotation

## Technical Details

### Current `log_state!` Macro:
```rust
macro_rules! log_state {
    ($entry:expr) => {
        {
            use tokio::io::AsyncWriteExt;
            if let Ok(mut file) = tokio::fs::OpenOptions::new().create(true).append(true).open("chimera_state.log").await {
                let _ = file.write_all(format!("{}\n", $entry).as_bytes()).await;
            }
        }
    };
}
```

### Current Telegram Integration:
```rust
if let Some((ref token, chat_id)) = tg_config {
    let tk = token.clone();
    let cid = chat_id.clone();
    let txt = content.clone();
    tokio::spawn(async move {
        crate::telegram::send_message(&tk, cid, &txt).await;
    });
}
```

## Risk Assessment
- **CRITICAL**: Unbounded file growth (8MB+)
- **HIGH**: External service spam (Telegram)
- **MEDIUM**: UI channel congestion  
- **LOW**: Performance impact from logging

## Implementation Strategy
1. Start with `log_state!` verbosity fix (quick win)
2. Add Telegram rate limiting
3. Implement log rotation
4. Add comprehensive configuration system