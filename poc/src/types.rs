//! Type definitions for `MuSig2` protocol

use blake2::{Blake2b512, Digest};
use secp256k1zkp::{PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};

/// Key aggregation coefficient (32 bytes)
///
/// Coefficients are computed as `a_i = H(L || X_i)` where:
/// - `L` is the hash of all public keys
/// - `X_i` is the participant's public key
///
/// # Example
///
/// ```rust
/// use grin_multisig_poc::Coefficient;
///
/// let coeff = Coefficient::new([0u8; 32]);
/// assert_eq!(coeff.as_bytes().len(), 32);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coefficient([u8; 32]);

impl Coefficient {
    /// Create a new coefficient from raw bytes
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get the raw bytes
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsRef<[u8]> for Coefficient {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for Coefficient {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

/// Nonce commitment (64 bytes from Blake2b-512)
///
/// Commitments are computed as `H(R1 || R2)` where:
/// - `R1`, `R2` are the public nonces
/// - `H` is the Blake2b-512 hash function
///
/// # Example
///
/// ```rust
/// use grin_multisig_poc::NonceCommitment;
///
/// let commitment = NonceCommitment::new([0u8; 64]);
/// assert_eq!(commitment.as_bytes().len(), 64);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonceCommitment([u8; 64]);

impl NonceCommitment {
    /// Create a new nonce commitment
    pub const fn new(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Compute commitment from public nonces
    pub fn from_nonces(secp: &Secp256k1, r1: &PublicKey, r2: &PublicKey) -> Self {
        let mut hasher = Blake2b512::new();
        let serialized1 = r1.serialize_vec(secp, true);
        let serialized2 = r2.serialize_vec(secp, true);
        hasher.update(&serialized1[..]);
        hasher.update(&serialized2[..]);
        let hash = hasher.finalize();

        let mut bytes = [0u8; 64];
        bytes.copy_from_slice(&hash[..64]);
        Self(bytes)
    }

    /// Get the raw bytes
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }

    /// Verify commitment matches public nonces
    pub fn verify(&self, secp: &Secp256k1, r1: &PublicKey, r2: &PublicKey) -> bool {
        let computed = Self::from_nonces(secp, r1, r2);
        self == &computed
    }
}

impl AsRef<[u8]> for NonceCommitment {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 64]> for NonceCommitment {
    fn from(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }
}

/// Challenge hash (32 bytes)
///
/// Challenges are computed as `c = H(X_agg || R || m)` where:
/// - `X_agg` is the aggregated public key
/// - `R` is the aggregated nonce
/// - `m` is the message to be signed
/// - `H` is the Blake2b-512 hash function
///
/// # Example
///
/// ```rust
/// use grin_multisig_poc::Challenge;
///
/// let challenge = Challenge::new([0u8; 32]);
/// assert_eq!(challenge.as_bytes().len(), 32);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Challenge([u8; 32]);

impl Challenge {
    /// Create a new challenge
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Compute challenge: c = `H(X_agg` || R || m)
    pub fn from_message(
        secp: &Secp256k1,
        agg_pubkey: &PublicKey,
        agg_nonce: &PublicKey,
        message: &[u8; 32],
    ) -> Self {
        let mut hasher = Blake2b512::new();
        let serialized_pubkey = agg_pubkey.serialize_vec(secp, true);
        let serialized_nonce = agg_nonce.serialize_vec(secp, true);
        hasher.update(&serialized_pubkey[..]);
        hasher.update(&serialized_nonce[..]);
        hasher.update(message);
        let hash = hasher.finalize();

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&hash[..32]);
        Self(bytes)
    }

    /// Get the raw bytes
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsRef<[u8]> for Challenge {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for Challenge {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

/// Participant identifier
///
/// Unique identifier for each participant in the `MuSig2` protocol.
///
/// # Example
///
/// ```rust
/// use grin_multisig_poc::ParticipantId;
///
/// let id = ParticipantId::new(1);
/// assert_eq!(id.inner(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ParticipantId(pub u32);

impl ParticipantId {
    /// Create a new participant ID
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the inner value
    pub const fn inner(&self) -> u32 {
        self.0
    }
}

impl From<u32> for ParticipantId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl fmt::Display for ParticipantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

use std::fmt;
