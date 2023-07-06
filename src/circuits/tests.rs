use crate::circuits::utils::mod_p_mul;
use super::* ;



use test_case::test_case;

#[test_case([1, 2, 3, 4, 5].map(Fr::from) => Fr::from(0) ; b"1*2 != 3*4 + 5 mod p")]
#[test_case([9, 6, 4, 10, 5].map(Fr::from) => Fr::from(1) ; b"4*10 == 9*6 + 5")]

fn test_crt_mul<F: ScalarField>(inputs: [F; 5]) -> F{
    let k = 6;
    let mut builder = GateThreadBuilder::mock();
    
    let res = mod_p_mul(builder.main(0), inputs);

    // auto-tune circuit
    builder.config(k, Some(9));
    // create circuit
    
    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

    *res.value()
}
