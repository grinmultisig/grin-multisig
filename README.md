# Grin MultiSig Research Project

> **⚠️ ACADEMIC RESEARCH PROJECT ⚠️**
>
> This is an **open-source research and educational project** exploring cryptographic protocols.
> **NOT a commercial product. NOT seeking funding. NOT for production use.**

## 🎓 Project Status

**Research & Development** - Open-source community contribution

**⚠️ READ [NOTICE](./NOTICE) FILE BEFORE PROCEEDING**

## What is This?

An **academic research project** exploring multi-signature cryptographic protocols for the Grin cryptocurrency:

- **MuSig2**: N-of-N Schnorr signature aggregation
- **FROST**: Flexible t-of-n threshold signatures (future research)

### Research Goals

1. **Study** how Schnorr signature aggregation can be adapted to MimbleWimble
2. **Implement** proof-of-concept code for educational purposes
3. **Document** technical challenges and design decisions
4. **Share** findings with the cryptography and Grin communities

### Motivation

From a **technical curiosity perspective**:
- MuSig2 + MimbleWimble is cryptographically elegant
- Privacy-preserving multisig is an interesting research topic
- Hands-on implementation aids understanding of advanced cryptography

This is **NOT** driven by commercial interests - purely educational and technical learning.

## Technical Approach

### Key Principles
- ✅ **Wallet-layer only** - No protocol changes required
- ✅ **Privacy-preserving** - Aggregated signatures indistinguishable from single-sig
- ✅ **Efficient** - O(1) transaction size regardless of signers
- ✅ **Compatible** - Works with existing Grin Slate mechanism

### Architecture
```
Grin Wallet
├── libwallet (core logic)
│   ├── slate.rs → MultiSigSlate extension
│   └── multisig/
│       ├── musig2.rs (N-of-N implementation)
│       └── frost.rs (t-of-n implementation)
├── Controller (API layer)
└── Config (nonce storage, participant management)
```

## Resources

📖 **Documentation**
- [Why MuSig2 for Grin?](./docs/why-musig2-for-grin.md) - Technical rationale
- [Architecture Design](./docs/architecture-draft.md) - Coming soon

🔬 **Proof of Concept**
- [MuSig2 Rust Implementation](./poc/) - Complete working demo with:
  - Key aggregation with coefficient computation
  - Two-round nonce commitment protocol
  - Type-safe API design
  - Comprehensive test coverage
  - Full documentation

**Quick Start**:
```bash
cd poc
cargo run              # Run the demonstration
cargo test             # Run all tests
cargo doc --open       # View API documentation
```

💬 **Community**
- [Grin Forum Discussion](https://forum.grin.mw) - Technical discussions welcome
- Open to collaboration with other researchers

## Research Roadmap

**Note**: This is a **volunteer project** with **no fixed timeline**. Progress depends on available free time.

### Phase 1: Core Concepts (Completed)
- [x] MuSig2 proof-of-concept implementation
- [x] Comprehensive test suite (7 unit tests + documentation tests)
- [x] Using production `grin_secp256k1zkp` library
- [x] Type-safe Rust implementation

### Phase 2: Refinement (In Progress)
- [ ] Community feedback incorporation
- [ ] Expand test coverage and edge cases
- [ ] Technical documentation and tutorials
- [ ] Code review and improvements

### Phase 3: Advanced Research (Future, Optional)
- [ ] Study wallet integration challenges
- [ ] Explore Slatepack format extensions
- [ ] Investigate FROST threshold signatures
- [ ] Security analysis and threat modeling

**Disclaimer**: No guarantee of completion. This is educational work done in spare time.

## Get Involved

This is **open-source academic research** - peer review and collaboration welcome!

### How to Contribute

**For Researchers & Students**:
- 📖 Review the code and provide technical feedback
- 🐛 Test the implementation and report bugs
- 💡 Suggest improvements or alternative approaches
- 📚 Help improve documentation

**For Cryptographers**:
- 🔍 Security analysis and threat modeling
- 📝 Review cryptographic correctness
- 💬 Discuss implementation trade-offs

**For Grin Developers**:
- 🔧 Feedback on integration strategy
- 🤝 Collaborate on wallet architecture
- 🎓 Share knowledge about Grin internals

**Note**: This is **volunteer work**. No payments, no commitments - just knowledge sharing.

### Stay Updated

- ⭐ Star this repo to follow progress
- 📢 Watch for releases and updates
- 💬 Open GitHub issues for technical discussion

---

## Disclaimer & Legal

**⚠️ IMPORTANT - READ BEFORE USE**

This project is provided **"AS IS"** for **educational and research purposes ONLY**.

- **NOT production-ready** - Has NOT been security audited
- **NOT for real funds** - Use ONLY on testnets with test coins
- **NO WARRANTIES** - Authors assume NO LIABILITY for any damages
- **Legal compliance** - Users responsible for their jurisdiction's laws
- **NOT financial advice** - This is NOT an endorsement of any cryptocurrency

See [NOTICE](./NOTICE) file and [poc/README.md](./poc/README.md) for full disclaimer.

**By using this code, you agree to all terms.**

---

## Contact

**For technical discussions ONLY** (not support):
- **Email**: grin.multisig@proton.me
- **GitHub Issues**: [Project Issues](../../issues)

Responses provided on voluntary, best-effort basis.

---

## License

**Apache License 2.0** - Open source for research and educational use.

See [LICENSE](./LICENSE) file for full terms.

---

## Acknowledgments

This research builds upon:
- [MuSig2 Paper](https://eprint.iacr.org/2020/1261) by Nick, Ruffing, Seurin
- [Grin Project](https://github.com/mimblewimble/grin) - MimbleWimble implementation
- [secp256k1-zkp](https://github.com/mimblewimble/rust-secp256k1-zkp) - Cryptographic library

**Thank you** to the Grin community and open-source cryptography researchers.

---

**Last Updated**: January 2, 2026

_Academic research contribution to privacy-preserving cryptography_
