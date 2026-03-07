"""Ochi REST API client compatibility module.

Preferred import path:

    from ochi_client import Ochi

Legacy import path `ochi_client` and class `Ochi` remain supported.
"""

from ochi_client import *  # noqa: F401,F403


class Ochi(Ochi):
    """Ochi-named alias for backward-compatible Ochi client implementation."""


OchiError = OchiError
