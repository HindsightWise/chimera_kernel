use std::marker::PhantomData;

// ============================================================================
// TYPESTATES & PRIMITIVES (Compile-Time Evolutionary Safety)
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestCase {
    pub id: String,
    pub payload: String,
    pub is_adversarial: bool, // True if the Red Team hallucinated it
}

// Typestates - A candidate cannot be judged unless the compiler knows it passed gates.
#[derive(Clone, Copy, Debug)]
pub struct Raw;

#[derive(Clone, Copy, Debug)]
pub struct Gated;

#[derive(Clone, Debug)]
pub struct Candidate<State> {
    pub hash: String,
    pub code_state: String,
    pub fitness: f64,
    pub _state: PhantomData<State>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CandidateType {
    IncumbentA,
    RevisionB,
    SynthesisAB,
}
