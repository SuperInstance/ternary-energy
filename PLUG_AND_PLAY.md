# PLUG_AND_PLAY — Energy

> Energy and thermodynamic models for ternary systems

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-energy = { git = "https://github.com/SuperInstance/ternary-energy" }
```

Use in your code:

```rust
use ternary_energy::{TernaryEngine, ThermodynamicState};

let mut engine = TernaryEngine::new();
engine.add_well(|x| x * x);
let energy = engine.compute(&[1, 0, -1]);
```

## 📚 Available Documentation

| Document | Description |
|----------|-------------|
| `docs/FROM_BINARY.md` | Understanding ternary concepts as a binary programmer |
| `docs/MIGRATION.md` | Version migration guide |
| `docs/FUTURE-INTEGRATION.md` | Planned features and roadmap |

## 🔗 Integration

This crate is part of the [SuperInstance ternary fleet](https://github.com/SuperInstance). It uses the canonical `Ternary` type from `ternary-types` for cross-crate compatibility.

## 📄 License

MIT
