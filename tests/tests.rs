#![allow(unused_imports)]
#![allow(unused_variables)]
use halo2_base::gates::builder::{GateCircuitBuilder, GateThreadBuilder, RangeCircuitBuilder};
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
use halo2_base::utils::{ScalarField, biguint_to_fe};
use halo2_base::{
    Context,
    QuantumCell::{Existing, Witness, Constant},
    SKIP_FIRST_PASS,
};
use itertools::Itertools;
use num_bigint::BigUint;
use rand::rngs::OsRng;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use ark_std::fs::File;
use ark_std::env::set_var;

use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};

use big_field_multiplication::crt_mul;
use big_field_multiplication::crt_int::{CRTint, biguint_into_crtint_bui_modulus, fe_into_crtint, fe_into_crtint_bui_modulus, modulus};
use big_field_multiplication::crt_int::modulus as find_field_modulus;
use big_field_multiplication::multiplication_gates::crt_to_bits_proof::{pow_of_two, BITStoCRT};

use ark_std::One;

use halo2_proofs_axiom::halo2curves::FieldExt;


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
    let k = 12;
    let lookup_bits = 10;
    let mut builder = GateThreadBuilder::new(false);
    let chip = RangeChip::<Fr>::default(lookup_bits);
    set_var("LOOKUP_BITS", lookup_bits.to_string());
    let ctx = builder.main(0);

    let moduli = &vec![5u64, 6, 11, 13].iter().map(|x| BigUint::from(*x)).collect();

    let p = Fr::from(4289);
    let crt_p = fe_into_crtint_bui_modulus(&p, moduli);
    let p = ctx.load_constant(p);


    let [a, b] = [a, b].map(BigUint::from);

    let res = crt_mul(&chip, ctx, &a, &b, &crt_p, moduli);

    builder.config(k, Some(9));

    let circuit = RangeCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

    println!("{:?}, {:?}", res[0].value(), res[1].value())
}


#[test_case(200u64, 300u64)]
#[test_case(444u64, 159u64)]

fn test_few_moduli(a: u64, b: u64){
    let k = 12;
    let lookup_bits = 10;
    let mut builder = GateThreadBuilder::new(false);
    let chip = RangeChip::<Fr>::default(lookup_bits);
//    set_var("LOOKUP_BITS", lookup_bits.to_string());
    let ctx = builder.main(0);

    let moduli: Vec<BigUint> = vec![7u64].iter().map(|x| BigUint::from(*x)).collect();

    let p = Fr::from(4289);
    let crt_p = fe_into_crtint_bui_modulus(&p, &moduli);

    let [a, b] = [a, b].map(BigUint::from);

    let res = crt_mul::<Fr>(&chip, ctx, &a, &b, &crt_p, &moduli);

    builder.config(k, Some(9));

    let circuit = RangeCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

    println!("{:?}, {:?}", res[0].value(), res[1].value())
}


fn limbs_to_biguint(limbs: [u128; 2])-> BigUint
{
    BigUint::from(limbs[0]) + BigUint::from(limbs[1])*pow_of_two()
}


#[test_case([200u128, 300u128], [444u128, 159u128])]
#[test_case([1u128 << 125, 300u128], [1u128 << 125, 159u128])]
#[test_case([1u128 << 125, 125u128 << 118], [159u128 << 120, 1u128 << 125])]
fn test_big_numbers(limbs_a: [u128; 2], limbs_b: [u128; 2]){
    let k = 12;
    let lookup_bits = 10;
    let mut builder = GateThreadBuilder::new(false);
    let chip = RangeChip::<Fr>::default(lookup_bits);
    set_var("LOOKUP_BITS", lookup_bits.to_string());
    let ctx = builder.main(0);

    let moduli = &vec![5u64, 6, 11, 13].iter().map(|x| BigUint::from(*x)).collect();

    let p = find_field_modulus::<Fq>();
    let crt_p = biguint_into_crtint_bui_modulus(&p, moduli);


    let [a, b] = [limbs_a, limbs_b].map(limbs_to_biguint);

    let res = crt_mul(&chip, ctx, &a, &b, &crt_p, moduli);
    
    builder.config(k, Some(9));

    let circuit = RangeCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

}

