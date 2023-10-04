#![allow(unused_imports)]
#![allow(unused_variables)]
use halo2_base::gates::builder::{GateThreadBuilder, RangeCircuitBuilder};
use halo2_base::gates::range::RangeChip;

use halo2_base::halo2_proofs::{dev::MockProver,
    halo2curves::bn256::{Fr, Fq},
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
use halo2_base::utils::biguint_to_fe;
use num_bigint::BigUint;
use num_integer::Integer;

use ark_std::env::set_var;

use big_field_multiplication::crt_mul;
use big_field_multiplication::crt_int::biguint_into_crtint_bui_modulus;
use big_field_multiplication::crt_int::modulus as find_field_modulus;
use big_field_multiplication::multiplication_gates::crt_to_bits_proof::pow_of_two;

use ark_std::{Zero, One};

use halo2_proofs_axiom::halo2curves::FieldExt;


use rand::Rng;


fn limbs_to_biguint(limbs: [u128; 2])-> BigUint
{
    BigUint::from(limbs[0]) + BigUint::from(limbs[1])*pow_of_two()
}

fn to_limbs(p: &BigUint) -> [u128; 2]{
    let mut e = p.iter_u64_digits();

    let mut limb0 = e.next().unwrap_or(0) as u128;
    let limb1 = e.next().unwrap_or(0) as u128;    
    limb0 |= ((limb1 & ((1u128 << 64) - 1u128)) as u128) << 64u32;

    let mut limb2 = e.next().unwrap_or(0) as u128;
    let limb3 = e.next().unwrap_or(0) as u128;
    limb2 |= ((limb3 & ((1u128 << 64) - 1u128)) as u128) << 64u32;
    [limb0, limb2]
}


fn random_moduli() -> Vec<u128>{
    let mut ans = vec![];

    for _ in 0..19{
        let mut rng = rand::thread_rng();
        let curr: u128 = rng.gen_range(2..200);
        ans.push(curr);
    }
    ans
}


fn main(){
    let k = 11;
    let lookup_bits = 10;
    let mut builder = GateThreadBuilder::new(false);
    let chip = RangeChip::<Fr>::default(lookup_bits);
    set_var("LOOKUP_BITS", lookup_bits.to_string());
    let ctx = builder.main(0);

    let moduli = &random_moduli().iter().map(|x| BigUint::from(*x)).collect();

    let p = find_field_modulus::<Fq>();
    let crt_p = biguint_into_crtint_bui_modulus(&p, moduli);

    let mut rng = rand::thread_rng();
    let big_limb_p = to_limbs(&p)[1];


    let [[a0, a1],[b0, b1]]: [[u128; 2]; 2] = [[rand::random::<u128>(), rng.gen_range(0..big_limb_p+1)],[rand::random::<u128>(), rng.gen_range(0..big_limb_p+1)] ];
    let [limbs_a, limbs_b]: [[u128; 2]; 2] = [[a0, a1], [b0, b1]];
    let [a, b] = [limbs_a, limbs_b].map(|x| limbs_to_biguint(x));

    
    println!("{:?}, {:?}", a, b);
    assert!(&a < &p);
    assert!(&b < &p);

    let res = crt_mul(&chip, ctx, &a, &b, &crt_p, moduli);
    let true_res: [Fr; 2] = to_limbs(&(a*b).div_rem(&p).1).map(|x| biguint_to_fe(&BigUint::from(x)));

    assert!(res[0].value() == &true_res[0]);
    assert!(res[1].value() == &true_res[1]);
    
    builder.config(k, Some(9));

    let circuit = RangeCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

}