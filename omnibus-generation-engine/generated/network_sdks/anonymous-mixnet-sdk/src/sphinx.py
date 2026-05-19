"""Protocol driver for `sphinx`"""

from dataclasses import dataclass

@dataclass
class Config:
    endpoint: str = "127.0.0.1:0"
    timeout_ms: int = 5000

async def handshake(cfg: Config) -> None:
    pass
