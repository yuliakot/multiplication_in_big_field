#![allow(unused_imports)]
#![allow(unused_variables)]
use halo2_base::gates::builder::{GateCircuitBuilder, GateThreadBuilder};
use halo2_base::gates::{flex_gate::{FlexGateConfig, GateChip, GateInstructions},
                range::{RangeStrategy, RangeChip, RangeInstructions}};
use halo2_base::halo2_proofs::{
    arithmetic::Field,
    circuit::*,
    dev::MockProver,
    halo2curves::bn256::{Bn256, Fr, Fq, G1Affine},
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
use num_bigint::BigUint;
use rand::rngs::OsRng;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use ark_std::fs::File;

use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};

use big_field_multiplication::crt_mul;
use big_field_multiplication::crt_int::{CRTint, biguint_into_crtint, fe_into_crtint};
use big_field_multiplication::moduli_precomuted::{Modulus, fe_to_modulus};

use test_case::test_case;


fn read_inputs(i: i32) -> [u64; 2]{
    let path = format!("tests/tests_input{i}.in");
    serde_json::from_reader(
        File::open(path).unwrap_or_else(|e| panic!("")),
    )
    .unwrap()
}

#[test_case(200u64, 300u64)]
#[test_case(444u64, 159u64)]

fn test_more_moduli(a: u64, b: u64){
    let k = 10;
    let mut builder = GateThreadBuilder::new(false);
    let chip = RangeChip::<Fr>::default(10);
    let ctx = builder.main(0);

    let moduli = vec![5u64, 6, 11, 13].iter().map(|x| Fr::from(*x)).collect();

    let p = Fr::from(4289);
    let crt_p = fe_into_crtint(&p, &moduli, &p);
    let p = ctx.load_constant(p);


    let [a, b] = [a, b].map(BigUint::from);

    let res = crt_mul(chip, ctx, &a, &b, &crt_p, &moduli);

    builder.config(k, Some(9));

    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();
}


#[test_case(200u64, 300u64)]
#[test_case(444u64, 159u64)]

fn test_few_moduli(a: u64, b: u64){
    let k = 6;
    let mut builder = GateThreadBuilder::new(false);
    let chip = RangeChip::<Fr>::default(10);
    let ctx = builder.main(0);

    let moduli = vec![7].iter().map(|x| Fr::from(*x)).collect();

    let p = Fr::from(5);
    let crt_p = fe_into_crtint(&p, &moduli, &p);
    let p = ctx.load_constant(p);


    let [a, b] = [a, b].map(BigUint::from);

    let res = crt_mul(chip, ctx, &a, &b, &crt_p, &moduli);

    builder.config(k, Some(9));

    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();
}
