
use super::*;
use halo2_base::halo2_proofs::halo2curves::bn256::{Bn256, Fr, G1Affine};

use test_case::test_case;
#[test_case(&BigUint::parse_bytes(b"150", 10).unwrap() => [150, 0].map(Fr::from))]//
#[test_case(&BigUint::parse_bytes(b"200_000_000_000_000_000_000_000_000_000_000", 16).unwrap()
             => [0, 2].map(Fr::from); "Check that 2 times 16 to pow 32 == [0, 2] in limbs")]//

fn test_limbs(a: &BigUint) -> [Fr; 2]{
    let moduli: Vec<u64> = vec![3, 4, 5, 7];
    let moduli = &moduli.iter().map(|x| Fr::from(*x)).collect::<Vec::<Fr>>();
    let p = BigUint::from(11u32);
    let crt_integer = biguint_into_crtint(a, moduli, &p);
    crt_integer.limbs_as_fe

}
