use halo2_base::Context;
use halo2_base::utils::ScalarField;
//use halo2_proofs::halo2curves::Engine;

pub const NUMBER_OF_TABLES: usize = 4;


pub struct CQContext<F: ScalarField>{
    pub context: Context<F>,
    pub cells_to_lookup: [Vec<F>; NUMBER_OF_TABLES],
}