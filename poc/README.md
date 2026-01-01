# MuSig2 Proof of Concept for Grin

> **⚠️ ACADEMIC RESEARCH PROJECT ⚠️**
>
> This is a **research and educational project** exploring cryptographic protocols.
> **NOT intended for production use or commercial purposes.**
> See [Disclaimer](#disclaimer) section below.

A Rust implementation demonstrating core MuSig2 multi-signature protocol concepts for Grin blockchain.

## Overview

This library provides a simplified implementation of the MuSig2 protocol, focusing on **educational value** and clear demonstration of the key concepts. This project is part of academic research into Schnorr signature aggregation within the MimbleWimble framework.

### Project Nature

- **Type**: Open-source academic research
- **Purpose**: Educational and technical learning
- **Status**: Proof-of-Concept (experimental)
- **Audience**: Cryptography students and researchers
- **License**: Open source for peer review

## Features Demonstrated

- **Key Aggregation** - Combines multiple public keys using coefficients to prevent rogue key attacks
- **Two-Round Nonces** - Commitment scheme prevents adaptive attacks (Wagner's attack)
- **Type Safety** - Uses newtype pattern for cryptographic primitives
- **Error Handling** - Proper Rust error types with detailed context

## Quick Start

```bash
# Run the demonstration
cargo run

# Run all tests
cargo test

# Run tests with detailed output
cargo test -- --show-output
```

## Example Output

```
======================================================================
MuSig2 Proof of Concept for Grin
======================================================================

Created 2 participants for 2-of-2 multisig
  Participant 1 pubkey: 03bc8a665a510507...
  Participant 2 pubkey: 02721f710732053e...

----------------------------------------------------------------------
Step 1: Key Aggregation
----------------------------------------------------------------------
  Coefficient for participant 1: 401211ef3bc4f7d4...
  Coefficient for participant 2: c28848c96d669c06...
  ✓ Coefficients computed (prevents rogue key attacks)

----------------------------------------------------------------------
Step 2: Nonce Generation (Round 1)
----------------------------------------------------------------------
  P1 commitment: 57fd3309bc82c4ee...
  P2 commitment: 6e512eab06f226e4...
  ✓ Nonce commitments generated (prevents adaptive attacks)

----------------------------------------------------------------------
Step 3: Commitment Verification (Round 2)
----------------------------------------------------------------------
  ✓ All commitments verified successfully
```

## API Usage

```rust
use grin_multisig_poc::{Session, Participant, ParticipantId};
use secp256k1zkp::{Secp256k1, SecretKey, PublicKey};
use rand::thread_rng;

// Create participants
let secp = Secp256k1::new();
let mut rng = thread_rng();
let sk = SecretKey::new(&secp, &mut rng);
let pk = PublicKey::from_secret_key(&secp, &sk)
    .expect("Failed to derive public key");
let participant = Participant::new(ParticipantId::new(1), pk);

// Create session
let session = Session::new(vec![participant]);

// Round 1: Generate nonces
let round1 = session.round1_generate_nonces().unwrap();
let commitment = round1.commitment();
```

## Implementation Details

### Core Types

- **`Coefficient`** - Key aggregation coefficient (32 bytes)
- **`NonceCommitment`** - Hash commitment for nonces (64 bytes)
- **`Challenge`** - Signature challenge hash (32 bytes)
- **`Participant`** - Protocol participant with ID and public key
- **`Session`** - Main protocol coordinator

### Protocol Flow

1. **Setup**: Create participants and session
2. **Round 1**: Generate nonce commitments
3. **Round 2**: Exchange and verify nonces
4. **Round 3**: Compute partial signatures (PoC only computes challenge)

## Important Notes

⚠️ **RESEARCH PROTOTYPE - NOT FOR PRODUCTION USE**

This is a **Proof of Concept for academic research purposes only**.

### What's Working (Educational Demonstration)
- ✅ Key aggregation coefficients computation
- ✅ Nonce generation and commitment verification
- ✅ Type-safe API design patterns
- ✅ Comprehensive test coverage

### What's Simplified (PoC Limitations)
- ❌ Key aggregation uses placeholder (needs proper elliptic curve point multiplication)
- ❌ Partial signatures only compute challenge hash (not actual signatures)
- ❌ No signature aggregation or verification implemented
- ❌ No nonce reuse prevention mechanisms
- ❌ No security audit performed

### ⚠️ Critical Warnings
- **DO NOT** use with real cryptocurrency funds
- **DO NOT** use in production environments
- **DO NOT** consider this cryptographically secure
- **FOR LEARNING AND RESEARCH ONLY**

See full [Disclaimer](#disclaimer) below for legal terms.

## Testing

The project includes comprehensive tests:

```bash
# Run all tests
cargo test

# Specific test categories
cargo test session::tests::test_key_aggregation_coefficient_deterministic
cargo test session::tests::test_commitment_verification_success
cargo test session::tests::test_challenge_computation
```

## Documentation

Full API documentation is available at:
```bash
cargo doc --open
```

## References

- [MuSig2: Multi-Signature Scheme](https://eprint.iacr.org/2020/1261) - Academic paper
- [Grin Project](https://github.com/mimblewimble/grin) - MimbleWimble implementation
- [secp256k1-zkp](https://github.com/mimblewimble/secp256k1-zkp) - Grin's crypto library

---

## Disclaimer

### ⚠️ Research and Educational Purpose Only

This project is provided **"AS IS"** for **academic research and educational purposes only**. By accessing, viewing, or using this code, you acknowledge and agree to the following:

#### 1. **Not Production Ready**
- This is a **Proof-of-Concept** implementation
- Has NOT undergone professional security audit
- NOT intended for use with real funds or in production environments
- May contain bugs, vulnerabilities, or incomplete features

#### 2. **No Warranties**
- Provided without warranties of any kind, either express or implied
- No guarantee of fitness for any particular purpose
- No guarantee of correctness, security, or reliability
- Use at your own risk

#### 3. **Educational and Research Nature**
- This project is part of **academic research** into cryptographic protocols
- Specifically studying Schnorr signature aggregation (MuSig2) in the context of MimbleWimble
- Similar to publishing a research paper or academic thesis
- Goal is to advance understanding of privacy-preserving cryptography

#### 4. **Not Financial Advice or Service**
- This is **NOT**:
  - Financial advice or investment recommendation
  - A commercial product or service
  - An endorsement of any cryptocurrency
  - A tool for conducting cryptocurrency transactions
- Do **NOT** use this code to manage real cryptocurrency funds

#### 5. **Legal Compliance**
- Users are responsible for ensuring compliance with their local laws and regulations
- Cryptocurrency regulations vary by jurisdiction
- Some jurisdictions may restrict or prohibit cryptocurrency-related activities
- Consult legal counsel in your jurisdiction before any cryptocurrency-related activities

#### 6. **No Liability**
- The author(s) shall NOT be liable for any damages, losses, or legal consequences arising from:
  - Use or misuse of this code
  - Bugs, errors, or security vulnerabilities
  - Financial losses from cryptocurrency transactions
  - Violation of laws or regulations in any jurisdiction
- You assume full responsibility for any use of this code

#### 7. **Third-Party Code**
- This project uses third-party libraries (secp256k1-zkp, etc.)
- Third-party code is subject to their respective licenses
- No warranty is provided for third-party dependencies

#### 8. **Testing Environment Only**
- If you choose to experiment with this code:
  - Use ONLY on cryptocurrency testnets (never mainnet)
  - Use ONLY with test funds (no real value)
  - Understand the risks before proceeding

#### 9. **Open Source Collaboration**
- This project welcomes peer review and constructive feedback
- Contributions should be for educational/research purposes
- Contributors agree to the same disclaimer terms

#### 10. **Changes and Updates**
- This project may be modified, updated, or discontinued at any time
- No guarantee of ongoing maintenance or support
- No obligation to fix bugs or implement features

---

### 🎓 Academic Context

This research project investigates:
- **Research Question**: How can MuSig2 Schnorr signature aggregation be adapted to the MimbleWimble protocol framework?
- **Methodology**: Implement a proof-of-concept in Rust using production cryptographic libraries
- **Goal**: Demonstrate technical feasibility and identify implementation challenges
- **Outcome**: Open-source code publication for peer review and community feedback

**This work is in the spirit of academic research** - contributing to the body of knowledge in privacy-preserving cryptography.

---

### 📧 Contact

For **technical discussions only**:
- GitHub Issues: [Repository Issues]
- Email: grin.multisig@proton.me

**Note**: This is not a support channel. Responses are provided on a voluntary, best-effort basis.

---

### 📄 License

This project is part of the Grin ecosystem and follows the same licensing terms.

**Apache License 2.0** - Open source for research, educational, and non-commercial use.

See [LICENSE](../LICENSE) file for full terms.

---

**Last Updated**: January 2, 2026

**By using this code, you acknowledge that you have read, understood, and agree to this disclaimer.**
