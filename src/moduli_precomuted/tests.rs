#![allow(unused_imports)]
#![allow(unused_variables)]
use halo2_base::gates::builder::{GateCircuitBuilder, GateThreadBuilder};
use halo2_base::gates::flex_gate::{FlexGateConfig, GateChip, GateInstructions, GateStrategy};
use halo2_base::halo2_proofs::{
    arithmetic::Field,
    circuit::*,
    dev::MockProver,
    halo2curves::bn256::{Bn256, Fr, G1Affine},
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

use super::pow_of_two;

#[test]
fn test_pow_of_two(){
    println!("\n2^128 = {:?}\n", pow_of_two());
}
