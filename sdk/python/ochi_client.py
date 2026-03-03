"""Ochi REST API client compatibility module.

Preferred import path:

    from ochi_client import Ochi

Legacy import path `openfang_client` and class `OpenFang` remain supported.
"""

from openfang_client import *  # noqa: F401,F403


class Ochi(OpenFang):
    """Ochi-named alias for backward-compatible OpenFang client implementation."""


OchiError = OpenFangError
