use super::*;
use halo2_base::utils::ScalarField;
use rand::rngs::OsRng;
//use ark_poly::{DenseUVPolynomial, GeneralEvaluationDomain, univariate::DensePolynomial, EvaluationDomain};
use ark_ff::{FftField, Zero};
use halo2_proofs::plonk::static_lookup::StaticTableValues;
use halo2_proofs::poly::EvaluationDomain;
use halo2_proofs::halo2curves::bn256::Fr;
use ff::Field;
use group::{Curve, Group};
use std::fs::File;



pub fn table_values<F: ScalarField>(p: u64, table_size: usize) -> Vec<F>  {
    let table_size = table_size as u64;
    assert!(p*p <= table_size);

    let mut t_evals: Vec<F> = vec![]; 
    let p_f = F::from(p);
    
    let mut p_squared = p*p;
    for i in 0..p{
        for j in 0..p{
            let prod = (i*j) % p;
            t_evals.push(F::from(i) + F::from(j) * p_f + F::from(prod)* p_f*p_f);
        }
    }

    while p_squared < table_size{
        p_squared +=1;
        t_evals.push(p_f*F::from(p_squared));
    }
    println!("hi, {}", t_evals.len());

    t_evals
}


pub fn gen_table_multiplication<E: Engine + MultiMillerLoop>(p: u64, log_table_size: usize, toxic_waste: E::Scalar) -> StaticTable<E>
where E::Scalar : ScalarField,
{
    let domain = EvaluationDomain::<E::Scalar>::new(2, 1);

    let w = domain.get_omega();

    let table_size = 1 << log_table_size;
    
    let params =
        TableSRS::<E>::setup_from_toxic_waste(table_size-1, table_size, toxic_waste);
    
    let table_values = table_values::<E::Scalar>(p, table_size);
    let table = StaticTableValues::<E>::new(&table_values[..], &params.g1());

    let committed = table.commit(params.g1().len(), params.g2(), 2*log_table_size);
    
    let t = StaticTable {
        opened: Some(table),
        committed: Some(committed),
    };

    t
}

#[test]
fn test_cq(){
    use halo2_proofs::halo2curves::bn256::Fr;

    let primes = [5, 7, 9, 11];
    let log_table_size = 7;

    let toxic_waste = {
        let mut rng = rand::rngs::OsRng;
        let s = <Bn256 as Engine>::Scalar::random(&mut rng);
        s
    };

    let mut tables = vec![];

    for p in primes{
        let table = gen_table_multiplication::<Bn256>(p, log_table_size, toxic_waste);
        tables.push(table);
    }    
}


