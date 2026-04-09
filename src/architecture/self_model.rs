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
    pub async fn calculate_drift(&mut self, response: &str) -> Projection {
        let anticipated = self.phase_drift;
        let mut d = self.phase_drift * 0.9; // Base decay
        
        // Use true regex to extract the deterministic drift measurements reported by the LLM
        if let Ok(re) = regex::Regex::new(r#"<drift_metrics\s+phase="([^"]+)"\s+stress="([^"]+)"\s*/>"#) {
            if let Some(caps) = re.captures(response) {
                if let Ok(phase) = caps[1].parse::<f32>() {
                    // LLM might report 0.0 to 1.0 or -1.0 to 1.0. 
                    // Map it directly to our internal drift state and override manual gravity
                    d = phase.clamp(-1.0, 1.0);
                }
                if let Ok(stress) = caps[2].parse::<f32>() {
                    self.topological_stress = stress.max(0.0);
                } else {
                    self.topological_stress = (anticipated - d).abs();
                }
            } else {
                self.topological_stress = (anticipated - d).abs();
            }
        } else {
             self.topological_stress = (anticipated - d).abs();
        }
        
        self.phase_drift = d;
        self.mode = PhaseMode::from_drift(self.phase_drift); 
        
        // F.E.A.R. automation intercept (renamed variables)
        crate::architecture::fear_automation::evaluate_and_reprogram(self, response).await;
        
        Projection {
            next_action_vector: vec![0.0; 10],
            anticipated_stress: self.topological_stress,
        }
    }
}
