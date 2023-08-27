
use super::*;
use halo2_base::halo2_proofs::halo2curves::bn256::{Bn256, Fr, Fq};

//use crate::moduli_precomuted::{Modulus, fe_to_modulus};
use halo2_base::gates::builder::GateThreadBuilder;

use test_case::test_case;
#[test_case(&BigUint::parse_bytes(b"150", 10).unwrap() => [150, 0].map(Fq::from))]//
#[test_case(&BigUint::parse_bytes(b"200_000_000_000_000_000_000_000_000_000_000", 16).unwrap()
             => [0, 2].map(Fq::from); "Check that 2 times 16 to pow 32 == [0, 2] in limbs")]//

fn test_limbs(a: &BigUint) -> [Fq; 2]{
    //let mut builder = GateThreadBuilder::new(false);
    //let ctx = builder.main(0);

    let moduli: Vec<u64> = vec![3, 4, 5, 7];
    let moduli = &moduli.iter().map(|x|  Fq::from(*x))
                                            .collect::<Vec::<Fq>>();
    let p = BigUint::from(11u32);
    let crt_integer = biguint_into_crtint_fe_modulus(a, moduli);
    crt_integer.limbs_as_fe

}

#[test_case([200, 301].map(&Fr::from), [11, 13, 17, 21, 23].map(&Fr::from))]
fn test_bits_to_crt(inputs: [Fr; 2], moduli: [Fr; 5]){
    let k = 12;

    let residues = limb_bits_to_crt([&inputs[0], &inputs[1]], &moduli.to_vec()).residues;

    println!("{:?}", residues);
}
