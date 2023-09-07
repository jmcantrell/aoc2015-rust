use itertools::Itertools;

pub type Weight = u64;
pub type QuantumEntanglement = u64;

pub fn ideal_quantum_entanglement(
    weights: &[Weight],
    num_compartments: usize,
) -> Option<QuantumEntanglement> {
    let total_weight: Weight = weights.iter().cloned().sum();
    let target_weight = total_weight / num_compartments as Weight;

    (1..weights.len()).find_map(|len| {
        weights
            .iter()
            .combinations(len)
            .filter_map(|compartment| {
                (compartment.iter().cloned().sum::<Weight>() == target_weight)
                    .then(|| compartment.iter().cloned().product::<Weight>())
            })
            .min()
    })
}
