#![allow(unused_imports)]
#![allow(unused_variables)]
use halo2_base::gates::builder::{GateCircuitBuilder, GateThreadBuilder};
use halo2_base::gates::flex_gate::{FlexGateConfig, GateChip, GateInstructions, GateStrategy};
use halo2_base::halo2_proofs::{
    arithmetic::Field,
    circuit::*,
    dev::MockProver,
    halo2curves::bn256::{Bn256, Fr, Fq},
    plonk::*,
    poly::kzg::multiopen::VerifierSHPLONK,
    poly::kzg::strategy::SingleStrategy,
    poly::kzg::{
        commitment::{KZGCommitmentScheme, ParamsKZG},
        multiopen::ProverSHPLONK,
    },
    transcript::{Blake2bRead, TranscriptReadBuffer},
    transcript::{Blake2bWrite, Challenge255, TranscriptWriterBuffer},
};
use halo2_base::utils::ScalarField;
use halo2_base::{
    Context,
    QuantumCell::{Existing, Witness},
    SKIP_FIRST_PASS,
};
use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};
use rand::rngs::OsRng;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use ark_std::fs::File;

use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};

use test_case::test_case;

//use super::pow_of_two;
use super::*;

#[test]
fn test_pow_of_two(){
    println!("\n2^128 = {:?}\n", pow_of_two());
}

#[test_case(11u32 => Fr::from(3); "2^128 = 3 mod 11")] // 32 = 2^5 = -1 mod 11; 2^128 = 2^120 * 2^5 * 8 = -8 = 3 mod 11
#[test_case(13u32 => Fr::from(9); "2^128 = 9 mod 13")] // 2^ 128 = 16^32 = 3^32 = 27^30 * 9 = 9


fn test_residue_precomputed<F: ScalarField>(modulus: u32) -> F {
    let modulus: Modulus<F> = biguint_to_modulus(&BigUint::from(modulus));
    modulus.residue_of_a_limb
}
