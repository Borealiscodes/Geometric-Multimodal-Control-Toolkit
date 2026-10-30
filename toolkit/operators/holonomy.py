"""
Holonomy Operator
-----------------
Computes holonomy for loops on multimodal soft manifolds.

Holonomy measures orientation change after traversing a closed loop γ.
This operator provides:
- Loop discretization
- Tangent vector computation
- Riemann curvature contraction
- Holonomy accumulation
- Operator-ready holonomy output

Integrates with:
- CurvatureOperator (T-009)
- Routing ecology
- Collapse detection
- Fantasy Engine world traversal
"""

import numpy as np
from toolkit.geometry.curvature import CurvatureOperator


class HolonomyOperator:
    """
    HolonomyOperator computes holonomy Δv for a closed loop γ on a manifold.

    Parameters
    ----------
    embedding_fn : callable
        A function z(x) -> R^D giving the manifold embedding.
    """

    def __init__(self, embedding_fn):
        self.curv = CurvatureOperator(embedding_fn)

    # ------------------------------------------------------------
    # Loop Discretization
    # ------------------------------------------------------------
    def discretize_loop(self, gamma, n_points=200):
        """
        Discretize a parametric loop γ(t) into n_points samples.

        Parameters
        ----------
        gamma : callable
            Parametric loop γ(t) with t ∈ [0, 1].
        """
        ts = np.linspace(0.0, 1.0, n_points)
        points = np.array([gamma(t) for t in ts])
        return points

    # ------------------------------------------------------------
    # Tangent Vectors
    # ------------------------------------------------------------
    def tangent_vectors(self, points):
        """
        Compute tangent vectors along the discretized loop.
        """
        tangents = np.diff(points, axis=0)
        return tangents

    # ------------------------------------------------------------
    # Holonomy Accumulation
    # ------------------------------------------------------------
    def holonomy(self, gamma, n_points=200):
        """
        Compute holonomy Δv for loop γ.

        Holonomy equation:
            Δv = ∮ H(v) dγ
        where:
            H(v) = R(v, γ̇) γ̇
        """
        points = self.discretize_loop(gamma, n_points)
        tangents = self.tangent_vectors(points)

        # Initialize holonomy vector
        D = len(points[0])
        delta_v = np.zeros(D)

        # Accumulate holonomy along loop
        for i in range(len(tangents)):
            x = points[i]
            v = tangents[i]

            # Riemann curvature tensor at x
            R = self.curv.riemann_tensor(x)

            # Contract R(v, γ̇)γ̇
            # R^l_{ijk} v^i v^j
            Hv = np.zeros(D)
            for l in range(D):
                for i in range(D):
                    for j in range(D):
                        Hv[l] += R[l, i, j, j] * v[i] * v[j]

            delta_v += Hv

        return delta_v

    # ------------------------------------------------------------
    # Holonomy Magnitude
    # ------------------------------------------------------------
    def holonomy_magnitude(self, gamma, n_points=200):
        """
        Compute ‖Δv‖, the magnitude of holonomy.
        """
        delta_v = self.holonomy(gamma, n_points)
        return np.linalg.norm(delta_v)

    # ------------------------------------------------------------
    # Holonomy Detection
    # ------------------------------------------------------------
    def has_nonzero_holonomy(self, gamma, threshold=1e-3):
        """
        Detect whether a loop γ exhibits nonzero holonomy.
        """
        return self.holonomy_magnitude(gamma) > threshold
