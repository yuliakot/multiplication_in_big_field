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
use rand::rngs::OsRng;
use std::marker::PhantomData;

use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};

use pprof::criterion::{Output, PProfProfiler};
// Thanks to the example provided by @jebbow in his article
// https://www.jibbow.com/posts/criterion-flamegraphs/

use big_field_multiplication::multiplication_gates::mod_p_verifications::mod_r_mul;
use big_field_multiplication::crt_mul;




use test_case::test_case;

#[test_case([1, 2, 3, 4, 5].map(Fr::from) => Fr::from(0))]//, b"1*2 != 3*4 + 5 mod p")]
#[test_case([9, 6, 4, 10, 5].map(Fr::from) => Fr::from(1))]//, b"4*10 == 9*6 + 5")]

fn test_crt_mod_p_mul<F: ScalarField>(inputs: [F; 5]) -> F{
    let k = 6;
    let mut builder = GateThreadBuilder::new(false);
    let mut chip = GateChip::default();
    
    let res = mod_r_mul(&chip, builder.main(0), &inputs);

    // auto-tune circuit
    builder.config(k, Some(9));
    // create circuit
    
    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

    *res.value()
}


#[test_case([1, 2, 3, 4, 5].map(Fr::from))]//, b"1*2 != 3*4 + 5 mod p")]
#[test_case([9, 6, 4, 10, 5].map(Fr::from))]//, b"4*10 == 9*6 + 5")]

fn test_crt_mul<F: ScalarField>(inputs: [F; 5]){
    
    let moduli = [10, 17, 19, 21];
    let moduli = Vec::from(moduli.map(F::from));
    let k = 6;
    let mut builder = GateThreadBuilder::new(false);
    let mut chip = GateChip::default();
    
    crt_mul(&chip, builder.main(0), inputs, &moduli);

    // auto-tune circuit
    builder.config(k, Some(9));
    // create circuit
    
    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();
}
