#![no_main]
sp1_zkvm::entrypoint!(main);

use passport_lib::validate_passport;

pub fn main() {
    // Read the input
    let mrz = sp1_zkvm::io::read::<String>();
    // Write the mrz to the public input
    sp1_zkvm::io::commit(&mrz);

    // Run the function
    let (is_valid, name) = validate_passport(mrz);

    // Write the result to the public output
    sp1_zkvm::io::commit(&is_valid);
    sp1_zkvm::io::commit(&name);
}
