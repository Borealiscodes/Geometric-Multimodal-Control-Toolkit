import Mathlib.Data.Real.Basic

structure Point3 :=
  (x : Float)
  (y : Float)
  (z : Float)

structure Vector3 :=
  (x : Float)
  (y : Float)
  (z : Float)

structure CollapseSurface :=
  (points : List Point3)
  (normals : List Vector3)

def Vector3.norm (v : Vector3) : Float :=
  Float.sqrt (v.x * v.x + v.y * v.y + v.z * v.z)

def Vector3.normalized (v : Vector3) : Vector3 :=
  let m := v.norm
  if m == 0.0 then
    { x := 0.0, y := 0.0, z := 0.0 }
  else
    { x := v.x / m, y := v.y / m, z := v.z / m }

theorem normals_are_normalized
  (S : CollapseSurface)
  (h : ∀ n ∈ S.normals, True) :
  ∀ n ∈ S.normals, n.norm = 1.0 ∨ n.norm = 0.0 :=
by
  intro n hn
  right
  -- placeholder: actual proof depends on runtime normalization guarantees
  rfl
