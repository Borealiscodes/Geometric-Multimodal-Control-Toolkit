import Mathlib.Data.Real.Basic
import geometry.collapse_surface.CollapseAlignment

structure CompatibilityEnvelope :=
  (compatible : List Bool)

def evaluateCompatibility (A : AlignmentTensor) (matrix : List Float) : CompatibilityEnvelope :=
  let comp := List.zipWith
    (fun v m => (Float.abs v.x + Float.abs v.y + Float.abs v.z) ≤ m)
    A.vectors
    matrix
  { compatible := comp }

def incompatibleRegions (E : CompatibilityEnvelope) : List Nat :=
  E.compatible.enum.filterMap (fun (i, ok) => if ok then none else some i)

theorem compatibility_boolean
  (A : AlignmentTensor) (M : List Float) :
  ∀ b ∈ (evaluateCompatibility A M).compatible, b = true ∨ b = false :=
by
  intro b hb
  right
  -- trivial: b is Bool
  rfl
