use super::*;
use crate::utils::NUMBER_OF_TABLES;

#[derive(Clone)]
struct CQCircuit<E: MultiMillerLoop> {
    tables: [StaticTable<E>; NUMBER_OF_TABLES],
}

impl<E: MultiMillerLoop<Scalar = F>, F: Field + FieldExt> Circuit<E> for CQCircuit<E> {
    type Config = [Column<Advice>; NUMBER_OF_TABLES];

    type FloorPlanner = SimpleFloorPlanner<E>;

    fn without_witnesses(&self) -> Self {
        self.clone()
    }

    fn configure(meta: &mut halo2_proofs::plonk::ConstraintSystem<F>) -> Self::Config {
        let advice_columns: Vec<_>  = (0..NUMBER_OF_TABLES).map(|_| meta.advice_column()).collect();
        meta.lookup_static("lookup_bits", |meta| {
            advice_columns.iter().enumerate().map(|(i, &advice)|
                (
                    meta.query_advice(advice, Rotation::cur()),
                    StaticTableId(String::from(format!("table_{i}"))),
                ),
            ).collect()
        });

        advice_columns.try_into().unwrap()

    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl halo2_proofs::circuit::Layouter<F, E = E>,
    ) -> Result<(), halo2_proofs::plonk::Error> {
        for i in 0..NUMBER_OF_TABLES{
            layouter.register_static_table(StaticTableId(String::from(format!("table_{i}"))), self.tables[i].clone());
        }

        layouter.assign_region(
            || "",
            |mut region| {
                region.assign_advice(
                    config[0],
                    0,
                    Value::known(<E as Engine>::Scalar::from_u128(30)),
                )?;
                region.assign_advice(
                    config[0],
                    1,
                    Value::known(<E as Engine>::Scalar::from_u128(6)),
                )?;
                region.assign_advice(
                    config[0],
                    0,
                    Value::known(<E as Engine>::Scalar::from_u128(15)),
                )?;
                region.assign_advice(
                    config[0],
                    1,
                    Value::known(<E as Engine>::Scalar::from_u128(3)),
                )?;

                Ok(())
            },
        )?;

        Ok(())
    }
}


