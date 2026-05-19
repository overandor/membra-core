"""Decentralized ML training with differential privacy and secure aggregation."""

class Aggregator:
    """Decentralized ML training with differential privacy and secure aggregation. — Aggregator"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class LocalTrainer:
    """Decentralized ML training with differential privacy and secure aggregation. — LocalTrainer"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class PrivacyFilter:
    """Decentralized ML training with differential privacy and secure aggregation. — PrivacyFilter"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class ModelExporter:
    """Decentralized ML training with differential privacy and secure aggregation. — ModelExporter"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

__all__ = ['Aggregator', 'LocalTrainer', 'PrivacyFilter', 'ModelExporter']
