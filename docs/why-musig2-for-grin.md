# Why MuSig2 is Perfect for Grin

## Background

Grin uses **Schnorr signatures** as part of the MimbleWimble protocol. This creates a unique opportunity for elegant multisig implementation.

## The Problem

Currently, Grin lacks multisig functionality, which blocks:
- **OTC platforms** needing escrow (e.g., connecting miners with buyers)
- **Institutional custody** requiring M-of-N security
- **Community fund management** (current council lacks secure multisig)

## Why MuSig2?

### 1. Native Compatibility
- Grin already uses Schnorr signatures
- MuSig2 is designed specifically for Schnorr aggregation
- No protocol-level changes needed (wallet-layer only)

### 2. Privacy Preservation
- Aggregated signature looks identical to single signature
- Transaction size remains O(1), not O(n)
- Maintains Grin's core privacy properties

### 3. Security Properties
- Provably secure under DL assumption
- Resistant to rogue key attacks (via key aggregation coefficients)
- 2-round protocol with 3-round security

## Technical Approach

### Key Aggregation
```
X_agg = Σ (a_i * X_i)

where a_i = H(L || X_i)
      L = H(X_1 || X_2 || ... || X_n)
```

This prevents rogue key attacks without requiring proofs of possession.

### Signature Flow
1. **Round 1**: Nonce commitment - each party commits to H(R_i)
2. **Round 2**: Nonce reveal - parties reveal R_i, verify commitments
3. **Round 3**: Partial signatures - compute s_i = r_i + c * a_i * x_i
4. **Aggregation**: s = Σ s_i (final signature)

### Integration with Grin Slate
Extend existing Slate mechanism:
```rust
pub struct MultiSigSlate {
    base_slate: Slate,
    participants: Vec<ParticipantData>,
    nonce_commitments: Vec<[u8; 32]>,
    // ... additional fields
}
```

## Challenges

1. **Bulletproof aggregation**: Range proofs need special handling
   - Phase 1: Single-party generation (requires trust)
   - Phase 2: MPC bulletproofs (future research)

2. **Nonce reuse prevention**: Critical security requirement
   - Deterministic + random hybrid approach
   - Persistent nonce tracking in wallet DB

3. **Network coordination**: Interactive protocol needs reliable communication
   - Slatepack encoding for async transport
   - Timeout and retry mechanisms

## Why Not Alternatives?

### Bitcoin-style Script Multisig
- ❌ Grin has no script language
- ❌ Would require protocol changes
- ❌ Increases transaction size

### Threshold EdDSA (like Monero)
- ❌ Grin uses secp256k1, not Ed25519
- ❌ Different curve arithmetic

### Simple MPC
- ❌ Higher round complexity
- ❌ Doesn't leverage Schnorr aggregation

## Conclusion

MuSig2 is the natural choice for Grin:
- Leverages existing Schnorr infrastructure
- Maintains privacy and efficiency
- Wallet-layer implementation (no consensus changes)
- Well-studied with formal security proofs

## References
- [MuSig2 Paper](https://eprint.iacr.org/2020/1261)
- [Grin Forum Discussion](https://forum.grin.mw/t/funding-proposal-grin-multisig/11837)
- [Schnorr Signatures in Grin](https://github.com/mimblewimble/grin/blob/master/doc/intro.md)

---

**Author**: @grinmultisig
**Contact**: grin.multisig@proton.me
