//! Sub-microsecond clock sync with Byzantine fault-tolerant time master election.
#![warn(missing_docs)]


pub trait ClockSync {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait MasterElection {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait DelayEstimator {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub trait LeapSecondHandler {
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub mod ptp;
pub mod ntp_bis;
pub mod huygens;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn smoke_init() {
        // TODO: instantiate mock and call init
    }
}
