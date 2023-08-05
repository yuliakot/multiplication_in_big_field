use halo2_base::{
    halo2_proofs::halo2curves::bn256::Fr,
    utils::ScalarField,
    gates::{
        builder::GateCircuitBuilder,
        builder::GateThreadBuilder,
        GateChip,
        GateInstructions,
        RangeChip, 
        RangeInstructions
    },
    AssignedValue,
    Context,
    QuantumCell::{self, Constant, Existing, Witness},
};
use halo2_proofs_axiom::dev::MockProver;
//use num_bigint::BigInt;

pub mod mod_p_verifications;
pub mod crt_lookup;

#[cfg(test)]
pub mod tests;
