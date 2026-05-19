//! Trust-minimized bridge with light-client verification and relayer auctions.
#![warn(missing_docs)]


pub trait LightClient {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait RelayerAuction {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait Lockbox {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait Prover {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub mod ibc;
pub mod layerzero;
pub mod wormhole;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn smoke_init() {
        // TODO: instantiate mock and call init
    }
}
