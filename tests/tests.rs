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
    use num_bigint::BigUint;
    use rand::rngs::OsRng;
    use std::marker::PhantomData;

    use serde::{Deserialize, Serialize};
    use ark_std::fs::File;

    use criterion::{criterion_group, criterion_main};
    use criterion::{BenchmarkId, Criterion};

    use big_field_multiplication::multiplication_gates::mod_p_verifications::mod_r_mul;
    use big_field_multiplication::crt_mul;

    use test_case::test_case;


    fn read_inputs(i: i32) -> [u64; 4]{
        let path = format!("tests/tests_input{i}.in");
        serde_json::from_reader(
            File::open(path).unwrap_or_else(|e| panic!("")),
        )
        .unwrap()
    }

    #[test_case(1 => Fr::from(0))]//, b"1*2 != 3*p + 4")]
    #[test_case(2 => Fr::from(1))]//, b"5*6 == 4*p + 2")]

    fn test_crt_mod_p_mul(i: i32) -> Fr{
        let k = 6;
        let mut builder = GateThreadBuilder::new(false);
        let chip = GateChip::default();
        let ctx = builder.main(0);

        let p = Fr::from(7);
        let p = ctx.load_constant(p);

        let inputs = read_inputs(i).map(Fr::from);
        
        let res = mod_r_mul(&chip, ctx,  &inputs, p);

        builder.config(k, Some(9));
        
        let circuit = GateCircuitBuilder::mock(builder);

        MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

        *res.value()
    }


    // #[test_case([1, 2, 3, 4, 5].map(Fr::from))]//, b"1*2 != 3*4 + 5 mod p")]
    // #[test_case([9, 6, 4, 10, 5].map(Fr::from))]//, b"4*10 == 9*6 + 5")]

    // fn test_crt_mul<F: ScalarField>(inputs: [F; 5]){
        
    //     let moduli = [10, 17, 19, 21];
    //     let moduli = Vec::from(moduli.map(F::from));
    //     let k = 6;
    //     let mut builder = GateThreadBuilder::new(false);
    //     let chip = GateChip::default();
        
    //     crt_mul(&chip, builder.main(0), inputs, &moduli);

    //     builder.config(k, Some(9));
        
    //     let circuit = GateCircuitBuilder::mock(builder);

    //     MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();
    // }
