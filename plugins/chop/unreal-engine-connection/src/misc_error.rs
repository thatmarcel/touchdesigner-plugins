use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MiscError {
    UnknownMessageId,
    NonMatchingMessageSender,
    NotConnected,
    FailedToLockController
}

impl Display for MiscError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            MiscError::UnknownMessageId => "Received message with unknown identifier",
            MiscError::NonMatchingMessageSender => "Received message from non-matching sender",
            MiscError::NotConnected => "Not connected",
            MiscError::FailedToLockController => "Failed to access helper object"
        };

        _ = f.write_str(desc);

        Ok(())
    }
}

impl Error for MiscError {}