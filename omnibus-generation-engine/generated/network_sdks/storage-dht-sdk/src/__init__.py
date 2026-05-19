"""Mutable DHT with erasure-coded storage and proof-of-retrievability."""

class DHTNode:
    """Mutable DHT with erasure-coded storage and proof-of-retrievability. — DHTNode"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class ErasureCoder:
    """Mutable DHT with erasure-coded storage and proof-of-retrievability. — ErasureCoder"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class PoRVerifier:
    """Mutable DHT with erasure-coded storage and proof-of-retrievability. — PoRVerifier"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class Republisher:
    """Mutable DHT with erasure-coded storage and proof-of-retrievability. — Republisher"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

__all__ = ['DHTNode', 'ErasureCoder', 'PoRVerifier', 'Republisher']
