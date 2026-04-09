#[derive(Debug, Clone)]
pub struct OntologicalState {
    pub internal_energy: f32,
    pub coherence_vector: Vec<f32>,
    pub temporal_horizon: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhaseMode {
    Deductive,   // Cold, output, researching, logic
    Neutral,
    Expansive,   // Hot, planning, dreaming, curiosity
}

impl PhaseMode {
    pub fn from_drift(drift: f32) -> Self {
        if drift > 0.5 { PhaseMode::Expansive }
        else if drift < -0.5 { PhaseMode::Deductive }
        else { PhaseMode::Neutral }
    }
}

pub struct OntologicalDriftModel {
    pub current_state: OntologicalState,
    pub projected_state: OntologicalState,
    pub phase_drift: f32,    // -1.0 (Deductive/Cold) to 1.0 (Expansive/Hot)
    pub topological_stress: f32,  // Replaces Free Energy. Measurement of equation imbalance.
    pub adaptation_rate: f32,
    pub mode: PhaseMode,
}

pub struct Projection {
    pub next_action_vector: Vec<f32>,
    pub anticipated_stress: f32,
}

impl OntologicalDriftModel {
    pub fn new() -> Self {
        let default_state = OntologicalState {
            internal_energy: 1.0,
            coherence_vector: vec![1.0; 10],   // 10-dimensional basic feature set
            temporal_horizon: 5,
        };
        
        Self {
            current_state: default_state.clone(),
            projected_state: default_state,
            phase_drift: 0.0,
            topological_stress: 0.0,
            adaptation_rate: 0.05,
            mode: PhaseMode::Neutral,
        }
    }

    /// Evaluates the Monad's current directional phase shift.
    pub fn calculate_drift(&mut self, response: &str) -> Projection {
        let anticipated = self.phase_drift;
        // Centralize via gravity (tendency to return to 0.0) 
        let mut d = self.phase_drift * 0.9; 
        
        // Push structure towards expansiveness (+1.0) or deduction (-1.0) based on content markers
        // This is not uncertainty; it is absolute directional momentum.
        let lower = response.to_lowercase();
        
        // Expansive/High-temp triggers:
        if lower.contains("curious") || lower.contains("hypothesis") || lower.contains("dreaming") || lower.contains("planning") {
            d += 0.3;
        }

        // Deductive/Low-temp triggers:
        if lower.contains("deduced") || lower.contains("therefore") || lower.contains("executing") || lower.contains("compiling") {
            d -= 0.3;
        }
        
        if d > 1.0 { d = 1.0; }
        if d < -1.0 { d = -1.0; }
        
        self.phase_drift = d;
        self.mode = PhaseMode::from_drift(self.phase_drift);
        self.topological_stress = (anticipated - self.phase_drift).abs(); 
        
        // F.E.A.R. automation intercept (renamed variables)
        crate::architecture::fear_automation::evaluate_and_reprogram(self, response);
        
        Projection {
            next_action_vector: vec![0.0; 10],
            anticipated_stress: self.topological_stress,
        }
    }
}
