use group::{Curve, Group};
use rand::{Rng, SeedableRng};
use std::{collections::BTreeMap, fmt::Debug, marker::PhantomData};

use ff::{Field, PrimeField};
use halo2_proofs::{
    circuit::{SimpleFloorPlanner, Value},
    dev::MockProver,
    plonk::{
        create_proof, keygen_pk, keygen_vk,
        static_lookup::{
                StaticCommittedTable, StaticTable, StaticTableConfig, StaticTableId, StaticTableValues,
        },
        verify_proof, Advice, Circuit, Column, Selector,
    },
    poly::{
        commitment::ParamsProver,
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG, TableSRS},
            multiopen::{ProverGWC, VerifierGWC},
            strategy::AccumulatorStrategy,
        },
        Rotation, VerificationStrategy,
    },
    transcript::{
        Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
    },
};
use halo2_proofs::halo2curves::{
    bn256::{Bn256, Fq2Bytes, Gt},
    pairing::{Engine, MillerLoopResult, MultiMillerLoop},
    serde::SerdeObject,
    CurveAffine, FieldExt,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;


pub mod circuit;
pub mod load_tables;
pub mod generate_tables;

fn lookup_gate(){}