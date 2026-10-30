"""
Symbolic Multimodal Adapter
---------------------------
Maps symbolic structures (ASTs, graphs, algebraic expressions) into the
multimodal manifold as geometric patches.

This adapter provides:
- Structural normalization
- Graph / AST embedding
- Patch construction
- Curvature-aware metadata
- Spectral field estimation
- Routing-surface integration

Integrates with:
- Multimodal Manifolds (T-006)
- Routing Ecology (T-007)
- Curvature Operator (T-009)
- Holonomy Operator (T-010)
- Fantasy Engine runtime
"""

import numpy as np
from toolkit.geometry.curvature import CurvatureOperator


class SymbolicAdapter:
    """
    SymbolicAdapter embeds symbolic structures into the multimodal manifold.

    Parameters
    ----------
    normalize_fn : callable
        Function that maps raw symbolic structure -> normalized representation.
    embed_fn : callable
        Function that maps normalized symbolic structure -> R^D embedding.
    """

    def __init__(self, normalize_fn, embed_fn):
        self.normalize_fn = normalize_fn
        self.embed_fn = embed_fn
        self.curv = CurvatureOperator(self._embedding_wrapper)

    # ------------------------------------------------------------
    # Internal embedding wrapper for curvature operator
    # ------------------------------------------------------------
    def _embedding_wrapper(self, x):
        """
        Wrap embedding_fn so curvature operator can treat embeddings as manifold points.
        """
        return np.array(x)

    # ------------------------------------------------------------
    # Normalization
    # ------------------------------------------------------------
    def normalize(self, symbolic_obj):
        """
        Normalize symbolic structure (AST, graph, algebraic expression).
        """
        return self.normalize_fn(symbolic_obj)

    # ------------------------------------------------------------
    # Embedding
    # ------------------------------------------------------------
    def embed(self, symbolic_obj):
        """
        Convert symbolic structure into an embedding in R^D.
        """
        normalized = self.normalize(symbolic_obj)
        embedding = self.embed_fn(normalized)
        return np.array(embedding)

    # ------------------------------------------------------------
    # Patch Construction
    # ------------------------------------------------------------
    def construct_patch(self, symbolic_obj):
        """
        Construct a manifold patch from a symbolic embedding.

        Returns
        -------
        patch : dict
            {
                "point": embedding,
                "curvature": scalar_curvature,
                "spectral": spectral_value,
                "routing": routing_metadata
            }
        """
        embedding = self.embed(symbolic_obj)

        curvature_value = float(self.curv.scalar_curvature(embedding))

        # Spectral field: gradient magnitude of embedding vector
        spectral_value = float(np.linalg.norm(np.gradient(embedding)))

        routing_metadata = {
            "curvature": curvature_value,
            "spectral": spectral_value,
            "collapse_risk": float(curvature_value < -1e-3)
        }

        return {
            "point": embedding,
            "curvature": curvature_value,
            "spectral": spectral_value,
            "routing": routing_metadata
        }

    # ------------------------------------------------------------
    # Collapse Detection
    # ------------------------------------------------------------
    def has_collapse(self, symbolic_obj):
        """
        Detect collapse surfaces in the symbolic patch.
        """
        patch = self.construct_patch(symbolic_obj)
        return patch["curvature"] < -1e-3

    # ------------------------------------------------------------
    # Spectral Drift
    # ------------------------------------------------------------
    def spectral_drift(self, symbolic_obj):
        """
        Compute spectral drift for the symbolic embedding.
        """
        patch = self.construct_patch(symbolic_obj)
        return float(abs(patch["spectral"]))
