# Accelerated Turbin3 Rust Solana

A collection of Solana programs written in Rust as part of the Turbin3 Accelerated Builders cohort. Each top-level folder is an independent project (mostly Anchor-based), covering escrow, delegated/ephemeral state, and Token-2022 transfer hooks with whitelisting.

This is a monorepo: there is no single build or entry point. Build and test each project from within its own folder.

## Projects

### `escrow-litesvm`

An Anchor escrow program (`anchor_escrow`). A maker deposits token A into a vault and specifies how much of token B they want in return; a taker can then fulfill the trade by depositing token B and receiving token A, or the maker can refund and close the vault before it's taken.

- Instructions: `make`, `take`, `refund`
- State: `Escrow` (seed, maker, mint_a, mint_b, receive amount, bump)
- Named for testing with [LiteSVM](https://github.com/LiteSVM/litesvm), a lightweight Solana runtime for fast unit tests

### `magicblock-er-example`

An Anchor program (`er_state_account`) demonstrating [MagicBlock](https://www.magicblock.gg/) Ephemeral Rollups: delegating a program account to an ephemeral rollup validator for fast, cheap updates, then committing the state back to and undelegating from the base Solana chain.

- Instructions: `initialize`, `update`, `update_commit`, `delegate`, `undelegate`, `close`
- State: `UserAccount` (owner, a `u64` data field, bump)
- Uses the `ephemeral-rollups-sdk` crate and the `#[ephemeral]` / `#[delegate]` / `#[commit]` Anchor macros

### `pinocchio`

Placeholder folder (contains only a `.gitkeep`), reserved for a future program built with the [Pinocchio](https://github.com/anza-xyz/pinocchio) framework, a `no_std` alternative to Anchor for writing lean native Solana programs.

### `week-1-test`

The Week 1 challenge: build a **Transfer Hook Vault** where only whitelisted users can deposit or withdraw a program-minted token, using a `Vec` for the initial whitelist implementation and requiring LiteSVM tests plus one additional Token-2022 extension. See `week-1-test/readme.md` for the original task description. It contains two sub-projects:

- **`transfer_hook`**: an initial, simpler Token-2022 transfer hook implementation.
- **`week1_challenge`**: the fuller solution, an Anchor program (`week1_challenge`) with:
  - Instructions: `create_vault`, `mint_token`, `add_to_whitelist`, `remove_from_whitelist`, `deposit`, `withdraw`
  - State: `Vault` (mint, owner, bump) and `Whitelist` (a `Vec<(Pubkey, u64, bool)>` of address, allowance, and whitelisted flag, plus an admin field)
  - Enforcement of whitelist rules happens in a transfer hook invoked automatically on every token transfer

### `whitelist-transfer-hook`

A polished, documented example of the same core idea: a Token-2022 transfer hook (`whitelist_transfer_hook`) that restricts token transfers to addresses on an on-chain whitelist.

- Instructions: initialize the whitelist, add/remove addresses (with dynamic account reallocation and rent adjustment), initialize the extra-account-metas list required by the Token-2022 transfer hook interface, mint tokens, and the transfer hook itself
- State: `Whitelist` (a dynamic `Vec<Pubkey>` of authorized addresses, plus a bump)
- This folder has its own detailed `README.md` walking through the account structures and instruction logic in depth — see [`whitelist-transfer-hook/README.md`](./whitelist-transfer-hook/README.md).

## Tech Stack

- Rust
- Anchor framework
- Solana Token-2022 program (transfer hook extension)
- MagicBlock Ephemeral Rollups SDK (`magicblock-er-example` only)
- TypeScript (Mocha/Chai via `ts-mocha`) for on-chain program tests
- Package managers vary by project: yarn (`escrow-litesvm`, `magicblock-er-example`), pnpm (`week-1-test/transfer_hook`, `week-1-test/week1_challenge`, `whitelist-transfer-hook`)

## Prerequisites

- Rust and Cargo
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- Node.js, with yarn and/or pnpm depending on the project

## Running a Project

Each Anchor project is self-contained. From inside a given project folder (for example `whitelist-transfer-hook/`):

```
# install JS dependencies (check the project for yarn vs pnpm)
yarn install   # or: pnpm install

# build the program
anchor build

# run the test suite defined in Anchor.toml
anchor test
```

`escrow-litesvm` additionally exposes a native Cargo test command (see its `Anchor.toml`):

```
cargo test -- --nocapture
```

## Notes

- Each project has its own `Anchor.toml`, `Cargo.toml`/`Cargo.lock`, and program ID; there is no shared workspace across the top-level folders.
- `whitelist-transfer-hook` is configured for the `devnet` cluster; `escrow-litesvm` is configured for `Localnet`; `magicblock-er-example` is configured for `devnet` as well but is intended to interact with a MagicBlock ephemeral rollup validator.
- The `pinocchio` folder is currently empty and reserved for future work.
