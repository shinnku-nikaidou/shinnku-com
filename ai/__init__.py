"""
AI components for shinnku-com project.

This package contains:
- BGE RAG training utilities
- Google Cloud Platform distillation tools
- Preloading utilities for embeddings
- Falcon-based serving API
"""

__version__ = "0.1.0"
__author__ = "shinnku-nikaidou"


# Import modules lazily to avoid execution-time errors
def __getattr__(name):
    if name == "bge_rag_train":
        from . import bge_rag_train

        return bge_rag_train
    elif name == "gcp_distill":
        from . import gcp_distill

        return gcp_distill
    elif name == "preload":
        from . import preload

        return preload
    elif name == "serve":
        from . import serve

        return serve
    else:
        raise AttributeError(f"module '{__name__}' has no attribute '{name}'")


__all__ = [
    "bge_rag_train",
    "gcp_distill",
    "preload",
    "serve",
]
