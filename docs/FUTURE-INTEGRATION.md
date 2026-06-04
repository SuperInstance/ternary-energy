# Future Integration: ternary-energy

## Current State
Provides kinetic/potential/total energy computation, energy conservation tracking, entropy production, free energy computation, and Carnot-style efficiency bounds for ternary energy systems.

## Integration Opportunities

### With ternary-cell (Conservation Laws)
Cell energy during the `vibe` phase must obey conservation. `EnergyConservation` tracks total energy across the grid and raises violations when tolerance is exceeded. This is the physical law layer — the `gc` phase cannot destroy more energy than the `acquire` phase created. `ternary-energy` provides the invariant checker.

### With construct-core
Construct skill loading/unloading has an energy cost. A `SyncConstruct` executing `query_lookup` costs less energy than an `AsyncConstruct` running `query_owned`. The efficiency bounds (`CarnotEfficiency`) provide theoretical limits on how efficiently a construct can process — any implementation exceeding Carnot efficiency has a bug.

### With ternary-thermodynamics
Natural pairing: `ternary-energy` tracks quantities, `ternary-thermodynamics` tracks transformations. Together they form the complete physics of ternary systems — energy in, entropy out, free energy available for computation.

## Potential in Mature Systems
In room-as-codespace, each room has an energy budget. A Codespace spinning up costs kinetic energy (compute). Running ensigns costs potential energy (memory). The total budget is finite — `ternary-energy` enforces that rooms cannot exceed their allocation. Carnot efficiency tells us the maximum useful compute we can extract from a given energy input.

## Cross-Pollination Ideas
- Free energy as a room scheduling metric: schedule the room with the highest available free energy
- Entropy production as a code quality metric — messy rooms produce more entropy
- Conservation violations as security alerts: if energy appears from nowhere, something is compromised

## Dependencies for Next Steps
- ternary-cell needs energy accounting in the tick cycle
- construct-core needs energy cost annotations on skill operations
- Conservation-matrix integration for multi-room energy balance
