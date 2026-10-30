# Developer Note — Testing Stack Inclusion (B vs C)

GeometricMultimodalControl · Internal Architecture Guidance

Formal vs Geometric vs Runtime Testing Layers

Summary

Only C (runtime stress‑testing) is required inside the GeometricMultimodalControl repository.  
Layer B (Lean geometric formalization) is optional and should live in a separate repo if included at all.

This note explains why.

---

A → B → C Testing Architecture

The hallucination‑guard system relies on three distinct layers:

A — Coq (Formal Invariants)
Proves logical constraints:
- monotonicity  
- boundedness  
- non‑collapse  
- operator safety  
- metric validity  

This layer belongs in CoQ‑Hallucination‑Guard, not in GeometricMultimodalControl.

---

B — Lean (Operator Algebra + CAD Geometry)
Models continuous geometry:
- curvature fields  
- Hessians  
- holonomy  
- spectral operators  
- multimodal manifolds  
- soft‑world geometry (80D)  

Lean is a geometric CAD system, not a runtime test harness.

Lean is extremely valuable, but it is not required for the core toolkit.

---

C — Runtime Stress Tests (Python/Rust)
Tests actual multimodal behavior:
- collapse surfaces  
- spectral explosions  
- routing instability  
- fusion drift  
- world‑building coherence  
- slack‑dimension collapse  
- generative hallucination modes  

This is the only testing layer that must be inside the main repo, because it validates the actual operators implemented in the toolkit.

---

Recommendation

Include only C in GeometricMultimodalControl.

✔ Required
Runtime stress tests must be part of the toolkit because they validate:
- operator stability  
- multimodal fusion behavior  
- collapse avoidance  
- spectral‑runtime safety  
- world‑building coherence  

These tests directly interact with the implemented operators and multimodal integration layer.

✖ Not required
Lean (Layer B) should not be included in the main repo because:
- it introduces a second formal system  
- it complicates the build and dependency chain  
- it belongs to a different workflow (CAD + formal geometry)  
- it is not needed for users of the toolkit  
- it is not required for operator correctness  

Lean is valuable, but it should be isolated.

---

Final Architecture Decision

GeometricMultimodalControl includes:
- operator implementations  
- multimodal integration  
- collapse detection  
- spectral stability checks  
- routing safety  
- world‑building geometry  
- runtime stress tests (C)

GeometricMultimodalControl does not include:
- Coq formal proofs (A)  
- Lean geometric CAD (B)  

Separate repos:
- CoQ‑Hallucination‑Guard → formal invariants  
- Lean‑Geometry‑CAD (optional) → operator algebra + manifold modeling  

This separation keeps the main repo clean, usable, and focused on the runtime system.

---

Provenance

`
Developer Note — Testing Stack Inclusion (B vs C)
Authored by Borealis S. Hedling on Friday, 30 October 2026 at 22:36 GMT
in Dublin, Ireland.

Defines which testing layers belong inside the GeometricMultimodalControl
repository. Recommends including only runtime stress tests (C) and keeping
formal systems (A, B) in separate repositories for clarity and architectural
separation.
`
`

---

