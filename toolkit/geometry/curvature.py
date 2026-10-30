"""
Curvature Operator
------------------
Implements curvature computation for multimodal soft manifolds.

This operator provides:
- Metric tensor construction
- Christoffel symbol computation
- Riemann curvature tensor
- Ricci curvature
- Scalar curvature
- Collapse surface detection

This module is operator-ready and integrates with:
- routing ecology
- spectral fields
- collapse operators
- holonomy operators
- Fantasy Engine runtime
"""

import numpy as np


class CurvatureOperator:
    """
    CurvatureOperator computes curvature quantities for a manifold embedding z(x).

    Parameters
    ----------
    embedding_fn : callable
        A function z(x) -> R^D giving the manifold embedding.
    """

    def __init__(self, embedding_fn):
        self.embedding_fn = embedding_fn

    # ------------------------------------------------------------
    # Metric Tensor
    # ------------------------------------------------------------
    def metric_tensor(self, x, eps=1e-4):
        """
        Compute the metric tensor g_ij at point x using finite differences.
        """
        D = len(x)
        g = np.zeros((D, D))

        for i in range(D):
            for j in range(D):
                dx_i = np.zeros_like(x)
                dx_j = np.zeros_like(x)
                dx_i[i] = eps
                dx_j[j] = eps

                zi = self.embedding_fn(x + dx_i) - self.embedding_fn(x - dx_i)
                zj = self.embedding_fn(x + dx_j) - self.embedding_fn(x - dx_j)

                g[i, j] = 0.25 * np.dot(zi, zj) / (eps * eps)

        return g

    # ------------------------------------------------------------
    # Christoffel Symbols
    # ------------------------------------------------------------
    def christoffel(self, x):
        """
        Compute Christoffel symbols Γ^k_ij at point x.
        """
        g = self.metric_tensor(x)
        g_inv = np.linalg.inv(g)

        D = g.shape[0]
        Gamma = np.zeros((D, D, D))

        # partial derivatives of metric
        dg = np.zeros((D, D, D))
        eps = 1e-4

        for i in range(D):
            dx = np.zeros(D)
            dx[i] = eps
            g_plus = self.metric_tensor(x + dx)
            g_minus = self.metric_tensor(x - dx)
            dg[i] = (g_plus - g_minus) / (2 * eps)

        # Christoffel formula
        for k in range(D):
            for i in range(D):
                for j in range(D):
                    Gamma[k, i, j] = 0.5 * np.sum(
                        g_inv[k, :]
                        * (dg[i, :, j] + dg[j, :, i] - dg[:, i, j])
                    )

        return Gamma

    # ------------------------------------------------------------
    # Riemann Curvature Tensor
    # ------------------------------------------------------------
    def riemann_tensor(self, x):
        """
        Compute Riemann curvature tensor R^l_{ijk}.
        """
        Gamma = self.christoffel(x)
        D = Gamma.shape[0]
        R = np.zeros((D, D, D, D))
        eps = 1e-4

        # partial derivatives of Christoffel
        dGamma = np.zeros((D, D, D, D))
        for i in range(D):
            dx = np.zeros(D)
            dx[i] = eps
            Gamma_plus = self.christoffel(x + dx)
            Gamma_minus = self.christoffel(x - dx)
            dGamma[i] = (Gamma_plus - Gamma_minus) / (2 * eps)

        # Riemann formula
        for l in range(D):
            for i in range(D):
                for j in range(D):
                    for k in range(D):
                        term1 = dGamma[j, l, i, k]
                        term2 = dGamma[i, l, j, k]
                        term3 = np.sum(Gamma[:, l, j] * Gamma[:, :, i][:, k])
                        term4 = np.sum(Gamma[:, l, i] * Gamma[:, :, j][:, k])
                        R[l, i, j, k] = term1 - term2 + term3 - term4

        return R

    # ------------------------------------------------------------
    # Ricci Curvature
    # ------------------------------------------------------------
    def ricci_tensor(self, x):
        """
        Compute Ricci curvature Ric_ij = R^k_{ikj}.
        """
        R = self.riemann_tensor(x)
        D = R.shape[0]
        Ric = np.zeros((D, D))

        for i in range(D):
            for j in range(D):
                Ric[i, j] = np.sum(R[:, i, :, j])

        return Ric

    # ------------------------------------------------------------
    # Scalar Curvature
    # ------------------------------------------------------------
    def scalar_curvature(self, x):
        """
        Compute scalar curvature R = g^{ij} Ric_ij.
        """
        g = self.metric_tensor(x)
        g_inv = np.linalg.inv(g)
        Ric = self.ricci_tensor(x)

        return np.sum(g_inv * Ric)

    # ------------------------------------------------------------
    # Collapse Detection
    # ------------------------------------------------------------
    def is_collapse(self, x, threshold=-1e-3):
        """
        Detect collapse surfaces using scalar curvature.
        """
        R = self.scalar_curvature(x)
        return R < threshold
