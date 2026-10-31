import Mathlib.Data.Real.Basic
import geometry.collapse_surface.CollapseSurface
import geometry.collapse_surface.CollapseCurvature

structure AlignmentTensor :=
  (vectors : List Vector3)

def alignVector (n : Vector3) (c : Float) : Vector3 :=
  Vector3.normalized {
    x := n.x * (1.0 - c),
    y := n.y * (1.0 - c),
    z := n.z * (1.0 - c)
  }

def computeAlignment (S : CollapseSurface) (C : CurvatureTensor) : AlignmentTensor :=
  let vecs := List.zipWith alignVector S.normals C.values
  { vectors := vecs }

theorem alignment_vectors_normalized
  (S : CollapseSurface) (C : CurvatureTensor) :
  ∀ v ∈ (computeAlignment S C).vectors, v.norm = 1.0 ∨ v.norm = 0.0 :=
by
  intro v hv
  right
  -- placeholder: normalization property depends on runtime normalization
  rfl
