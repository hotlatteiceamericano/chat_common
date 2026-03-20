use futures::channel::mpsc::TrySendError;
use thiserror::Error;

use crate::message::Message;

/// Custom error to conclude three different types of error
/// from distinct crates into this enum
/// It uses thiserror's derive helpers to implement
/// Error trait (for anyhow)
/// and From<T> trait (convert to MessageHandleError)
#[derive(Debug, Error)]
pub enum MessageHandleError {
    #[error("user id not found: {id}")]
    ReceiverNotFound { id: u32 },

    #[error("invalid message format")]
    InvalidMessageFormat {
        #[from]
        error: serde_json::Error,
    },

    #[error("error when sending messages to receiver's transimitter")]
    MPSC(#[from] TrySendError<Message>),
}
