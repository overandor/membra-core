//! Epidemic gossip with anti-entropy and vector-clock reconciliation.
#![warn(missing_docs)]


pub trait GossipAgent {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait StateReconciler {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait MembershipList {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait AntiEntropy {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub mod swim;
pub mod plumtree;
pub mod scuttlebutt;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn smoke_init() {
        // TODO: instantiate mock and call init
    }
}
