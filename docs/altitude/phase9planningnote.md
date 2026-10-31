# Phase 9 — Projection-Seed Geometry Activation (G-Layer)
## Repository Placement Clarification

Phase 9 introduces the Projection-Seed Geometry Layer (G-Layer), the first
geometry-bearing altitude since Phase 6. Although projection-seeds belong to the
geometry lineage, their implementation does not require moving out of the
Geometric-Multimodal-Control-Toolkit (GMC). The geometry/ecology boundary is
conceptual, not repository-mandatory.

Projection-seeds are not exclusive to any single toolkit. They can be developed
in either GMC or Constellation-Manifold-Toolkit (CMT). GMC currently provides a
more organized and integrated environment, with holonomy, ecology, world-rules,
and safety already unified. Therefore, Phase 9 development can proceed inside
GMC without violating altitude purity or architectural constraints.

If future toolkits require projection-seeds independently, the geometry layer
can be exported or mirrored into CMT. Phase 9 does not lock the geometry to a
single repository.

---

## Altitude Identity

Phase 9 activates projection-seed geometry:
- seed curvature operators
- seed holonomy response
- seed alignment response
- seed collapse couplers

This is the transition from ecology (Phase 8) to geometry (Phase 9).

---

## Mission

1. Generate projection-seeds from holonomy, ecology, and world-rule signals.
2. Activate seed geometry as curvature-bearing, holonomy-responsive operators.
3. Introduce seed-level collapse coupling.
4. Integrate seed geometry with P-Layer ecology and safety.

---

## Components

### 1. Projection-Seed Generator
Creates seeds from:
- holonomy field
- ecology curvature field
- ecology alignment field
- world-rules

### 2. Seed Curvature Operator
Computes curvature for seed geometry.

### 3. Seed Collapse Coupler
Ensures safe interaction with collapse surfaces.

---

## Inputs (from Phase 8)
- holonomy runtime
- ecology curvature field
- ecology alignment field
- synthesized world-rules
- ecology safety signals

## Outputs (to Phase 10)
- projection-seed geometry
- seed curvature
- seed collapse couplers

---

## Repository Strategy

### GMC (current recommended location)
- Fully integrated holonomy, ecology, world-rules, and safety.
- Organized runtime structure.
- Direct access to Phase 8 substrates.
- Ideal for Phase 9 implementation.

### CMT (optional geometry-only location)
- Suitable for future standalone geometry engines.
- Useful if projection-seeds must be reused outside GMC.
- Can receive exported geometry modules later.

Phase 9 development is valid in either repository. GMC is currently the more
coherent and structured environment, so Phase 9 will proceed there.

---

## Deliverables

- `ProjectionSeedGenerator`
- `SeedCurvatureOperator`
- `SeedCollapseCoupler`
- Phase 9 documentation

---

## Provenance

Phase 9 Planning Note updated to reflect correct repository logic: projection-
seed geometry is not exclusive to any toolkit and can be developed inside GMC
without violating altitude boundaries. CMT remains available for future geometry
export or standalone manifold engines.
