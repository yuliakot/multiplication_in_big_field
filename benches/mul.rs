#![allow(unused_imports)]
#![allow(unused_variables)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use big_field_multiplication::crt_mul; // function to profile
use big_field_multiplication::crt_int::{CRTint, biguint_into_crtint_bui_modulus, modulus}; // function to profile

use num_bigint::{BigUint, RandBigInt};
use ark_std::env::set_var;

use halo2_base::{
    halo2_proofs::halo2curves::bn256::{Fr, Fq},
    utils::{ScalarField, biguint_to_fe},
    gates:: RangeChip,
    gates::builder::{GateThreadBuilder, CircuitBuilderStage, RangeCircuitBuilder, MultiPhaseThreadBreakPoints},
};
use rand::rngs::OsRng;

 
fn input_generator<F: ScalarField>() -> (    
    BigUint,
    BigUint,
    Vec<BigUint>){
        let mut rng = rand::thread_rng();
        let [a, b]: [BigUint; 2] = [rng.gen_biguint(253), rng.gen_biguint(253)];

        let moduli =  [0..10].map(|_| rng.gen_biguint(5)).to_vec();
        

        (a, b, moduli)
    }

        
fn crt_mul_circuit(
    stage: CircuitBuilderStage,
    bases: Vec<G1Affine>,
    scalars: Vec<Fr>,
    break_points: Option<MultiPhaseThreadBreakPoints>,
) -> RangeCircuitBuilder<Fr> {
    let k = 12;
    let mut builder = match stage {
        CircuitBuilderStage::Mock => GateThreadBuilder::mock(),
        CircuitBuilderStage::Prover => GateThreadBuilder::prover(),
        CircuitBuilderStage::Keygen => GateThreadBuilder::keygen(),
    };

    let start0 = start_timer!(|| format!("Witness generation for circuit in {stage:?} stage"));
    fixed_base_msm_bench(&mut builder, params, bases, scalars);

    let circuit = match stage {
        CircuitBuilderStage::Mock => {
            builder.config(k, Some(20));
            RangeCircuitBuilder::mock(builder)
        }
        CircuitBuilderStage::Keygen => {
            builder.config(k, Some(20));
            RangeCircuitBuilder::keygen(builder)
        }
        CircuitBuilderStage::Prover => RangeCircuitBuilder::prover(builder, break_points.unwrap()),
    };
    end_timer!(start0);
    circuit
}



pub fn criterion_benchmark(c: &mut Criterion) {
    
    let (a, b, moduli) = input_generator();
    
    let p = modulus::<Fq>();
    let crt_p = biguint_into_crtint_bui_modulus(&p, &moduli);

    let mut group = c.benchmark_group("mul");
    group.sample_size(10);
    group.bench_with_input(
        "mul", 
        &(&a, &b, &crt_p, &moduli),
        |bencher, &(a, b, crt_p, moduli)| {
            let k = 12;
            let lookup_bits = 10;
            let mut builder = GateThreadBuilder::new(false);
            let chip = RangeChip::<Fr>::default(lookup_bits);
            set_var("LOOKUP_BITS", lookup_bits.to_string());
            let ctx = builder.main(0);
            bencher.iter(|| {
                let circuit = fixed_base_msm_circuit(
                    config,
                    CircuitBuilderStage::Prover,
                    bases.clone(),
                    scalars.clone(),
                    Some(break_points.clone()),
                );
            }
        
        }
            );
    group.finish();
    
    group.bench_with_input(
        BenchmarkId::new("fixed base msm", k),
        &(&params, &pk, &bases, &scalars),
        |b, &(params, pk, bases, scalars)| {
            b.iter(|| {
                let circuit = fixed_base_msm_circuit(
                    config,
                    CircuitBuilderStage::Prover,
                    bases.clone(),
                    scalars.clone(),
                    Some(break_points.clone()),
                );

                let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
                create_proof::<
                    KZGCommitmentScheme<Bn256>,
                    ProverSHPLONK<'_, Bn256>,
                    Challenge255<G1Affine>,
                    _,
                    Blake2bWrite<Vec<u8>, G1Affine, Challenge255<_>>,
                    _,
                >(params, pk, &[circuit], &[&[]], &mut rng, &mut transcript)
                .expect("prover should not fail");
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);