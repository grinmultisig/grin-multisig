# Grin MultiSig Implementation

> Building MuSig2/FROST wallet-layer multisig for Grin

## 🚧 Status

**Proposal Stage** - Seeking community funding

## What is This?

A comprehensive multisig implementation for Grin wallet using:
- **MuSig2**: N-of-N Schnorr signature aggregation
- **FROST**: Flexible t-of-n threshold signatures

## Why?

Grin currently lacks multisig functionality, which blocks critical use cases:
- 🔒 **OTC Escrow**: Secure peer-to-peer trading
- 🏦 **Institutional Custody**: M-of-N security for funds
- 🏛️ **Community Governance**: Secure multisig for community funds

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
- [MuSig2 Simple Demo](./poc/musig2_simple.rs) - Work in progress

💬 **Community**
- [Funding Proposal](https://forum.grin.mw/...) - Coming soon
- [Original Discussion](https://forum.grin.mw/t/funding-proposal-grin-multisig/11837)

## Roadmap

### Milestone 1: Foundation (Month 1-2)
- [ ] RFC document with detailed specification
- [ ] Core cryptographic libraries (MuSig2 + FROST)
- [ ] Comprehensive test suite

### Milestone 2: Integration (Month 3-4)
- [ ] Extend Grin Slate for multisig coordination
- [ ] Wallet API implementation
- [ ] Nonce management and security hardening

### Milestone 3: Production (Month 5-6)
- [ ] CLI interface and documentation
- [ ] Security audit preparation
- [ ] Real-world testing with community

## Get Involved

This project will be developed fully in the open. Contributions welcome!

**Stay Updated**:
- ⭐ Star this repo for updates
- 💬 Join discussion on [Grin Forum](https://forum.grin.mw)
- 🔑 Reach out on Keybase: [@grinmultisig](https://keybase.io/grinmultisig)

## Contact

- **Email**: grin.multisig@proton.me
- **Keybase**: @grinmultisig
- **Forum**: @grinmultisig

## License

Apache 2.0 (same as Grin)

---

_Built with ❤️ for the Grin community_
