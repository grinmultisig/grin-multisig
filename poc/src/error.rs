//! Error types for `MuSig2` `PoC`

use std::fmt;

/// Result type alias for `MuSig2` operations
pub type Result<T> = std::result::Result<T, Error>;

/// `MuSig2` protocol errors
///
/// # Examples
///
/// ```rust
/// use grin_multisig_poc::{Error, Result};
///
/// fn check_participants(count: usize) -> Result<()> {
///     if count == 0 {
///         Err(Error::NoParticipants)
///     } else {
///         Ok(())
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// No participants in the session
    ///
    /// This error occurs when trying to create a session or perform operations
    /// without any participants.
    NoParticipants,

    /// Commitment verification failed
    ///
    /// This error occurs when a participant's revealed nonces don't match
    /// their previously committed values.
    CommitmentMismatch {
        /// Index of the participant with mismatched commitment
        participant_index: usize,
    },

    /// No nonces provided for aggregation
    ///
    /// This error occurs when trying to aggregate nonces but no nonces
    /// were provided.
    NoNonces,

    /// Cryptographic operation failed
    ///
    /// This error wraps underlying cryptographic errors from the secp256k1-zkp
    /// library.
    Crypto(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoParticipants => write!(f, "No participants in session"),
            Self::CommitmentMismatch { participant_index } => {
                write!(f, "Commitment mismatch for participant {participant_index}")
            }
            Self::NoNonces => write!(f, "No nonces provided for aggregation"),
            Self::Crypto(msg) => write!(f, "Cryptographic error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<secp256k1zkp::Error> for Error {
    fn from(e: secp256k1zkp::Error) -> Self {
        Self::Crypto(e.to_string())
    }
}
