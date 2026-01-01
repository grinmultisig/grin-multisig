//! Participant data structures

use crate::types::ParticipantId;
use secp256k1zkp::PublicKey;
use serde::{Deserialize, Serialize};

/// Represents a participant in the `MuSig2` protocol
///
/// Each participant has a unique ID and a public key. The public key is used
/// in the multi-signature scheme.
///
/// # Example
///
/// ```rust,no_run
/// use grin_multisig_poc::{Participant, ParticipantId};
/// use secp256k1zkp::{Secp256k1, SecretKey, PublicKey};
/// use rand::thread_rng;
///
/// let secp = Secp256k1::new();
/// let mut rng = thread_rng();
///
/// // Create a new keypair
/// let sk = SecretKey::new(&secp, &mut rng);
/// let pk = PublicKey::from_secret_key(&secp, &sk)
///     .expect("Failed to derive public key");
///
/// // Create a participant
/// let participant = Participant::new(ParticipantId::new(1), pk);
/// assert_eq!(participant.id().inner(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// Participant's unique ID
    id: ParticipantId,

    /// Participant's public key
    #[serde(with = "hex_pubkey")]
    public_key: PublicKey,
}

impl Participant {
    /// Create a new participant
    pub const fn new(id: ParticipantId, public_key: PublicKey) -> Self {
        Self { id, public_key }
    }

    /// Get participant ID
    pub const fn id(&self) -> ParticipantId {
        self.id
    }

    /// Get participant's public key
    pub const fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

// Helper module for hex serialization of PublicKey
mod hex_pubkey {
    use secp256k1zkp::{PublicKey, Secp256k1};
    use serde::{Deserializer, Serializer};

    // Thread-local Secp256k1 context for serialization
    thread_local! {
        static SECP: Secp256k1 = Secp256k1::new();
    }

    pub fn serialize<S>(pubkey: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized_bytes = SECP.with(|secp| pubkey.serialize_vec(secp, true));
        serializer.serialize_str(&hex::encode(&serialized_bytes[..]))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(Error::custom)?;
        SECP.with(|secp| PublicKey::from_slice(secp, &bytes).map_err(Error::custom))
    }
}
