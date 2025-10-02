# Changelog

All notable changes to eclipx are documented here.

## [0.4.2] - 2026-03-02

### Changed
- Optimized relay node selection algorithm for lower latency
- Improved ZK proof generation throughput by 18%

### Fixed
- Corrected padding alignment in privacy state account
- Resolved race condition in multi-hop relay path construction

## [0.4.0] - 2026-02-15

### Added
- WebSocket streaming API for real-time relay events
- Wallet privacy exposure analysis endpoint
- Node health monitoring with regional failover

### Changed
- Migrated ZK circuit from Bellman to Groth16 backend
- Reduced on-chain account size by 24 bytes

## [0.3.0] - 2026-01-10

### Added
- TypeScript SDK with full type coverage
- CLI tool for relay node management
- Integration test suite against devnet

## [0.2.0] - 2025-11-20

### Added
- Multi-hop relay path obfuscation engine
- ChaCha20-Poly1305 payload encryption layer
- Private RPC tunnel for anti-MEV protection

## [0.1.0] - 2025-10-01

### Added
- Initial ZK-state compression program
- Privacy account state management
- Merkle root verification on-chain
