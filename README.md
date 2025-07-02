# Solana Smart Contract: Message Storage

Welcome! 👋

This project is a simple Solana smart contract built with [Anchor](https://book.anchor-lang.com/) that lets you store and update a message on-chain. It's a great starting point for learning how to build, test, and deploy programs on Solana.

## What does it do?
- **Initialize**: Store a custom message in a Solana account.
- **Update**: Change the stored message later.
- The contract ensures you can't store an empty message (it will throw an error!).

## Technologies & Tools Used
- **Rust**: For the smart contract logic.
- **Anchor**: Framework for Solana smart contract development.
- **TypeScript**: For writing tests and scripts.
- **Mocha & Chai**: For testing.
- **Prettier**: For code formatting.
- **Yarn**: As the package manager.

## Project Structure
- `programs/solana-smart-contract/`: The main smart contract code (Rust).
- `tests/`: TypeScript tests for the contract.
- `migrations/`: Deployment scripts.
- `Anchor.toml`, `Cargo.toml`, `package.json`: Project configs.

## How to Run
1. **Install dependencies**: `yarn install`
2. **Build the contract**: `anchor build`
3. **Test it**: `anchor test`

Make sure you have [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) and [Anchor](https://book.anchor-lang.com/getting_started/installation.html) installed.

## What I Learned
- How to structure a Solana program with Anchor.
- How to use PDAs (Program Derived Addresses) for account management.
- Writing and running TypeScript tests for Solana contracts.
- The importance of input validation (no empty messages!).
- The basics of deploying and interacting with Solana programs locally.

---

Feel free to fork, play around, and build something cool on Solana! 🚀 