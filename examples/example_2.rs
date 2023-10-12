// #![allow(unused_imports)]
// #![allow(unused_variables)]
// use halo2_base::gates::builder::{GateThreadBuilder, RangeCircuitBuilder};
// use halo2_base::gates::range::RangeChip;

// use halo2_base::halo2_proofs::{dev::MockProver,
//     halo2curves::bn256::{Fr, Fq},
//     plonk::*,
//     poly::kzg::multiopen::VerifierSHPLONK,
//     poly::kzg::strategy::SingleStrategy,
//     poly::kzg::{
//         commitment::{KZGCommitmentScheme, ParamsKZG},
//         multiopen::ProverSHPLONK,
//     },
//     transcript::{Blake2bRead, TranscriptReadBuffer},
//     transcript::{Blake2bWrite, Challenge255, TranscriptWriterBuffer},
// };
// use halo2_base::utils::biguint_to_fe;
// use num_bigint::BigUint;
// use num_integer::Integer;

// use ark_std::env::set_var;

// use big_field_multiplication::crt_mul;
// use big_field_multiplication::crt_int::biguint_into_crtint_bui_modulus;
// use big_field_multiplication::crt_int::modulus as find_field_modulus;
// use big_field_multiplication::multiplication_gates::crt_to_bits_proof::pow_of_two;
// use big_field_multiplication::loading_tables::{
//     generate_tables::gen_table_multiplication,
//     load_tables::*,
// };

// use ark_std::{Zero, One};

// use halo2_proofs_axiom::halo2curves::FieldExt;


// use rand::Rng;

// fn generate_tables(){

// }


// fn limbs_to_biguint(limbs: [u128; 2])-> BigUint
// {
//     BigUint::from(limbs[0]) + BigUint::from(limbs[1])*pow_of_two()
// }

// fn to_limbs(p: &BigUint) -> [u128; 2]{
//     let mut e = p.iter_u64_digits();

//     let mut limb0 = e.next().unwrap_or(0) as u128;
//     let limb1 = e.next().unwrap_or(0) as u128;    
//     limb0 |= ((limb1 & ((1u128 << 64) - 1u128)) as u128) << 64u32;

//     let mut limb2 = e.next().unwrap_or(0) as u128;
//     let limb3 = e.next().unwrap_or(0) as u128;
//     limb2 |= ((limb3 & ((1u128 << 64) - 1u128)) as u128) << 64u32;
//     [limb0, limb2]
// }


// fn random_moduli() -> Vec<u128>{
//     let mut ans = vec![];

//     for _ in 0..20{
//         let mut rng = rand::thread_rng();
//         let curr: u128 = rng.gen_range(2..200);
//         ans.push(curr);
//     }
//     ans
// }


// fn main(){
//     let k = 11;
//     const K: u32 = 3;
//     let mut rng = rand::rngs::OsRng;
//     let s = <Bn256 as Engine>::Scalar::random(&mut rng);
//     let table_size = 256;
//     let table_path = "/tables";
    
    
// //    use halo2_proofs::plonk::static_lookup::StaticTableValues;
// //    let s = StaticTableValues::new();

//     let table_16_srs =
//         TableSRS::<Bn256>::setup_from_toxic_waste(table_16_size - 1, table_16_size, s);
    
//     let circuit = CQCircuit { table, table_2 };

//     let prover = MockProver::run(K, &circuit, vec![]).unwrap();
//     prover.assert_satisfied();

//     let params = ParamsKZG::<Bn256>::setup_from_toxic_waste(K, s);

//     let config = StaticTableConfig::new(
//         table_16_size,
//         table_16_srs.g1_lagrange().to_vec(),
//         table_16_srs.g_lagrange_opening_at_0().to_vec(),
//     );
//     let mut configs: BTreeMap<usize, StaticTableConfig<Bn256>> = BTreeMap::new();
//     configs.insert(table_16_size, config);

//     let b0_g1_bound = table_16_srs.g1()[((1 << K) + 1)..].to_vec();

//     // Initialize keys
//     let vk = keygen_vk(&params, &circuit).expect("keygen_vk should not fail");
//     let pk =
//         keygen_pk(&params, configs, b0_g1_bound, vk, &circuit).expect("keygen_pk should not fail");

//     // Create proof
//     let proof = {
//         let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
//         // Create a proof
//         create_proof::<Bn256, ProverGWC<_>, _, _, _, _>(
//             &params,
//             &pk,
//             &[circuit],
//             &[&[]],
//             OsRng,
//             &mut transcript,
//         )
//         .unwrap();

//         transcript.finalize()
//     };

//     let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);

//     let verifier_params = params.verifier_params();
//     let strategy = VerificationStrategy::<Bn256, VerifierGWC<_>>::new(verifier_params);

//     let p_batcher = verify_proof::<
//         Bn256,
//         VerifierGWC<_>,
//         _,
//         Blake2bRead<_, _, Challenge255<_>>,
//         AccumulatorStrategy<_>,
//     >(
//         verifier_params,
//         pk.get_vk(),
//         strategy,
//         &[&[]],
//         &mut transcript,
//     )
//     .unwrap();

//     let batched_tuples = p_batcher.finalize();
//     let result: Gt = Bn256::multi_miller_loop(
//         &batched_tuples
//             .iter()
//             .map(|(g1, g2)| (g1, g2))
//             .collect::<Vec<_>>(),
//     );

//     let pairing_result = result.final_exponentiation();
//     assert!(bool::from(pairing_result.is_identity()));
// }

fn main(){}