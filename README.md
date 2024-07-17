# Alloy CLI Demo

build by:

- [alloy](https://crates.io/crates/alloy)
- [clap](https://crates.io/crates/clap)

## Features

Available commands:

- `block`: get block number
- `balance`: get balance of address
- `transfer`: transfer 100 wei ETH from sender to receiver

## How to start ?

### Global options

- `--mainnet` Set network to mainnet [default: sepolia]

### Commands

- Check all commands

```bash
cargo run -- --help
```

- Get block number (mainnet)

```bash
cargo run -- --mainnet block
```

- Get balance

```bash
cargo run -- balance <YOUR ADDRESS e.g. 0x056703bb4E0866909E1767D9b079237D1C44962f>
```

- Transfer 100 wei ETH

```bash
cargo run -- transfer <SENDER PRIVATEKEY> <RECEIVER ADDRESS>
```
