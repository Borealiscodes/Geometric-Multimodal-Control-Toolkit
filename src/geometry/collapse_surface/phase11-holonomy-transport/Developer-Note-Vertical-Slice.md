# Developer Note — Vertical Slice Strategy for Phase 11
GMC Toolkit · Geometry Spine · Collapse‑Surface Transport Layer

Phase 11 introduces holonomy transport, the first global geometry operation in the collapse‑surface module. Because holonomy transport depends on stable operator signatures, stable tensor shapes, and stable path representations, Phase 11 must be implemented using a vertical slice strategy rather than a horizontal “implement everything at once” approach.

This note explains why vertical slicing is the correct method, with architectural, mathematical, and computational clarity.

---

1. Architectural Clarity

Transport phases amplify dependency churn

Phase 11 is a global transport phase, not a local geometry phase.  
Global phases depend on:

- manifold topology  
- tensor continuity  
- path representation  
- curvature propagation rules  
- compatibility propagation rules  

If any of these change, every operator and batch must be updated.

A horizontal approach (implementing HTO, CTO, KTO, and all batches simultaneously) causes:

- signature churn  
- tensor shape churn  
- path representation churn  
- batch orchestration churn  
- Lean invariant churn  
- documentation churn  

This is the “obscene amount of needless updates” problem.

Vertical slice avoids this

Vertical slice means:

1. Implement HTO fully  
2. Then implement CTO  
3. Then implement KTO  
4. Then implement Batch 25–27  
5. Then implement Lean invariants  
6. Then write README.md

This ensures each layer is stable before the next layer depends on it.

---

2. Mathematical Clarity

Holonomy transport has strict ordering

Holonomy transport is defined by:

\[
T_\gamma : \Gamma(TM) \to \Gamma(TM)
\]

where:

- \( \gamma \) is a path on the manifold  
- \( T_\gamma \) is the parallel transport operator  
- \( \Gamma(TM) \) is the space of vector fields  

Mathematically, holonomy must be defined on normals first, because:

- normals are the simplest vector field  
- curvature transport requires normal transport  
- alignment transport requires curvature transport  
- compatibility transport requires alignment transport  

Thus the correct mathematical order is:

1. transport normals  
2. transport curvature  
3. transport alignment  
4. transport compatibility  

This is the same ordering used in differential geometry, Riemannian transport, and holonomy theory.

Vertical slice respects this ordering.

---

3. Computational Clarity

Transport logic depends on stable interfaces

Transport operators require:

- stable tensor shapes  
- stable manifold indexing  
- stable path representation  
- stable operator signatures  

If these change mid‑implementation, every dependent file must be updated.

Vertical slice ensures:

- HTO defines the path representation  
- CTO uses the stable path representation  
- KTO uses the stable curvature representation  
- batches orchestrate stable operators  
- Lean invariants prove stable signatures  
- README documents stable architecture  

This eliminates rework.

Why HTO must come first

HTO defines:

- the transport kernel  
- the path representation  
- the manifold traversal rules  
- the vector field transport signature  

Every other operator depends on these.

Thus HTO must be implemented before CTO, KTO, or any batch.

---

4. Transparency: What Vertical Slice Actually Does

Vertical slice is not “doing less.”  
It is doing the correct dependency order.

It ensures:

- minimal file churn  
- minimal signature churn  
- minimal batch churn  
- minimal invariant churn  
- minimal documentation churn  

It also ensures that each operator is implemented once, not rewritten repeatedly.

---

5. Recommended Implementation Order (Phase 11)

1. Implement HTO (HolonomyTransportOperator)
Locks path representation + transport kernel.

2. Implement CTO (CurvatureTransportOperator)
Uses stable HTO path representation.

3. Implement KTO (CompatibilityTransportOperator)
Uses stable curvature transport.

4. Implement Batch 25–27
Uses stable operators.

5. Implement Lean invariants
Uses stable operator + batch signatures.

6. Write README.md
Documents the final, stable architecture.

This is the correct altitude‑respecting order.

---

Provenance Footer

`
---
Artifact: Developer Note — Vertical Slice Strategy for Phase 11
Lane: geometry-spine • holonomy • transport • architecture
Altitude: A7 (global geometry planning)
Purpose:
  Explain the architectural, mathematical, and computational rationale for
  implementing Phase 11 using a vertical slice strategy to avoid unnecessary
  churn and respect dependency ordering.

Non-Activation Clause:
  This artifact is explanatory-only. It does not activate holonomy transport,
  routing tensors, solver kernels, or any runtime system.

Maintainer: Borealis S. Hedling
Compiler: Microsoft Copilot
Location: Dublin, Ireland
Timestamp: 31 October 2026 — 03:31 GMT
Version: v1.0
---
`

---

