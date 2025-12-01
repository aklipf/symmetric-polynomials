# Symmetric polynomial package

## Installation of rust on linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Invariant generation

Generate all the invariants of degree `DEGREE`.

```bash
cargo run --release --bin invariant -- -d <DEGREE>
```

You can put an arbitrary limit on the size of the model with the argument `-n <DOMAIN>`. If no domain is specified, all the invariants are generated (for any domain size).

