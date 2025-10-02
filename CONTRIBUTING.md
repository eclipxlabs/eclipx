# Contributing to eclipx

Thank you for considering contributing to eclipx.

## Development Setup

1. Install Rust (1.75+) and Solana CLI (1.17+)
2. Install Node.js (18+) and npm
3. Clone the repository:

```bash
git clone https://github.com/eclipxlabs/eclipx.git
cd eclipx
```

4. Build the on-chain programs:

```bash
cargo build
```

5. Install SDK dependencies:

```bash
cd sdk && npm install
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch from `main`
3. Write tests for new functionality
4. Ensure all tests pass locally
5. Submit a pull request with a clear description

## Code Style

- Rust: Follow `rustfmt` configuration in `rustfmt.toml`
- TypeScript: ESLint + Prettier (configured in `sdk/`)
- Commit messages: Use conventional commit format (`feat:`, `fix:`, `docs:`, etc.)

## Reporting Issues

Open an issue with reproduction steps, expected behavior, and environment details.
