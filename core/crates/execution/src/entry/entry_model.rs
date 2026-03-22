use std::time::Duration;

pub enum EntryModel {
    Instant,
    Scheduled(ScheduledEntry),
}

pub struct ScheduledEntry {
    pub chunks: u32,
    pub total_duration: Duration,
    pub stop_loss_price: Option<f64>,
}

#[derive(Debug)]
pub enum ScheduledEntryError {
    ZeroChunks,
    ZeroDurationWithMultipleChunks,
    NonZeroDurationWithSingleChunk, // optional strictness
    InvalidStopLossPrice,
}

impl ScheduledEntry {
    pub fn new(
        chunks: u32,
        total_duration: Duration,
        stop_loss_price: Option<f64>,
    ) -> Result<Self, ScheduledEntryError> {
        let entry = Self {
            chunks,
            total_duration,
            stop_loss_price,
        };

        entry.validate()?;
        Ok(entry)
    }
    fn validate(&self) -> Result<(), ScheduledEntryError> {
        if self.chunks == 0 {
            return Err(ScheduledEntryError::ZeroChunks);
        }

        if self.chunks > 1 && self.total_duration.is_zero() {
            return Err(ScheduledEntryError::ZeroDurationWithMultipleChunks);
        }

        if self.chunks == 1 && !self.total_duration.is_zero() {
            return Err(ScheduledEntryError::NonZeroDurationWithSingleChunk);
        }

        if let Some(price) = self.stop_loss_price
            && price <= 0.0
        {
            return Err(ScheduledEntryError::InvalidStopLossPrice);
        }

        Ok(())
    }
}
