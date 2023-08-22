use std::env::set_var;
use ark_std::One;

use super::*;
use halo2_base::halo2_proofs::halo2curves::bn256::{Bn256, Fr, Fq};
use halo2_proofs_axiom::halo2curves::FieldExt;

use crate::moduli_precomuted::{Modulus, fe_to_modulus};
use crate::crt_int::modulus;
use halo2_base::gates::builder::{GateCircuitBuilder, RangeCircuitBuilder, GateThreadBuilder};
use halo2_base::gates::{flex_gate::{FlexGateConfig, GateChip, GateInstructions},
                range::{RangeStrategy, RangeChip, RangeInstructions}};

use halo2_base::halo2_proofs::dev::MockProver;
use std::str::FromStr;


use test_case::test_case;
#[test_case(&BigUint::parse_bytes(b"145", 10).unwrap(), &BigUint::parse_bytes(b"15100000000", 10).unwrap() )]//
#[test_case(&((BigUint::from(145u64) <<128u32) + BigUint::from(170u64)), &((BigUint::from(151u64) <<128u32) + BigUint::from(150u64)) )]//
//#[test_case(&BigUint::parse_bytes(b"145", 10).unwrap(), &BigUint::parse_bytes(b"151", 10).unwrap() )]//
#[test_case(&((modulus::<Fr>() - BigUint::one())/BigUint::from(2u64)), &((modulus::<Fq>() - BigUint::one())/BigUint::from(2u64)); "")]
#[test_case(&(BigUint::from(200u64)), &(BigUint::from(300u64));  "test 200, 300")]

fn test_range(a: &BigUint, p: &BigUint){
    let k = 12;
    let lookup_bits = 10;
    set_var("LOOKUP_BITS", lookup_bits.to_string());
    let mut builder = GateThreadBuilder::mock();
    let ctx = builder.main(0);
    let chip = RangeChip::default(lookup_bits);


    let moduli: Vec::<Fr> = vec![Fr::from(7)];
    let (a, p) = (biguint_into_crtint_fe_modulus::<Fr>(a, &moduli), biguint_into_crtint_fe_modulus::<Fr>(p, &moduli));

    let (a, p) = (ctx.assign_witnesses(a.limbs_as_fe).try_into().unwrap(), ctx.assign_witnesses(p.limbs_as_fe).try_into().unwrap());

    check_big_less_than_p::<Fr>(&chip, ctx, a, p);

    // auto-tune circuit
    builder.config(k, Some(9));
    // create circuit
    let circuit = RangeCircuitBuilder::mock(builder);
    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied()

}

#[should_panic]
#[test_case(&BigUint::parse_bytes(b"145", 10).unwrap(), &BigUint::parse_bytes(b"15100000000", 10).unwrap() )]//
#[test_case(&((BigUint::from(145u64) <<128u32) + BigUint::from(170u64)), &((BigUint::from(151u64) <<128u32) + BigUint::from(150u64)) )]//
//#[test_case(&BigUint::parse_bytes(b"145", 10).unwrap(), &BigUint::parse_bytes(b"151", 10).unwrap() )]//
#[test_case(&((modulus::<Fr>() - BigUint::one())/BigUint::from(2u64)), &((modulus::<Fq>() - BigUint::one())/BigUint::from(2u64)); "")]
#[test_case(&(BigUint::from(200u64)), &(BigUint::from(300u64));  "test 200, 300")]

fn test_range_panicking(p: &BigUint, a: &BigUint){
    let k = 12;
    let lookup_bits = 10;
    set_var("LOOKUP_BITS", lookup_bits.to_string());
    let mut builder = GateThreadBuilder::mock();
    let ctx = builder.main(0);
    let chip = RangeChip::default(lookup_bits);


    let moduli: Vec::<Fr> = vec![Fr::from(7), Fr::from(13), Fr::from(17), Fr::from(19)];
    let (a, p) = (biguint_into_crtint_fe_modulus::<Fr>(a, &moduli), biguint_into_crtint_fe_modulus::<Fr>(p, &moduli));

    let (a, p) = (ctx.assign_witnesses(a.limbs_as_fe).try_into().unwrap(), ctx.assign_witnesses(p.limbs_as_fe).try_into().unwrap());

    check_big_less_than_p::<Fr>(&chip, ctx, a, p);

    // auto-tune circuit
    builder.config(k, Some(9));
    // create circuit
    let circuit = RangeCircuitBuilder::mock(builder);
    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied()

}