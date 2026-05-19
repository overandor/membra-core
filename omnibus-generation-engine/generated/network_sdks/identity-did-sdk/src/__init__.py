"""Self-sovereign identity with verifiable credentials and selective disclosure."""

class DIDResolver:
    """Self-sovereign identity with verifiable credentials and selective disclosure. — DIDResolver"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class CredentialIssuer:
    """Self-sovereign identity with verifiable credentials and selective disclosure. — CredentialIssuer"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class SelectiveDisclosure:
    """Self-sovereign identity with verifiable credentials and selective disclosure. — SelectiveDisclosure"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

class RevocationRegistry:
    """Self-sovereign identity with verifiable credentials and selective disclosure. — RevocationRegistry"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError

__all__ = ['DIDResolver', 'CredentialIssuer', 'SelectiveDisclosure', 'RevocationRegistry']
