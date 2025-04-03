#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use passport_lib::{validate_passport, PublicValuesStruct};

pub fn main() {
    // Read the input
    let mrz = sp1_zkvm::io::read::<String>();

    // Run the function
    let (is_valid, name) = validate_passport(mrz.clone());

    // Convert public values to bytes
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        mrz,
        is_valid,
        name,
    });

    // Write the bytes as output
    sp1_zkvm::io::commit_slice(&bytes);
}
