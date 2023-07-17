use halo2_base::{
    halo2_proofs::halo2curves::bn256::Fr,
    utils::ScalarField,
    gates::{
        builder::GateCircuitBuilder,
        builder::GateThreadBuilder,
        GateChip,
        GateInstructions,
    },
    AssignedValue,
    Context,
    QuantumCell::{self, Constant, Existing, Witness, WitnessFraction},
};
use halo2_proofs_axiom::dev::MockProver;
//use num_bigint::BigInt;

pub mod mod_p_verifications;
pub mod crt_lookup;