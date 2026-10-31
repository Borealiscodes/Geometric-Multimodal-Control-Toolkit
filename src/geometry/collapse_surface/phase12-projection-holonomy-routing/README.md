# Phase 12 — Projection-Holonomy Routing
Geometry Spine · Collapse-Surface Module · Global Routing Layer

Phase 12 introduces the routing layer that connects holonomy transport (Phase 11)
to global invariants and continuity rules (Phase 13). While Phase 11 defines how
geometric quantities are transported along holonomy paths, Phase 12 defines how
those transported quantities are routed, projected, merged, and resolved across
the collapse-surface manifold.

This README documents the architecture, mathematics, operators, batches, Lean
invariants, and integration points for Phase 12.

---

1. Architectural Role

Phase 12 is the global routing layer. It is responsible for:

- selecting holonomy paths across the manifold
- projecting transported quantities into manifold regions
- resolving conflicts when multiple paths reach the same region
- producing globally consistent routed fields
- preparing data for Phase 13 global invariants

Phase 12 does not modify transport kernels. It operates after transport.

---

2. Mathematical Basis

Routing is defined by the projection operator:

\[
P : T_\gamma(X) \to \Gamma(TM)
\]

where:

- \( T_\gamma(X) \) is the transported quantity from Phase 11
- \( P \) projects it into the global manifold field
- \( \Gamma(TM) \) is the space of global vector fields

Routing must satisfy:

2.1 Path Validity
Paths must follow manifold adjacency and region validity rules.

2.2 Projection Consistency
Projected values must be consistent with local geometry.

2.3 Conflict Resolution
If multiple transported values reach the same region:

\[
P(T{\gamma1}(X)), P(T{\gamma2}(X)) \to \text{merged field}
\]

2.4 Continuity Preservation
Routing must preserve continuity across manifold boundaries.

These invariants are proven in Phase 13.

---

3. Routing Operators

3.1 HolonomyPathSelector
- adjacency-based traversal  
- depth-bounded expansion  
- deterministic path selection  
- produces region-indexed holonomy paths  

3.2 HolonomyProjectionOperator
- projects transported quantities into terminal regions  
- produces region-level assignments  
- prepares fields for merging  

3.3 HolonomyMergeOperator
- resolves conflicts between overlapping projections  
- uses geometry-aware weighted blending  
- preserves continuity and scalar bounds  

These operators form the routing pipeline.

---

4. Routing Batches

Batch 28 — Path Selection
- selects holonomy paths for all transported quantities  
- uses adjacency graph and seed points  

Batch 29 — Projection & Merge
- projects transported quantities into manifold regions  
- merges overlapping projections using geometry-aware blending  

These batches produce the routed fields used in Phase 13.

---

5. Lean Invariants (Scaffolding)

Phase 12 declares but does not prove:

- path validity  
- projection consistency  
- merge stability  
- continuity preservation  

Proofs are deferred to Phase 13.

Lean file:

`
lean/geometry/collapse_surface/HolonomyRouting.lean
`

---

6. Integration

Inputs (from Phase 11):
- holonomy_normals  
- holonomy_curvature  
- holonomy_alignment  
- holonomy_compatibility  
- holonomy_gradient  

Outputs (to Phase 13):
- routed_normals  
- routed_curvature  
- routed_alignment  
- routed_compatibility  
- routed_gradient  

Phase 12 is the final computational layer before global invariants.

---

Provenance

Phase 12 documentation for the Projection-Holonomy Routing Layer.  
Aligned with Phase 10–11 geometry-spine architecture and collapse-surface routing
design.  
Authored by Borealis S. Hedling and Copilot.  
Location: Dublin, Ireland.  
Timestamp: 31 October 2026 — 04:22 GMT.  
Version: v1.0
`

---

