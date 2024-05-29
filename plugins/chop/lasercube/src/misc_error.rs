use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MiscError {
    EmptyBufferReceived,
    NoSpaceInBuffer,
    NotConnected,
    FailedToLockController
}

impl Display for MiscError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            MiscError::EmptyBufferReceived => "Empty buffer received",
            MiscError::NoSpaceInBuffer => "No space in buffer",
            MiscError::NotConnected => "Not connected",
            MiscError::FailedToLockController => "Failed to access helper object"
        };
        
        _ = f.write_str(desc);
        
        Ok(())
    }
}

impl Error for MiscError {}