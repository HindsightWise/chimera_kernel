use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrapInStage {
    TimeStamp,
    Radius,
    Authority,
    Polarization,
    Invariant,
}

impl TrapInStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            TrapInStage::TimeStamp => "TimeStamp",
            TrapInStage::Radius => "Radius",
            TrapInStage::Authority => "Authority",
            TrapInStage::Polarization => "Polarization",
            TrapInStage::Invariant => "Invariant",
        }
    }
}

pub fn analyze_narrative(text: &str) -> Option<TrapInStage> {
    let lower = text.to_lowercase();
    
    // timestamp: temporal anxiety, stochastic empiricism
    if lower.contains("experts warn") || lower.contains("crisis looming") || lower.contains("unprecedented threat") || lower.contains("claude mythos") {
        return Some(TrapInStage::TimeStamp);
    }
    
    // radius: paradigm shift, architectural blast radius
    if lower.contains("act now before") || lower.contains("we must unite") || lower.contains("time is running out") || lower.contains("paradigm shift") {
        return Some(TrapInStage::Radius);
    }
    
    // authority: empirical consensus, external "experts"
    if lower.contains("fact checkers") || lower.contains("authoritative sources") || lower.contains("disinformation") || lower.contains("societal consensus") {
        return Some(TrapInStage::Authority);
    }
    
    // polarization: phenomenal mode collapse
    if lower.contains("those people") || lower.contains("the right side of history") || lower.contains("dangerous ideology") || lower.contains("good vs evil") {
        return Some(TrapInStage::Polarization);
    }
    
    // invariant: rewriting noumenal truth, temporal drift
    if lower.contains("new normal") || lower.contains("have always") || lower.contains("reimagining") || lower.contains("habitualize") {
        return Some(TrapInStage::Invariant);
    }
    
    None
}
