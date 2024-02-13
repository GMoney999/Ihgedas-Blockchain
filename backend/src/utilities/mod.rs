use std::time::{SystemTime, UNIX_EPOCH};
use super::error::{BlockError};

pub fn get_timestamp() -> Result<u128, Box<BlockError>> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let nanos= duration.as_nanos();
            Ok(nanos)
        },
        Err(_) => Err(Box::new(BlockError::InvalidTimestamp))
    }
}
