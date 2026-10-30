"""
Audio Multimodal Adapter
------------------------
Maps audio input into the multimodal manifold as a geometric patch.

This adapter provides:
- Waveform preprocessing
- Spectrogram / feature extraction
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


class AudioAdapter:
    """
    AudioAdapter embeds audio signals into the multimodal manifold.

    Parameters
    ----------
    preprocess_fn : callable
        Function that maps raw waveform -> normalized tensor.
    embed_fn : callable
        Function that maps preprocessed audio -> R^D embedding.
    """

    def __init__(self, preprocess_fn, embed_fn):
        self.preprocess_fn = preprocess_fn
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
    # Preprocessing
    # ------------------------------------------------------------
    def preprocess(self, waveform):
        """
        Normalize and prepare audio waveform for embedding.
        """
        return self.preprocess_fn(waveform)

    # ------------------------------------------------------------
    # Embedding
    # ------------------------------------------------------------
    def embed(self, waveform):
        """
        Convert audio waveform into an embedding in R^D.
        """
        tensor = self.preprocess(waveform)
        embedding = self.embed_fn(tensor)
        return np.array(embedding)

    # ------------------------------------------------------------
    # Patch Construction
    # ------------------------------------------------------------
    def construct_patch(self, waveform):
        """
        Construct a manifold patch from an audio embedding.

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
        embedding = self.embed(waveform)

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
    def has_collapse(self, waveform):
        """
        Detect collapse surfaces in the audio patch.
        """
        patch = self.construct_patch(waveform)
        return patch["curvature"] < -1e-3

    # ------------------------------------------------------------
    # Spectral Drift
    # ------------------------------------------------------------
    def spectral_drift(self, waveform):
        """
        Compute spectral drift for the audio embedding.
        """
        patch = self.construct_patch(waveform)
        return float(abs(patch["spectral"]))
