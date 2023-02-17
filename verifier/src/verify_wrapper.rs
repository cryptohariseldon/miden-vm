use::verify

#![cfg_attr(not(feature = "std"), no_std)]

use air::{ProcessorAir, PublicInputs};
use core::fmt;
use vm_core::{utils::collections::Vec, ProgramOutputs};
use winterfell::VerifierError;
use bytemuck::{Pod, Zeroable};

use miden_assembly::Assembler;
use miden_prover::{prove, ProgramInputs, ProofOptions};

use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};


// EXPORTS
// ================================================================================================

pub use assembly;
pub use vm_core::chiplets::hasher::Digest;
pub use winterfell::StarkProof;

pub fn read_proof() -> StarkProof {
    let mut f = File::open("proofd.bin")?;
    let mut buffer = Vec::new();
    // read the whole file
    let StarkProof serializedproof = f.read_to_end(&mut buffer)?;
    let StarkProof proof = proof.from_bytes();
    let mut filer = File::create("proofd.bin");
    filer?.write_all(&serializedproof)?;
    println!("Proof file written");
}
