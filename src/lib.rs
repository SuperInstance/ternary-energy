#![forbid(unsafe_code)]

//! Energy and thermodynamic models for ternary systems.
//!
//! Provides kinetic/potential/total energy computation, energy conservation tracking,
//! entropy production, free energy computation, thermodynamic equilibrium detection,
//! and Carnot-style efficiency bounds for ternary engines.

use std::collections::HashMap;

/// Ternary energy state: maps to {-1, 0, +1} energy quanta.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TernaryEnergy {
    pub kinetic: f64,
    pub potential: f64,
}

impl TernaryEnergy {
    pub fn new(kinetic: f64, potential: f64) -> Self {
        TernaryEnergy { kinetic, potential }
    }

    /// Quantize to ternary levels based on thresholds.
    pub fn ternary_kinetic(&self) -> i8 {
        if self.kinetic < -0.5 { -1 }
        else if self.kinetic > 0.5 { 1 }
        else { 0 }
    }

    pub fn ternary_potential(&self) -> i8 {
        if self.potential < -0.5 { -1 }
        else if self.potential > 0.5 { 1 }
        else { 0 }
    }

    pub fn total(&self) -> f64 {
        self.kinetic + self.potential
    }

    /// Convert to ternary state pair.
    pub fn to_ternary_pair(&self) -> (i8, i8) {
        (self.ternary_kinetic(), self.ternary_potential())
    }
}

/// Tracks energy conservation across transformations.
#[derive(Clone, Debug)]
pub struct EnergyConservation {
    initial_total: f64,
    history: Vec<f64>,
    tolerance: f64,
}

impl EnergyConservation {
    pub fn new(initial: &TernaryEnergy, tolerance: f64) -> Self {
        let total = initial.total();
        EnergyConservation {
            initial_total: total,
            history: vec![total],
            tolerance,
        }
    }

    pub fn record(&mut self, energy: &TernaryEnergy) {
        self.history.push(energy.total());
    }

    /// Check if energy has been conserved (within tolerance).
    pub fn is_conserved(&self) -> bool {
        let latest = self.history.last().unwrap_or(&self.initial_total);
        (latest - self.initial_total).abs() <= self.tolerance
    }

    /// Maximum deviation from initial energy.
    pub fn max_deviation(&self) -> f64 {
        self.history.iter()
            .map(|e| (e - self.initial_total).abs())
            .fold(0.0, f64::max)
    }

    /// Total energy drift over all recorded states.
    pub fn total_drift(&self) -> f64 {
        if self.history.len() < 2 { return 0.0; }
        let mut drift = 0.0;
        for i in 1..self.history.len() {
            drift += (self.history[i] - self.history[i - 1]).abs();
        }
        drift
    }

    pub fn history(&self) -> &[f64] {
        &self.history
    }
}

/// Compute Shannon entropy of a ternary distribution.
pub fn ternary_entropy(counts: &[usize]) -> f64 {
    let total = counts.iter().sum::<usize>() as f64;
    if total == 0.0 { return 0.0; }

    let mut entropy = 0.0;
    for &count in counts {
        if count > 0 {
            let p = count as f64 / total;
            entropy -= p * p.log2();
        }
    }
    entropy
}

/// Compute maximum possible entropy for n ternary states.
pub fn max_ternary_entropy(n_states: usize) -> f64 {
    if n_states == 0 { return 0.0; }
    (n_states as f64).log2()
}

/// Compute entropy production rate from a sequence of ternary states.
pub fn entropy_production(states: &[(i8, i8)]) -> f64 {
    if states.is_empty() { return 0.0; }

    let mut state_counts: HashMap<(i8, i8), usize> = HashMap::new();
    for &state in states {
        *state_counts.entry(state).or_insert(0) += 1;
    }

    let counts: Vec<usize> = state_counts.values().copied().collect();
    let current_entropy = ternary_entropy(&counts);

    // Compare to uniform distribution entropy
    let n_unique = state_counts.len();
    let max_entropy = max_ternary_entropy(n_unique);

    max_entropy - current_entropy
}

/// Compute free energy: F = E - T*S
pub fn free_energy(energy: f64, temperature: f64, entropy: f64) -> f64 {
    energy - temperature * entropy
}

/// Compute Helmholtz free energy for a ternary system.
pub fn helmholtz_free_energy(internal_energy: f64, temperature: f64, states: &[(i8, i8)]) -> f64 {
    let counts: Vec<usize> = {
        let mut map: HashMap<(i8, i8), usize> = HashMap::new();
        for &s in states {
            *map.entry(s).or_insert(0) += 1;
        }
        map.values().copied().collect()
    };
    let s = ternary_entropy(&counts);
    free_energy(internal_energy, temperature, s)
}

/// Check if a system is in thermodynamic equilibrium.
/// Equilibrium: entropy is maximized and energy is evenly distributed.
pub fn is_equilibrium(states: &[(i8, i8)], tolerance: f64) -> bool {
    if states.is_empty() { return true; }

    let mut state_counts: HashMap<(i8, i8), usize> = HashMap::new();
    for &state in states {
        *state_counts.entry(state).or_insert(0) += 1;
    }

    let n_unique = state_counts.len();
    if n_unique == 0 { return true; }
    let expected = states.len() as f64 / n_unique as f64;
    let max_dev = state_counts.values()
        .map(|&c| (c as f64 - expected).abs() / expected)
        .fold(0.0_f64, f64::max);

    max_dev <= tolerance
}

/// A ternary thermodynamic engine operating between hot and cold reservoirs.
#[derive(Clone, Debug)]
pub struct TernaryEngine {
    pub hot_temp: f64,
    pub cold_temp: f64,
    pub work_output: f64,
    pub heat_input: f64,
}

impl TernaryEngine {
    pub fn new(hot_temp: f64, cold_temp: f64) -> Self {
        TernaryEngine {
            hot_temp,
            cold_temp,
            work_output: 0.0,
            heat_input: 0.0,
        }
    }

    /// Carnot efficiency bound for ternary engine.
    pub fn carnot_efficiency(&self) -> f64 {
        if self.hot_temp <= 0.0 || self.cold_temp >= self.hot_temp {
            return 0.0;
        }
        1.0 - self.cold_temp / self.hot_temp
    }

    /// Actual efficiency: work / heat_input.
    pub fn efficiency(&self) -> f64 {
        if self.heat_input <= 0.0 { return 0.0; }
        self.work_output / self.heat_input
    }

    /// Check if engine operates within Carnot bounds.
    pub fn within_carnot_bound(&self) -> bool {
        self.efficiency() <= self.carnot_efficiency() + 1e-10
    }

    /// Perform a cycle: extract heat from hot, reject to cold, produce work.
    /// Work is ternary-quantized.
    pub fn cycle(&mut self, heat_in: f64) -> f64 {
        self.heat_input = heat_in;
        let max_work = heat_in * self.carnot_efficiency();
        // Quantize work to ternary levels
        let ternary_work = if max_work < 0.5 { 0.0 }
                          else if max_work < 1.5 { 1.0 }
                          else { max_work };
        self.work_output = ternary_work.min(max_work);
        self.work_output
    }

    /// Multiple cycles with cumulative tracking.
    pub fn run_cycles(&mut self, heats: &[f64]) -> Vec<f64> {
        let mut outputs = Vec::new();
        let mut total_work = 0.0;
        let mut total_heat = 0.0;
        for &heat in heats {
            total_heat += heat;
            let work = self.cycle(heat);
            total_work += work;
            outputs.push(work);
        }
        self.work_output = total_work;
        self.heat_input = total_heat;
        outputs
    }
}

/// Compute internal energy of a ternary configuration.
/// Each state (-1, 0, +1) contributes its value as energy.
pub fn internal_energy(states: &[i8]) -> f64 {
    states.iter().map(|&s| s as f64).sum()
}

/// Compute the average energy per particle.
pub fn average_energy(states: &[i8]) -> f64 {
    if states.is_empty() { return 0.0; }
    internal_energy(states) / states.len() as f64
}

/// Compute specific heat capacity (derivative of energy w.r.t. temperature).
/// Estimated from energy fluctuations.
pub fn specific_heat(energies: &[f64], temperature: f64) -> f64 {
    if energies.len() < 2 || temperature <= 0.0 { return 0.0; }
    let mean = energies.iter().sum::<f64>() / energies.len() as f64;
    let variance = energies.iter()
        .map(|e| (e - mean).powi(2))
        .sum::<f64>() / energies.len() as f64;
    variance / (temperature * temperature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_energy_creation() {
        let e = TernaryEnergy::new(1.0, -1.0);
        assert_eq!(e.kinetic, 1.0);
        assert_eq!(e.potential, -1.0);
    }

    #[test]
    fn test_ternary_quantization() {
        let e = TernaryEnergy::new(1.5, -2.0);
        assert_eq!(e.ternary_kinetic(), 1);
        assert_eq!(e.ternary_potential(), -1);
    }

    #[test]
    fn test_ternary_quantization_zero() {
        let e = TernaryEnergy::new(0.2, 0.3);
        assert_eq!(e.ternary_kinetic(), 0);
        assert_eq!(e.ternary_potential(), 0);
    }

    #[test]
    fn test_total_energy() {
        let e = TernaryEnergy::new(3.0, -1.0);
        assert!((e.total() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_ternary_pair() {
        let e = TernaryEnergy::new(1.0, -1.0);
        assert_eq!(e.to_ternary_pair(), (1, -1));
    }

    #[test]
    fn test_energy_conservation_conserved() {
        let initial = TernaryEnergy::new(1.0, 1.0);
        let mut tracker = EnergyConservation::new(&initial, 0.01);
        tracker.record(&TernaryEnergy::new(1.5, 0.5));
        tracker.record(&TernaryEnergy::new(0.5, 1.5));
        assert!(tracker.is_conserved());
    }

    #[test]
    fn test_energy_conservation_violated() {
        let initial = TernaryEnergy::new(1.0, 1.0);
        let mut tracker = EnergyConservation::new(&initial, 0.01);
        tracker.record(&TernaryEnergy::new(5.0, 5.0));
        assert!(!tracker.is_conserved());
    }

    #[test]
    fn test_max_deviation() {
        let initial = TernaryEnergy::new(0.0, 0.0);
        let mut tracker = EnergyConservation::new(&initial, 1.0);
        tracker.record(&TernaryEnergy::new(1.0, 0.0));
        tracker.record(&TernaryEnergy::new(-1.0, 0.0));
        assert!((tracker.max_deviation() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_total_drift() {
        let initial = TernaryEnergy::new(0.0, 0.0);
        let mut tracker = EnergyConservation::new(&initial, 1.0);
        tracker.record(&TernaryEnergy::new(1.0, 0.0));
        tracker.record(&TernaryEnergy::new(0.0, 0.0));
        assert!((tracker.total_drift() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_ternary_entropy_uniform() {
        let counts = vec![10, 10, 10];
        let entropy = ternary_entropy(&counts);
        let max = max_ternary_entropy(3);
        assert!((entropy - max).abs() < 1e-10);
    }

    #[test]
    fn test_ternary_entropy_concentrated() {
        let counts = vec![30, 0, 0];
        let entropy = ternary_entropy(&counts);
        assert!((entropy - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_entropy_production_uniform() {
        let states = vec![(0i8, 0i8), (1, 1), (-1, -1)];
        let prod = entropy_production(&states);
        assert!(prod >= 0.0);
    }

    #[test]
    fn test_free_energy() {
        let f = free_energy(10.0, 300.0, 0.5);
        assert!((f - (10.0 - 150.0)).abs() < 1e-10);
    }

    #[test]
    fn test_helmholtz_free_energy() {
        let states = vec![(1, 0), (1, 0), (-1, 0)];
        let f = helmholtz_free_energy(5.0, 1.0, &states);
        assert!(f.is_finite());
    }

    #[test]
    fn test_is_equilibrium_uniform() {
        let states = vec![(1i8, 0i8), (-1, 0), (0, 1), (1, 0), (-1, 0), (0, 1)];
        assert!(is_equilibrium(&states, 0.5));
    }

    #[test]
    fn test_is_equilibrium_not() {
        // 90% in one state, 10% in another -> not balanced
        let mut states = vec![(1i8, 0i8); 90];
        states.push((-1, 0));
        states.push((-1, 0));
        // With tolerance 0.1 (10%), the deviation is huge
        assert!(!is_equilibrium(&states, 0.1));
    }

    #[test]
    fn test_carnot_efficiency() {
        let engine = TernaryEngine::new(600.0, 300.0);
        assert!((engine.carnot_efficiency() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_engine_cycle() {
        let mut engine = TernaryEngine::new(600.0, 300.0);
        let work = engine.cycle(100.0);
        assert!(work > 0.0);
        assert!(engine.within_carnot_bound());
    }

    #[test]
    fn test_engine_multiple_cycles() {
        let mut engine = TernaryEngine::new(800.0, 200.0);
        let outputs = engine.run_cycles(&[100.0, 200.0, 150.0]);
        assert_eq!(outputs.len(), 3);
        assert!(engine.efficiency() > 0.0);
    }

    #[test]
    fn test_engine_zero_temp() {
        let engine = TernaryEngine::new(0.0, 0.0);
        assert_eq!(engine.carnot_efficiency(), 0.0);
    }

    #[test]
    fn test_internal_energy() {
        let states = vec![1i8, -1, 0, 1];
        assert!((internal_energy(&states) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_average_energy() {
        let states = vec![1i8, -1, 1];
        assert!((average_energy(&states) - (1.0 / 3.0)).abs() < 1e-10);
    }

    #[test]
    fn test_specific_heat() {
        let energies = vec![1.0, 2.0, 1.5, 1.5];
        let cv = specific_heat(&energies, 1.0);
        assert!(cv > 0.0);
    }
}
