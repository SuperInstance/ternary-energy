# ternary-energy

Energy and thermodynamic models for ternary systems — conservation tracking, entropy production, free energy computation, equilibrium detection, and Carnot-style efficiency bounds for ternary engines.

## Why This Exists

Every physical system obeys thermodynamic laws: energy is conserved, entropy tends to increase, and no engine can exceed Carnot efficiency. These laws apply equally to computational systems modeled in ternary {-1, 0, +1}. This crate provides the thermodynamic framework for reasoning about energy in ternary systems: tracking conservation across transformations, measuring entropy production, computing free energy, detecting equilibrium, and bounding efficiency.

The ternary state space {-1, 0, +1} maps naturally to energy levels: a particle at +1 carries positive energy, -1 carries negative energy, and 0 is the ground state. This makes it possible to model thermodynamic processes — heating (biasing toward +1), cooling (biasing toward -1), and equilibrium (uniform distribution) — as operations on ternary ensembles. The `TernaryEngine` models heat engines operating between ternary reservoirs with quantized work output.

This crate is part of the **Negative Space Intelligence** ecosystem.

## Core Concepts

- **TernaryEnergy** — Energy state with kinetic and potential components. Quantizes continuous values to ternary levels {-1, 0, +1} based on thresholds.
- **EnergyConservation** — Tracker that records energy at each step and verifies conservation within a configurable tolerance. Reports maximum deviation and cumulative drift.
- **Entropy Functions** — Shannon entropy of ternary distributions, maximum entropy, and entropy production rate from state sequences.
- **Free Energy** — Helmholtz free energy F = E − TS for ternary systems.
- **Equilibrium Detection** — Checks whether a ternary ensemble is in thermodynamic equilibrium (states uniformly distributed within tolerance).
- **TernaryEngine** — A Carnot-style heat engine with ternary-quantized work output. Operates between hot and cold reservoirs, computing efficiency bounds.
- **Specific Heat** — Estimated from energy fluctuations via the fluctuation-dissipation theorem.

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-energy = "0.1"
```

```rust
use ternary_energy::*;

// Ternary energy state
let e = TernaryEnergy::new(1.5, -0.3);
assert_eq!(e.ternary_kinetic(), 1);   // > 0.5 → +1
assert_eq!(e.ternary_potential(), 0);  // between -0.5 and 0.5 → 0
assert_eq!(e.to_ternary_pair(), (1, 0));

// Track energy conservation
let initial = TernaryEnergy::new(1.0, 1.0);
let mut tracker = EnergyConservation::new(&initial, 0.01);
tracker.record(&TernaryEnergy::new(1.5, 0.5));  // total = 2.0 ✓
tracker.record(&TernaryEnergy::new(0.5, 1.5));  // total = 2.0 ✓
assert!(tracker.is_conserved());
println!("Max deviation: {:.4}", tracker.max_deviation());

// Entropy of a ternary distribution
let counts = vec![10, 10, 10]; // uniform
let entropy = ternary_entropy(&counts);
let max_entropy = max_ternary_entropy(3);
assert!((entropy - max_entropy).abs() < 1e-10);

// Entropy production from state sequence
let states = vec![(1i8, 0i8), (-1, 0), (0, 1), (1, 0), (-1, 0), (0, 1)];
let production = entropy_production(&states);

// Free energy
let f = free_energy(10.0, 300.0, 0.5);
assert!((f - (10.0 - 150.0)).abs() < 1e-10);

// Equilibrium check
assert!(is_equilibrium(&states, 0.5));

// Ternary heat engine
let mut engine = TernaryEngine::new(600.0, 300.0);
println!("Carnot efficiency: {:.1}%", engine.carnot_efficiency() * 100.0); // 50%

let work = engine.cycle(100.0);
assert!(engine.within_carnot_bound());

// Multiple cycles
let outputs = engine.run_cycles(&[100.0, 200.0, 150.0]);
println!("Engine efficiency: {:.1}%", engine.efficiency() * 100.0);

// Specific heat from energy fluctuations
let energies = vec![1.0, 2.0, 1.5, 1.5, 2.0, 1.0];
let cv = specific_heat(&energies, 1.0);
```

## API Overview

### TernaryEnergy
| Method | Description |
|---|---|
| `new(kinetic, potential)` | Create energy state |
| `ternary_kinetic()` / `ternary_potential()` | Quantize to {-1, 0, +1} |
| `total()` | Kinetic + Potential |
| `to_ternary_pair()` | (ternary_kinetic, ternary_potential) |

### EnergyConservation
| Method | Description |
|---|---|
| `new(initial, tolerance)` | Start tracking |
| `record(energy)` | Log an energy state |
| `is_conserved()` | Check within tolerance |
| `max_deviation()` | Worst-case drift from initial |
| `total_drift()` | Cumulative step-to-step drift |

### Thermodynamic Functions
| Function | Description |
|---|---|
| `ternary_entropy(counts)` | Shannon entropy of distribution |
| `max_ternary_entropy(n)` | Maximum possible entropy |
| `entropy_production(states)` | Entropy gap from uniform |
| `free_energy(E, T, S)` | F = E − TS |
| `helmholtz_free_energy(E, T, states)` | F from state ensemble |
| `is_equilibrium(states, tolerance)` | Uniform distribution check |
| `internal_energy(states)` | Sum of ternary values |
| `average_energy(states)` | Per-particle energy |
| `specific_heat(energies, T)` | Fluctuation-based estimate |

### TernaryEngine
| Method | Description |
|---|---|
| `new(hot_temp, cold_temp)` | Create engine |
| `carnot_efficiency()` | Theoretical maximum |
| `efficiency()` | Actual work/heat ratio |
| `within_carnot_bound()` | Physics compliance check |
| `cycle(heat_in)` | Single cycle with ternary-quantized work |
| `run_cycles(heats)` | Multiple cycles with cumulative tracking |

## How It Works

Energy quantization maps continuous values to ternary levels using thresholds at ±0.5. Values below -0.5 become -1, above 0.5 become +1, and everything in between becomes 0. This is analogous to ternary analog-to-digital conversion, providing discrete energy levels while preserving the sign information that binary quantization would lose.

The conservation tracker records total energy (kinetic + potential) at each step and checks whether the latest value remains within tolerance of the initial total. This catches both gradual drift (small cumulative errors) and sudden violations (large single-step changes). The `total_drift` metric sums absolute step-to-step changes, revealing noisy but bounded transformations versus smooth conservation.

The TernaryEngine models a Carnot-style heat engine with work quantization. The Carnot efficiency η = 1 − T_cold/T_hot provides the thermodynamic upper bound. Work output is ternary-quantized: below 0.5 → 0 (no work), 0.5–1.5 → 1.0 (unit work), above 1.5 → full value. This reflects the discrete nature of ternary energy while still respecting thermodynamic limits — `within_carnot_bound()` verifies that actual efficiency never exceeds theoretical maximum.

Entropy production measures how far a state distribution is from maximum entropy (uniform distribution). A system at equilibrium has zero entropy production, while ordered systems have positive production — the gap that the second law of thermodynamics says must tend to increase.

## Use Cases

1. **Ternary system simulation** — Model physical or computational systems where energy conservation must be verified. The tracker provides both verification and diagnostics.

2. **Thermodynamic analysis of ternary algorithms** — Compute the energy cost of ternary computations. `internal_energy` and `entropy_production` quantify the thermodynamic footprint of ternary state transformations.

3. **Ternary engine design** — Explore the efficiency limits of hypothetical ternary computing hardware. The `TernaryEngine` with its quantized work output models discrete energy extraction from ternary processes.

4. **Equilibrium and phase transitions** — Track ternary ensembles as they evolve. `is_equilibrium` detects steady states; entropy production identifies when a system is far from equilibrium and actively evolving.

## Ecosystem

| Crate | Relationship |
|---|---|
| `ternary-cell` | Cell energy dynamics use these thermodynamic primitives |
| `ternary-hardware` | Hardware energy consumption modeling |
| `ternary-econ` | Economic "energy" (capital) follows similar conservation laws |
| `ternary-network` | Network flow can be analyzed as energy transfer |
| `ternary-quantum` | Quantum systems have their own energy level structure |

## Known Limitations

- **Physics terminology is analogical, not rigorous.** Terms like "Carnot efficiency," "specific heat," "Helmholtz free energy," and "entropy production" describe simplified computations on ternary distributions, not physically rigorous thermodynamic quantities.
- **`entropy_production` computes entropy deficit** (distance from uniform), not a production rate in the thermodynamic sense (dS/dt).
- **`is_equilibrium` checks distribution uniformity**, not thermodynamic equilibrium in any physical sense.
- **Energy quantization thresholds (±0.5) are hardcoded** and not configurable.
- **`TernaryEngine::cycle` has inconsistent quantization.** For large heat inputs the quantization effectively does nothing (work = max_work), while for small inputs it collapses to 0 or 1.0.

## See Also

- **ternary-thermodynamics** — Statistical mechanics analogs for ternary systems
- **ternary-entropy** — Entropy and information theory for ternary distributions
- **ternary-irradiate** — Radiation and energy propagation models
- **ternary-fire** — Fire spread and combustion modeling with ternary states
- **ternary-ising** — Ising model simulations with ternary spin states
- **ternary-chaos** — Chaos and nonlinear dynamics for ternary systems

## License

MIT
