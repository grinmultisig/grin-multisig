//! `MuSig2` Proof of Concept for Grin
//!
//! This library demonstrates the core `MuSig2` protocol concepts in a simplified form.
//! It is NOT production-ready and serves only as a conceptual demonstration.
//!
//! # Key Features
//!
//! - **Key Aggregation**: Combines multiple public keys using coefficients to prevent rogue key attacks
//! - **Two-Round Nonces**: Commitment protocol prevents adaptive attacks
//! - **Type Safety**: Uses newtype pattern for strong typing
//! - **Error Handling**: Proper Rust error types instead of strings
//!
//! # Example
//!
//! ```rust,no_run
//! use grin_multisig_poc::{Session, Participant, ParticipantId};
//! use secp256k1zkp::{Secp256k1, SecretKey, PublicKey};
//! use rand::thread_rng;
//!
//! let secp = Secp256k1::new();
//! let mut rng = thread_rng();
//!
//! // Create participants
//! let sk1 = SecretKey::new(&secp, &mut rng);
//! let pk1 = PublicKey::from_secret_key(&secp, &sk1)
//!     .expect("Failed to derive public key");
//! let p1 = Participant::new(ParticipantId::new(1), pk1);
//!
//! // Create session
//! let session = Session::new(vec![p1]);
//!
//! // Round 1: Generate nonces
//! let round1 = session.round1_generate_nonces().unwrap();
//!
//! // Get the commitment for sharing
//! let commitment = round1.commitment();
//!
//! // In a real scenario, you would:
//! // 1. Exchange commitments with other participants
//! // 2. Reveal public nonces and verify commitments
//! // 3. Aggregate nonces and compute partial signatures
//! ```
//!
//! # Note
//!
//! Uses Grin's `secp256k1-zkp` library (with Pedersen commitments and Bulletproofs support).
//! This is the same library used in Grin production.
//!
//! # References
//!
//! - [MuSig2 Paper](https://eprint.iacr.org/2020/1261)
//! - [Grin Documentation](https://github.com/mimblewimble/grin)

// Module declarations
mod error;
mod participant;
mod session;
mod types;

// Re-exports for public API
pub use error::{Error, Result};
pub use participant::Participant;
pub use session::{Round1State, Session};
pub use types::{Challenge, Coefficient, NonceCommitment, ParticipantId};
