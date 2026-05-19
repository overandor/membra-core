"""Layered encryption mixnet with Sphinx packet format and cover traffic."""

class MixNode:
    """Layered encryption mixnet with Sphinx packet format and cover traffic. — MixNode"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class SphinxBuilder:
    """Layered encryption mixnet with Sphinx packet format and cover traffic. — SphinxBuilder"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class PathSelector:
    """Layered encryption mixnet with Sphinx packet format and cover traffic. — PathSelector"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class CoverTraffic:
    """Layered encryption mixnet with Sphinx packet format and cover traffic. — CoverTraffic"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

__all__ = ['MixNode', 'SphinxBuilder', 'PathSelector', 'CoverTraffic']
