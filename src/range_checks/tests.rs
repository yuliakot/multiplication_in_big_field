use std::env::set_var;
use ark_std::One;

use super::*;
use halo2_base::halo2_proofs::halo2curves::bn256::{Bn256, Fr, Fq};
use halo2_proofs_axiom::halo2curves::FieldExt;

use crate::moduli_precomuted::{Modulus, fe_to_modulus};
use halo2_base::gates::builder::{GateCircuitBuilder, RangeCircuitBuilder, GateThreadBuilder};
use halo2_base::gates::{flex_gate::{FlexGateConfig, GateChip, GateInstructions},
                range::{RangeStrategy, RangeChip, RangeInstructions}};

use halo2_base::halo2_proofs::dev::MockProver;
use std::str::FromStr;


//const p_num: &str = Fr::MODULUS;
//const n_num: &str = Fq::MODULUS;

use test_case::test_case;
#[test_case(&BigUint::parse_bytes(b"152", 10).unwrap(), &BigUint::parse_bytes(b"151", 10).unwrap() )]//
//#[test_case( &(BigUint::from_bytes_be(Fq::MODULUS.as_bytes()) - BigUint::one()), &(BigUint::from_bytes_be(Fr::MODULUS.as_bytes()) - BigUint::one()); "")]

fn test_range(a: &BigUint, p: &BigUint){
    //let mut builder = GateThreadBuilder::new(false);
    //let ctx = builder.main(0);


    let k = 12;
    let lookup_bits = 10;
    set_var("LOOKUP_BITS", lookup_bits.to_string());
    let mut builder = GateThreadBuilder::mock();
    let ctx = builder.main(0);
    let chip = RangeChip::default(lookup_bits);


    let moduli: Vec::<Fr> = vec![];
    let (a, p) = (biguint_into_crtint::<Fr>(a, &moduli, p), biguint_into_crtint::<Fr>(p, &moduli, p));

    let (a, p) = (ctx.assign_witnesses(a.limbs_as_fe).try_into().unwrap(), ctx.assign_witnesses(p.limbs_as_fe).try_into().unwrap());

    check_big_less_than_p::<Fr>(&chip, ctx, a, p);

    // auto-tune circuit
    builder.config(k, Some(9));
    // create circuit
    let circuit = RangeCircuitBuilder::mock(builder);
    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied()

}
