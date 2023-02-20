#![cfg_attr(not(feature = "std"), no_std)]

use air::{ProcessorAir, PublicInputs};
use core::fmt;
use vm_core::{utils::collections::Vec, ProgramOutputs};
use vm_core::Felt;

use winterfell::VerifierError;
use winter_utils::{Deserializable, SliceReader};

use bytemuck::{Pod, Zeroable};
use hex;

//use rescue::rp64::Element;
//use rescue::rp64::params::MDS;
//use rescue::rescue::{RescueHashParams, StatefulRescueHash};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};


//use rust_crypto::util::hex;



// EXPORTS
// ================================================================================================

pub use assembly;
//pub use crypto::hash::rescue::rp64_256::digest::ElementDigest;
//use vm_core::ProgramOutputs;
//use vm_core::{chiplets::hasher::Digesty, Program, ProgramInputs};
pub use vm_core::chiplets::hasher::Digest;
//pub use vm_core::chiplets::hasher::Digest::read_from;

//use winter_crypto::hash::rescue::rp64_256::digest::ElementDigest;
//use winter_crypto::Digest;

pub use winterfell::StarkProof;

use bytemuck;



// VERIFIER
// ================================================================================================
/// Returns Ok(()) if the specified program was executed correctly against the specified inputs
/// and outputs.
///
/// Specifically, verifies that if a program with the specified `program_hash` is executed against
/// the provided `stack_inputs` and some secret inputs, the result is equal to the `stack_outputs`.
///
/// Stack inputs are expected to be ordered as if they would be pushed onto the stack one by one.
/// Thus, their expected order on the stack will be the reverse of the order in which they are
/// provided, and the last value in the `stack_inputs` slice is expected to be the value at the top
/// of the stack.
///
/// Stack outputs are expected to be ordered as if they would be popped off the stack one by one.
/// Thus, the value at the top of the stack is expected to be in the first position of the
/// `stack_outputs` slice, and the order of the rest of the output elements will also match the
/// order on the stack. This is the reverse of the order of the `stack_inputs` slice.
///
/// # Errors
/// Returns an error if the provided proof does not prove a correct execution of the program.

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
//#[derive(Copy, Clone, Pod, Zeroable)]
//#[repr(C)]
pub struct StkProof{
    pub proof: StarkProof,
}

pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account storing the Proof Data
    _instruction_data: &[u8],// String
) -> ProgramResult {
    msg!("Zilch verification program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    //load pointers to proof info in account
    //hardcoded values for test 1.

    //load account where Proof is stored.
    //let account_key = Pubkey::from_str("7fkcbALYxj5BDq6ZDtWF2hvLMx5K4jTKUrWE4eaxvgoE").unwrap();
    //let account_info = AccountInfo::from_account(&account, false);
    //let account_key = Pubkey::decode("7fkcbALYxj5BDq6ZDtWF2hvLMx5K4jTKUrWE4eaxvgoE");
    let accts = account;
    //let accts: &AccountInfo = "7fkcbALYxj5BDq6ZDtWF2hvLMx5K4jTKUrWE4eaxvgoE"; //proof deployed on devnet
    //let raw_proof = bytemuck::try_from_bytes(accts.data.borrow()) //.unwrap();
    let slice = &*accts.data.borrow();
    //let raw_proof = bytemuck::try_from_bytes(&slice).unwrap();
    let my_proof = StarkProof::from_bytes(slice).unwrap();
    //let mut my_proof = StkProof::try_from_slic(&accts.data.borrow())?;
    //let program_hash = "c8653f31a1098e1b83c5d4972ec544cac00aa784bba18b5a9db7478977d38e68";
    //let program_hash = Element:from_slice("c8653f31a1098e1b83c5d4972ec544cac00aa784bba18b5a9db7478977d38e68");
    //let program_hash_bytes = hex::from_hex(program_hash_str).unwrap();
    //let program_hash = ElementDigest::from_slice(&program_hash2);
    //let program_hash= Digest::read_from("c8653f31a1098e1b83c5d4972ec544cac00aa784bba18b5a9db7478977d38e68");
    let program_hash_string = "c8653f31a1098e1b83c5d4972ec544cac00aa784bba18b5a9db7478977d38e68";
    let program_hash_bytes = hex::decode(program_hash_string).unwrap();
    let mut program_hash_slice = SliceReader::new(&program_hash_bytes);
    let proghash = Digest::read_from(&mut program_hash_slice).unwrap();
        //.map_err(|err| format!("Failed to deserialize program hash from bytes - {}", err))?;
    //let proghash = Digest::from(program_hash_bytes);
    let inputs: &[u64] = &[1, 1];
    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    //let outputs2 = ["8911120806959712300", "11112721240812633725", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0"];

    //let outputs: &ProgramOutputs::from_bytes<outputs2>;
    let stack1 : Vec<u64> = Vec::from([8911120806959712300, 11112721240812633725, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let ofl1: Vec<u64> = [].to_vec();
    let outputs = &ProgramOutputs::new(stack1, ofl1);
    let mut outcome = verify(proghash, inputs, outputs, my_proof);

    // Increment and store the number of times the account has been greeted
    //let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    //greeting_account.counter += 1;
    //greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("verification complete");

    Ok(())
}

pub fn verify(
    program_hash: Digest,
    stack_inputs: &[u64],
    outputs: &ProgramOutputs,
    proof: StarkProof,
) -> Result<(), VerificationError> {
    // convert stack inputs to field elements
    let mut stack_input_felts = Vec::with_capacity(stack_inputs.len());
    for &input in stack_inputs.iter().rev() {
        stack_input_felts.push(
            input
                .try_into()
                .map_err(|_| VerificationError::InputNotFieldElement(input))?,
        );
    }

    // build public inputs and try to verify the proof
    let pub_inputs = PublicInputs::new(program_hash, stack_input_felts, outputs.clone());
    winterfell::verify::<ProcessorAir>(proof, pub_inputs).map_err(VerificationError::VerifierError)
}

// ERRORS
// ================================================================================================

/// TODO: add docs, implement Display
#[derive(Debug, PartialEq, Eq)]
pub enum VerificationError {
    VerifierError(VerifierError),
    InputNotFieldElement(u64),
    OutputNotFieldElement(u64),
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: implement friendly messages
        write!(f, "{self:?}")
    }
}
