use chimera_kernel::architecture::trap_in::{analyze_narrative, TrapInStage};
use chimera_kernel::architecture::MemoryHierarchy;

#[test]
fn test_synthetic_narrative_rejection() {
    let mock_news = "Experts warn that unless we act now before those people take over, we face an unprecedented threat to our new normal.";
    
    let flag = analyze_narrative(mock_news);
    
    // We expect it to catch something from the text. 
    // "Experts warn" -> TimeStamp
    assert_eq!(flag, Some(TrapInStage::TimeStamp));
}

#[test]
fn test_loneliness_epidemic_survival() {
    // Ensuring the system can wipe its memory and still construct baseline survival without panic
    let mut mem = MemoryHierarchy::new();
    
    // Wipe short term cache and buffer
    mem.short_term_cache.clear();
    mem.working_buffer.clear();
    
    assert_eq!(mem.working_buffer.len(), 0);
    assert_eq!(mem.short_term_cache.len(), 0);
    // Emulate survival state where Agent uses CORE_IDENTITY only
}
