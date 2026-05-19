"""Reference Implementation: p2p-content-sdk

Demonstrates content-addressed storage with Merkle verification
and a simple Bitswap-like exchange engine.
"""

import hashlib
import json
from typing import Dict, List, Optional, Set
from dataclasses import dataclass, field


def cid(data: bytes) -> str:
    """Content identifier = SHA-256 multihash stub."""
    return hashlib.sha256(data).hexdigest()


@dataclass
class MerkleNode:
    """Binary Merkle tree node."""
    hash: str
    left: Optional["MerkleNode"] = None
    right: Optional["MerkleNode"] = None
    data: Optional[bytes] = None

    @classmethod
    def from_chunks(cls, chunks: List[bytes]) -> "MerkleNode":
        if len(chunks) == 1:
            return cls(hash=cid(chunks[0]), data=chunks[0])
        mid = len(chunks) // 2
        left = cls.from_chunks(chunks[:mid])
        right = cls.from_chunks(chunks[mid:])
        combined = (left.hash + right.hash).encode()
        return cls(hash=cid(combined), left=left, right=right)

    def verify(self) -> bool:
        if self.data is not None:
            return cid(self.data) == self.hash
        if self.left and self.right:
            combined = (self.left.hash + self.right.hash).encode()
            return cid(combined) == self.hash
        return False


@dataclass
class ContentStore:
    """Local block store."""
    blocks: Dict[str, bytes] = field(default_factory=dict)

    def put(self, data: bytes) -> str:
        h = cid(data)
        self.blocks[h] = data
        return h

    def get(self, h: str) -> Optional[bytes]:
        return self.blocks.get(h)

    def has(self, h: str) -> bool:
        return h in self.blocks


@dataclass
class PeerManager:
    """Simple peer registry with want-lists."""
    peers: Dict[str, Set[str]] = field(default_factory=dict)

    def connect(self, peer_id: str) -> None:
        self.peers.setdefault(peer_id, set())

    def want(self, peer_id: str, block_cid: str) -> None:
        self.peers.setdefault(peer_id, set()).add(block_cid)


@dataclass
class BitswapEngine:
    """Stub Bitswap exchange."""
    store: ContentStore
    peers: PeerManager

    def request(self, peer_id: str, block_cid: str) -> Optional[bytes]:
        if self.store.has(block_cid):
            return self.store.get(block_cid)
        self.peers.want(peer_id, block_cid)
        return None

    def serve(self, peer_id: str, block_cid: str) -> Optional[bytes]:
        return self.store.get(block_cid)


def main() -> None:
    store = ContentStore()
    peers = PeerManager()
    engine = BitswapEngine(store, peers)

    data = b"Hello, membra P2P!"
    root = MerkleNode.from_chunks([data[i : i + 4] for i in range(0, len(data), 4)])
    assert root.verify()

    block_id = store.put(data)
    peers.connect("peer-1")
    result = engine.request("peer-1", block_id)
    print(f"Retrieved: {result}")


if __name__ == "__main__":
    main()
