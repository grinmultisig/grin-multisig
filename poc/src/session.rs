//! `MuSig2` signing session

use blake2::{Blake2b512, Digest};
use rand::thread_rng;
use secp256k1zkp::{PublicKey, Secp256k1, SecretKey};

use crate::error::{Error, Result};
use crate::participant::Participant;
use crate::types::{Challenge, Coefficient, NonceCommitment};

/// `MuSig2` Round 1 state (nonce commitment phase)
#[derive(Debug, Clone)]
pub struct Round1State {
    /// Secret nonces (r1, r2) - MUST be kept private!
    secret_nonces: (SecretKey, SecretKey),

    /// Public nonces (R1, R2)
    public_nonces: (PublicKey, PublicKey),

    /// Commitment to public nonces H(R1, R2)
    commitment: NonceCommitment,
}

impl Round1State {
    /// Get the secret nonces (use with caution!)
    pub const fn secret_nonces(&self) -> &(SecretKey, SecretKey) {
        &self.secret_nonces
    }

    /// Get the public nonces
    pub const fn public_nonces(&self) -> &(PublicKey, PublicKey) {
        &self.public_nonces
    }

    /// Get the nonce commitment
    pub const fn commitment(&self) -> &NonceCommitment {
        &self.commitment
    }
}

/// `MuSig2` signing session
///
/// Manages the complete `MuSig2` protocol flow:
/// 1. Key aggregation with coefficients
/// 2. Two-round nonce commitment
/// 3. Partial signature generation
pub struct Session {
    /// All participants' public keys
    participants: Vec<Participant>,

    /// Secp256k1 context
    secp: Secp256k1,
}

impl Session {
    /// Create a new `MuSig2` session
    ///
    /// # Arguments
    /// * `participants` - List of participants with their public keys
    ///
    /// # Returns
    /// New session instance
    pub fn new(participants: Vec<Participant>) -> Self {
        Self {
            participants,
            secp: Secp256k1::new(),
        }
    }

    /// Get the number of participants
    pub const fn participant_count(&self) -> usize {
        self.participants.len()
    }

    /// Compute key aggregation coefficient: `a_i` = H(L || `X_i`)
    ///
    /// This prevents rogue key attacks without requiring proofs of possession.
    ///
    /// # Algorithm
    /// ```text
    /// L = H(X_1 || X_2 || ... || X_n)  // Hash of all public keys
    /// a_i = H(L || X_i)                 // Coefficient for each key
    /// ```
    ///
    /// # Arguments
    /// * `pubkey` - The public key to compute coefficient for
    ///
    /// # Returns
    /// 32-byte coefficient `a_i`
    pub fn key_agg_coefficient(&self, pubkey: &PublicKey) -> Coefficient {
        // Step 1: Compute L = H(X_1 || X_2 || ... || X_n)
        let mut hasher = Blake2b512::new();
        for participant in &self.participants {
            let serialized = participant.public_key().serialize_vec(&self.secp, true);
            hasher.update(&serialized[..]);
        }
        let l_hash = hasher.finalize();

        // Step 2: Compute a_i = H(L || X_i)
        let mut hasher = Blake2b512::new();
        hasher.update(l_hash);
        let serialized = pubkey.serialize_vec(&self.secp, true);
        hasher.update(&serialized[..]);
        let result = hasher.finalize();

        // Take first 32 bytes as scalar
        let mut coefficient = [0u8; 32];
        coefficient.copy_from_slice(&result[..32]);

        Coefficient::new(coefficient)
    }

    /// Aggregate public keys: `X_agg` = `sum(a_i` * `X_i`)
    ///
    /// # Note
    /// This is a simplified `PoC` implementation.
    /// Production requires proper scalar multiplication.
    ///
    /// # Returns
    /// Aggregated public key (simplified for `PoC`)
    ///
    /// # Errors
    /// Returns `Error::NoParticipants` if no participants are in the session
    pub fn aggregate_pubkeys(&self) -> Result<PublicKey> {
        if self.participants.is_empty() {
            return Err(Error::NoParticipants);
        }

        // For PoC: return first participant's key
        // TODO: Implement proper aggregation
        // X_agg = sum(a_i * X_i) for all participants

        #[cfg(debug_assertions)]
        {
            eprintln!("⚠️  Note: Using simplified aggregation for PoC");
            eprintln!("   Production needs: X_agg = sum(a_i * X_i)");
        }

        Ok(*self.participants[0].public_key())
    }

    /// Round 1: Generate nonce commitment
    ///
    /// Each participant generates two random nonces and commits to them.
    /// The two-nonce design prevents adaptive attacks (Wagner's attack).
    ///
    /// # Security Note
    /// - Nonces MUST be random and never reused
    /// - Commitment prevents other parties from adapting their nonces
    ///
    /// # Returns
    /// `Round1State` containing secret nonces, public nonces, and commitment
    ///
    /// # Errors
    /// Returns `Error::Crypto` if nonce generation fails
    pub fn round1_generate_nonces(&self) -> Result<Round1State> {
        let mut rng = thread_rng();

        // Generate two random secret nonces (security requirement for MuSig2)
        let secret_nonce1 = SecretKey::new(&self.secp, &mut rng);
        let secret_nonce2 = SecretKey::new(&self.secp, &mut rng);

        // Compute public nonces R1 = r1 * G, R2 = r2 * G
        let public_nonce1 = PublicKey::from_secret_key(&self.secp, &secret_nonce1)?;
        let public_nonce2 = PublicKey::from_secret_key(&self.secp, &secret_nonce2)?;

        // Compute commitment H(R1 || R2)
        let commitment = NonceCommitment::from_nonces(&self.secp, &public_nonce1, &public_nonce2);

        Ok(Round1State {
            secret_nonces: (secret_nonce1, secret_nonce2),
            public_nonces: (public_nonce1, public_nonce2),
            commitment,
        })
    }

    /// Round 2: Verify commitments and aggregate nonces
    ///
    /// After all participants reveal their nonces:
    /// 1. Verify each nonce against its commitment
    /// 2. Aggregate nonces: R = `sum(R1_i` + `R2_i`)
    ///
    /// # Arguments
    /// * `commitments` - Commitments from Round 1
    /// * `revealed_nonces` - Public nonces revealed in Round 2
    ///
    /// # Returns
    /// Aggregated nonce R (simplified for `PoC`)
    ///
    /// # Errors
    /// Returns `Error::NoNonces` if no nonces are provided
    /// Returns `Error::CommitmentMismatch` if any commitment doesn't match
    pub fn round2_aggregate_nonces(
        &self,
        commitments: &[NonceCommitment],
        revealed_nonces: &[(PublicKey, PublicKey)],
    ) -> Result<PublicKey> {
        if revealed_nonces.is_empty() {
            return Err(Error::NoNonces);
        }

        // Verify commitments match revealed nonces
        for (i, (nonce1, nonce2)) in revealed_nonces.iter().enumerate() {
            if !commitments[i].verify(&self.secp, nonce1, nonce2) {
                return Err(Error::CommitmentMismatch {
                    participant_index: i,
                });
            }
        }

        // Aggregate nonces: R = sum(R1_i + R2_i) for all participants
        // For PoC: return first nonce
        // TODO: Implement proper point addition

        #[cfg(debug_assertions)]
        {
            eprintln!("⚠️  Note: Using simplified nonce aggregation for PoC");
            eprintln!("   Production needs: R = sum(R1_i + R2_i)");
        }

        Ok(revealed_nonces[0].0)
    }

    /// Round 3: Compute challenge hash (partial signature structure)
    ///
    /// In a complete implementation, this would compute:
    /// `s_i` = `r_i` + c * `a_i` * `x_i`
    ///
    /// Where:
    /// - `r_i`: secret nonce
    /// - c: challenge hash
    /// - `a_i`: key aggregation coefficient
    /// - `x_i`: secret key
    ///
    /// # Note
    /// This `PoC` only computes the challenge hash.
    /// Production requires proper scalar arithmetic.
    ///
    /// # Arguments
    /// * `message` - Message to sign (32 bytes)
    /// * `_secret_key` - Participant's secret key (unused in `PoC`)
    /// * `_secret_nonce` - Secret nonce from Round 1 (unused in `PoC`)
    /// * `agg_nonce` - Aggregated nonce from Round 2
    /// * `agg_pubkey` - Aggregated public key
    ///
    /// # Returns
    /// Challenge hash c = `H(X_agg` || R || m)
    ///
    /// # Errors
    /// Returns `Error::Crypto` if challenge computation fails
    pub fn round3_partial_sign(
        &self,
        message: &[u8; 32],
        _secret_key: &SecretKey,
        _secret_nonce: &SecretKey,
        agg_nonce: &PublicKey,
        agg_pubkey: &PublicKey,
    ) -> Result<Challenge> {
        // Compute challenge: c = H(X_agg || R || m)
        let challenge = Challenge::from_message(&self.secp, agg_pubkey, agg_nonce, message);

        #[cfg(debug_assertions)]
        {
            eprintln!("⚠️  Note: Returning challenge hash only (PoC)");
            eprintln!("   Production needs: s_i = r_i + c * a_i * x_i");
        }

        // In production: compute s_i = r_i + c * a_i * x_i
        // using proper scalar arithmetic

        Ok(challenge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ParticipantId;

    fn create_test_session(n_participants: u32) -> Session {
        let secp = Secp256k1::new();
        let mut rng = thread_rng();

        let participants: Vec<_> = (0..n_participants)
            .map(|i| {
                let sk = SecretKey::new(&secp, &mut rng);
                let pk =
                    PublicKey::from_secret_key(&secp, &sk).expect("Failed to derive public key");
                Participant::new(ParticipantId::new(i), pk)
            })
            .collect();

        Session::new(participants)
    }

    #[test]
    fn test_key_aggregation_coefficient_deterministic() {
        let session = create_test_session(2);
        let pk = session.participants[0].public_key();

        let coeff1 = session.key_agg_coefficient(pk);
        let coeff2 = session.key_agg_coefficient(pk);

        assert_eq!(coeff1, coeff2, "Coefficients should be deterministic");
    }

    #[test]
    fn test_key_aggregation_coefficient_unique() {
        let session = create_test_session(2);
        let pk1 = session.participants[0].public_key();
        let pk2 = session.participants[1].public_key();

        let coeff1 = session.key_agg_coefficient(pk1);
        let coeff2 = session.key_agg_coefficient(pk2);

        assert_ne!(
            coeff1, coeff2,
            "Different keys should have different coefficients"
        );
    }

    #[test]
    fn test_nonce_generation_and_commitment() {
        let secp = Secp256k1::new();
        let session = create_test_session(1);
        let round1 = session.round1_generate_nonces().unwrap();

        // Verify commitment matches public nonces
        let (r1, r2) = round1.public_nonces();
        assert!(
            round1.commitment().verify(&secp, r1, r2),
            "Commitment should match public nonces"
        );
    }

    #[test]
    fn test_commitment_verification_success() {
        let session = create_test_session(1);
        let round1 = session.round1_generate_nonces().unwrap();

        let commitments = vec![*round1.commitment()];
        let revealed = vec![*round1.public_nonces()];

        let result = session.round2_aggregate_nonces(&commitments, &revealed);
        assert!(result.is_ok(), "Valid commitments should verify");
    }

    #[test]
    fn test_commitment_verification_failure() {
        let session = create_test_session(1);
        let round1 = session.round1_generate_nonces().unwrap();
        let round2 = session.round1_generate_nonces().unwrap();

        // Use round1 commitment but round2 nonces (mismatch)
        let commitments = vec![*round1.commitment()];
        let revealed = vec![*round2.public_nonces()];

        let result = session.round2_aggregate_nonces(&commitments, &revealed);
        assert!(
            matches!(result, Err(Error::CommitmentMismatch { .. })),
            "Mismatched commitments should fail"
        );
    }

    #[test]
    fn test_challenge_computation() {
        let secp = Secp256k1::new();
        let mut rng = thread_rng();

        let sk = SecretKey::new(&secp, &mut rng);
        let pk = PublicKey::from_secret_key(&secp, &sk).expect("Failed to derive public key");

        let session = create_test_session(1);
        let round1 = session.round1_generate_nonces().unwrap();

        let message = [0x42u8; 32];
        let challenge = session
            .round3_partial_sign(
                &message,
                &sk,
                &round1.secret_nonces().0,
                &round1.public_nonces().0,
                &pk,
            )
            .unwrap();

        assert_eq!(
            challenge.as_bytes().len(),
            32,
            "Challenge should be 32 bytes"
        );
    }

    #[test]
    fn test_empty_participants() {
        let session = Session::new(vec![]);
        assert!(
            matches!(session.aggregate_pubkeys(), Err(Error::NoParticipants)),
            "Should fail with no participants"
        );
    }
}
