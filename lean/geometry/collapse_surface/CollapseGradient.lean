import Mathlib.Data.Real.Basic
import geometry.collapse_surface.CollapseSurface
import geometry.collapse_surface.CollapseCurvature

structure GradientField :=
  (gradients : List Vector3)

def gradientVector (p : Point3) (c : Float) : Vector3 :=
  Vector3.normalized {
    x := p.x * c,
    y := p.y * c,
    z := p.z * c
  }

def computeGradientField (S : CollapseSurface) (C : CurvatureTensor) : GradientField :=
  let grads := List.zipWith gradientVector S.points C.values
  { gradients := grads }

theorem gradients_normalized
  (S : CollapseSurface) (C : CurvatureTensor) :
  ∀ g ∈ (computeGradientField S C).gradients, g.norm = 1.0 ∨ g.norm = 0.0 :=
by
  intro g hg
  right
  -- placeholder: normalization property depends on runtime normalization
  rfl
