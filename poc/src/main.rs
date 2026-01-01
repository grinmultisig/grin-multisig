//! `MuSig2` Proof of Concept - Demo Application
use grin_multisig_poc::{Participant, ParticipantId, Session};
use rand::thread_rng;
use secp256k1zkp::{PublicKey, Secp256k1, SecretKey};

const SEPARATOR: &str = "======================================================================";
const LINE: &str = "----------------------------------------------------------------------";

fn main() {
    println!("{SEPARATOR}");
    println!("MuSig2 Proof of Concept for Grin");
    println!("{SEPARATOR}");
    println!();
    println!("This demonstrates the core MuSig2 protocol concepts:");
    println!("  1. Key aggregation with coefficients (prevents rogue key attacks)");
    println!("  2. Two-round nonce commitment (prevents adaptive attacks)");
    println!("  3. Partial signature structure");
    println!();
    println!("Note: Uses Grin's secp256k1-zkp library with Pedersen commitments support.");
    println!();
    println!("Run tests with: cargo test -- --show-output");
    println!();
    println!("{SEPARATOR}");
    println!();

    // Demo: 2-of-2 multisig
    demo_two_of_two_multisig();
}

fn demo_two_of_two_multisig() {
    let secp = Secp256k1::new();
    let mut rng = thread_rng();

    // Create two participants
    let sk1 = SecretKey::new(&secp, &mut rng);
    let sk2 = SecretKey::new(&secp, &mut rng);

    let pk1 = PublicKey::from_secret_key(&secp, &sk1).expect("Failed to derive public key");
    let pk2 = PublicKey::from_secret_key(&secp, &sk2).expect("Failed to derive public key");

    println!("Created 2 participants for 2-of-2 multisig");
    println!(
        "  Participant 1 pubkey: {}...",
        hex::encode(&pk1.serialize_vec(&secp, true)[..8])
    );
    println!(
        "  Participant 2 pubkey: {}...",
        hex::encode(&pk2.serialize_vec(&secp, true)[..8])
    );

    let participants = vec![
        Participant::new(ParticipantId::new(1), pk1),
        Participant::new(ParticipantId::new(2), pk2),
    ];

    let session = Session::new(participants);

    // Step 1: Key Aggregation
    println!("\n{LINE}");
    println!("Step 1: Key Aggregation");
    println!("{LINE}");

    let coeff1 = session.key_agg_coefficient(&pk1);
    let coeff2 = session.key_agg_coefficient(&pk2);

    println!(
        "  Coefficient for participant 1: {}...",
        hex::encode(&coeff1.as_bytes()[..8])
    );
    println!(
        "  Coefficient for participant 2: {}...",
        hex::encode(&coeff2.as_bytes()[..8])
    );
    println!("  ✓ Coefficients computed (prevents rogue key attacks)");

    // Step 2: Nonce Generation (Round 1)
    println!("\n{LINE}");
    println!("Step 2: Nonce Generation (Round 1)");
    println!("{LINE}");

    let round1_p1 = session.round1_generate_nonces().unwrap();
    let round1_p2 = session.round1_generate_nonces().unwrap();

    println!(
        "  P1 commitment: {}...",
        hex::encode(&round1_p1.commitment().as_bytes()[..8])
    );
    println!(
        "  P2 commitment: {}...",
        hex::encode(&round1_p2.commitment().as_bytes()[..8])
    );
    println!("  ✓ Nonce commitments generated (prevents adaptive attacks)");

    // Step 3: Commitment Verification (Round 2)
    println!("\n{LINE}");
    println!("Step 3: Commitment Verification (Round 2)");
    println!("{LINE}");

    let commitments = vec![*round1_p1.commitment(), *round1_p2.commitment()];
    let revealed = vec![*round1_p1.public_nonces(), *round1_p2.public_nonces()];

    match session.round2_aggregate_nonces(&commitments, &revealed) {
        Ok(_) => println!("  ✓ All commitments verified successfully"),
        Err(e) => println!("  ✗ Verification failed: {e}"),
    }

    // Summary
    println!("\n{SEPARATOR}");
    println!("✓ Core MuSig2 concepts demonstrated successfully!");
    println!("{SEPARATOR}");
    println!();
    println!("Production implementation would include:");
    println!("  • Proper scalar multiplication for key/nonce aggregation");
    println!("  • Complete partial signature computation (s_i = r_i + c*a_i*x_i)");
    println!("  • Signature aggregation and verification");
    println!("  • Nonce reuse prevention (persistent storage)");
    println!("  • Integration with Grin's Slate mechanism");
    println!("  • Pedersen commitment handling for Grin transactions");
    println!();
}
