use super::*;

use std::io::BufWriter;

#[derive(Clone, Copy, Debug, Subcommand)]
pub enum SnarkCmd {
    /// Run the mock prover
    Mock,
    /// Generate new proving & verifying keys
    Keygen,
    /// Generate a new proof
    Prove,
    /// Verify a proof
    Verify,
}

impl std::fmt::Display for SnarkCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mock => write!(f, "mock"),
            Self::Keygen => write!(f, "keygen"),
            Self::Prove => write!(f, "prove"),
            Self::Verify => write!(f, "verify"),
        }
    }
}

// ascii of cq
static SEED: [u8; 32] = [
    99, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0,
];

fn generate_table(params: &TableSRS<Bn256>, k: usize) -> (StaticTable<Bn256>, StaticTable<Bn256>) {
    use halo2_proofs::halo2curves::bn256::Fr;

    let table_values = [
        Fr::from(0),
        Fr::from(1),
        Fr::from(6),
        Fr::from(8),
        Fr::from(10),
        Fr::from(12),
        Fr::from(14),
        Fr::from(16),
        Fr::from(18),
        Fr::from(20),
        Fr::from(22),
        Fr::from(24),
        Fr::from(26),
        Fr::from(28),
        Fr::from(30),
        Fr::from(32),
    ];

    let table_2_values = [
        Fr::from(0),
        Fr::from(2),
        Fr::from(3),
        Fr::from(4),
        Fr::from(5),
        Fr::from(6),
        Fr::from(7),
        Fr::from(8),
        Fr::from(9),
        Fr::from(10),
        Fr::from(11),
        Fr::from(12),
        Fr::from(13),
        Fr::from(14),
        Fr::from(15),
        Fr::from(16),
    ];

    let n = 1 << k;
    let table = StaticTableValues::new(&table_values, &params.g1());
    let table_2 = StaticTableValues::new(&table_2_values, &params.g1());

    let committed = table.commit(params.g1().len(), params.g2(), n);
    let committed_2 = table_2.commit(params.g1().len(), params.g2(), n);

    let t1 = StaticTable {
        opened: Some(table),
        committed: Some(committed),
    };

    let t2 = StaticTable {
        opened: Some(table_2),
        committed: Some(committed_2),
    };

    (t1, t2)
}

