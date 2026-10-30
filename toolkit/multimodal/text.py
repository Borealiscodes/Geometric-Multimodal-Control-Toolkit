"""
Text Multimodal Adapter
-----------------------
Maps text input into the multimodal manifold as a geometric patch.

This adapter provides:
- Tokenization
- Embedding into R^D
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


class TextAdapter:
    """
    TextAdapter embeds text into the multimodal manifold.

    Parameters
    ----------
    tokenizer : callable
        Function that maps text -> list of tokens.
    embed_fn : callable
        Function that maps tokens -> R^D embeddings.
    """

    def __init__(self, tokenizer, embed_fn):
        self.tokenizer = tokenizer
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
    # Tokenization
    # ------------------------------------------------------------
    def tokenize(self, text):
        """
        Convert text into tokens.
        """
        return self.tokenizer(text)

    # ------------------------------------------------------------
    # Embedding
    # ------------------------------------------------------------
    def embed(self, text):
        """
        Convert text into a sequence of embeddings in R^D.
        """
        tokens = self.tokenize(text)
        embeddings = np.array([self.embed_fn(tok) for tok in tokens])
        return embeddings

    # ------------------------------------------------------------
    # Patch Construction
    # ------------------------------------------------------------
    def construct_patch(self, text):
        """
        Construct a manifold patch from text embeddings.

        Returns
        -------
        patch : dict
            {
                "points": embeddings,
                "curvature": curvature_values,
                "spectral": spectral_field,
                "routing": routing_metadata
            }
        """
        embeddings = self.embed(text)

        curvature_values = np.array([
            self.curv.scalar_curvature(point)
            for point in embeddings
        ])

        spectral_field = np.array([
            np.linalg.norm(np.gradient(point))
            for point in embeddings
        ])

        routing_metadata = {
            "avg_curvature": float(np.mean(curvature_values)),
            "spectral_variation": float(np.mean(spectral_field)),
            "collapse_risk": float(np.mean(curvature_values < -1e-3))
        }

        return {
            "points": embeddings,
            "curvature": curvature_values,
            "spectral": spectral_field,
            "routing": routing_metadata
        }

    # ------------------------------------------------------------
    # Collapse Detection
    # ------------------------------------------------------------
    def has_collapse(self, text):
        """
        Detect collapse surfaces in the text patch.
        """
        patch = self.construct_patch(text)
        return np.any(patch["curvature"] < -1e-3)

    # ------------------------------------------------------------
    # Spectral Drift
    # ------------------------------------------------------------
    def spectral_drift(self, text):
        """
        Compute spectral drift across the text patch.
        """
        patch = self.construct_patch(text)
        return float(np.std(patch["spectral"]))
