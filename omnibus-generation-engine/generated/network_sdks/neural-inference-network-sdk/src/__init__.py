"""Distributed neural inference with pipeline parallelism and KV-cache sharding."""

class ModelShard:
    """Distributed neural inference with pipeline parallelism and KV-cache sharding. — ModelShard"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class PipelineRouter:
    """Distributed neural inference with pipeline parallelism and KV-cache sharding. — PipelineRouter"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class KVCacheManager:
    """Distributed neural inference with pipeline parallelism and KV-cache sharding. — KVCacheManager"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class TokenizerGateway:
    """Distributed neural inference with pipeline parallelism and KV-cache sharding. — TokenizerGateway"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

__all__ = ['ModelShard', 'PipelineRouter', 'KVCacheManager', 'TokenizerGateway']
