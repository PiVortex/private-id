use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const PASSPORT_ELF: &[u8] = include_elf!("passport-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();
    let mrz = std::env::var("PASSPORT_MRZ").expect("PASSPORT_MRZ environment variable not set");

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&mrz);

    println!("mrz: {}", mrz);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(PASSPORT_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        println!("output: {:?}", output);
        println!("\n\n");
        println!("report: {:?}", report);
        // println!("is_valid: {}", output[0]);
        // println!("name: {}", output[1]);

        // assert_eq!(is_valid, true, "The passport is invalid!");
        // println!("The passport is valid!");
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(PASSPORT_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
