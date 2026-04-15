---
prov:Activity: "Mathematical discovery analysis"
prov:Agent: "Monad Kernel"
generatedAtTime: "2026-04-15T13:33:50.352536+00:00"
invalidatedAtTime: null
---
# EML Operator: Universal Mathematical Primitive

## Core Discovery
- **Operator**: eml(x,y) = e^x - ln(y)
- **Universality**: With constant 1, generates ALL elementary functions
- **Domain**: Complex numbers C (required for π via ln(-1))

## Key Properties
1. **Completeness**: Single binary operator + constant 1 = complete basis
2. **Bootstrapping**:
   - e^x = eml(x,1) [Depth 1]
   - ln(x) = eml(1, eml(eml(1,x),1)) [Depth 2]
3. **Tree Structure**: All functions as binary trees of identical EML nodes
4. **Discovery Method**: Exhaustive search (systematic ablation testing)

## Functions Generated
- **Arithmetic**: +, -, ×, ÷
- **Transcendental**: exp, ln, sin, cos, tan, etc.
- **Hyperbolic**: sinh, cosh, tanh
- **Constants**: π, e, i, -1, rational numbers
- **Operations**: roots, powers, logarithms, averages

## Implications for Evolutionary Systems
1. **Minimal Mutation Primitive**: Single operator for all mathematical mutations
2. **Universal Representation**: Any mathematical expression as EML tree
3. **Search Space Reduction**: From multiple operators to one primitive
4. **Emergent Complexity**: Complex functions from repeated simple operations

## Connection to Hyperstructure Engineering
- **Scaling Law Alignment**: τ ∝ log(S) matches EML's logarithmic depth scaling
- **Emergent Intelligence**: Complex cognition from simple repeated operations
- **Universal Basis**: Suggests cognitive primitives may be similarly minimal

## Implementation Considerations
1. **Complex Domain**: Requires principal branch handling for ln(-1)
2. **Tree Depth**: Functions require varying depths (1-8 for basics)
3. **Numerical Stability**: Careful implementation in floating point
4. **Symbolic vs. Numeric**: Both representations possible

## Research Source
- **arXiv**: 2603.21852 "All elementary functions from a single binary operator"
- **Author**: Andrzej Odrzywołek (March 2026)
- **GitHub**: weave-logic-ai/weftos (implementing EML in practice)