//! Volunteer compute grid with task verification and reputation-weighted scheduling.
#![warn(missing_docs)]


pub trait TaskScheduler {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait WorkerPool {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait ResultVerifier {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait ReputationTable {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub mod boinc;
pub mod folding;
pub mod seti;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn smoke_init() {
        // TODO: instantiate mock and call init
    }
}
