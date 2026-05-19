//! Distributed neural inference with pipeline parallelism and KV-cache sharding.
#![warn(missing_docs)]


pub trait ModelShard {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait PipelineRouter {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait KVCacheManager {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait TokenizerGateway {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub mod nccl;
pub mod grpc;
pub mod rdma;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn smoke_init() {
        // TODO: instantiate mock and call init
    }
}
