<div align="center">

<img src="./banner.png" alt="eclipx banner" width="100%" />

<br />
<br />

<a href="https://github.com/eclipxlabs/eclipx/actions/workflows/ci.yml">
  <img src="https://img.shields.io/github/actions/workflow/status/eclipxlabs/eclipx/ci.yml?branch=main&style=flat-square&label=CI&logo=githubactions&logoColor=white" alt="CI" />
</a>
<a href="https://github.com/eclipxlabs/eclipx/blob/main/LICENSE">
  <img src="https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square" alt="License" />
</a>
<img src="https://img.shields.io/badge/rust-1.75+-93450a?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
<img src="https://img.shields.io/badge/solana-1.18-9945FF?style=flat-square&logo=solana&logoColor=white" alt="Solana" />
<a href="https://eclipx.tech">
  <img src="https://img.shields.io/badge/website-eclipx.tech-8b5cf6?style=flat-square" alt="Website" />
</a>
<a href="https://x.com/eclipxtech">
  <img src="https://img.shields.io/badge/@eclipxtech-000000?style=flat-square&logo=x&logoColor=white" alt="X" />
</a>

<br />
<br />

<h3>Zero-knowledge privacy infrastructure for Solana. Stealth RPC tunneling, path obfuscation, and ZK-compressed transactions.</h3>

</div>

---

## Usage

### TypeScript SDK

```typescript
import { EclipxClient } from "@eclipxlabs/eclipx-sdk";

const client = new EclipxClient({
  rpcEndpoint: "https://api.mainnet-beta.solana.com",
  programId: "Ec1pXmjiR5DVFhbYr4urnUdFKNhKwgjJXwbkFCfCa5sM",
});

const score = await client.getPrivacyScore(walletPublicKey);
console.log("Privacy Score:", score);

const state = await client.getPrivacyState(walletPublicKey);
if (state?.isActive) {
  console.log("Stealth mode is active");
}
```

### CLI

```bash
# Check relay network status
eclipx status --rpc https://api.mainnet-beta.solana.com

# Initialize privacy state
eclipx init --relay-count 5 --obfuscation 3

# Activate stealth mode
eclipx activate

# Check privacy score
eclipx score <WALLET_ADDRESS>
```

## Architecture

```mermaid
graph LR
    A[User Wallet] -->|encrypt| B[ECLIPX Relay Network]
    B -->|multi-hop| C[ZK-Compress Engine]
    C -->|hash only| D[Solana Mainnet]
    B -->|full data| E[Shadow Ledger]

    style A fill:#ef4444,stroke:#ef4444,color:#fff
    style B fill:#8b5cf6,stroke:#8b5cf6,color:#fff
    style C fill:#3b82f6,stroke:#3b82f6,color:#fff
    style D fill:#22c55e,stroke:#22c55e,color:#fff
    style E fill:#6b21a8,stroke:#6b21a8,color:#fff
```

| Component | Language | Description |
|-----------|----------|-------------|
| `programs/eclipx_core` | Rust | On-chain ZK-state compression program (Anchor) |
| `libs/eclipx_math` | Rust | Mathematical primitives for scoring and interpolation |
| `cli` | Rust | Command-line interface for relay node management |
| `sdk` | TypeScript | Client SDK for interacting with the protocol |

## Installation

```bash
git clone https://github.com/eclipxlabs/eclipx.git
cd eclipx
```

### Build On-Chain Programs

```bash
cargo build
cargo test
```

### SDK Setup

```bash
cd sdk
npm install
npm run build
```

### CLI

```bash
cargo install --path cli
eclipx status
eclipx score <WALLET_ADDRESS>
```

## Features

| Feature | Status | Description |
|---------|--------|-------------|
| ZK-State Compression | Active | Compress transaction data to on-chain verification hashes |
| Private RPC Tunneling | Active | Bypass public mempool via encrypted relay channels |
| Path Obfuscation | Active | Multi-hop routing across 3-7 randomized relay nodes |
| Anti-MEV Protection | Active | Direct validator submission eliminates sandwich attacks |
| Shadow Ledger | Active | Encrypted full transaction records accessible only by sender |
| Privacy Scoring | Active | Real-time wallet exposure analysis (0-1000 scale) |

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

<!-- touch: e23e34c1 -->

<!-- touch: 7ee52983 -->

<!-- touch: dc2429d8 -->

<!-- touch: e4fc53a0 -->

<!-- touch: 4aadf93d -->

<!-- touch: 6d97410d -->

<!-- touch: 8a78a051 -->

<!-- touch: f0fff0bf -->

<!-- touch: 3737fd8f -->

<!-- touch: 206dde94 -->

<!-- touch: 4d3e559c -->

<!-- touch: 531fb2b2 -->
