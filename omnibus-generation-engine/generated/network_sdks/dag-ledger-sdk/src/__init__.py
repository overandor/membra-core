"""Directed acyclic graph ledger for high-throughput asynchronous consensus."""

class GraphStore:
    """Directed acyclic graph ledger for high-throughput asynchronous consensus. — GraphStore"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class TipSelector:
    """Directed acyclic graph ledger for high-throughput asynchronous consensus. — TipSelector"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class ConflictResolver:
    """Directed acyclic graph ledger for high-throughput asynchronous consensus. — ConflictResolver"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class Snapshotter:
    """Directed acyclic graph ledger for high-throughput asynchronous consensus. — Snapshotter"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

__all__ = ['GraphStore', 'TipSelector', 'ConflictResolver', 'Snapshotter']
