use clap::Parser;
use risc0_zkvm::{default_prover, ExecutorEnv};
use tracing::{error, info};

use ream_consensus::deneb::beacon_state::BeaconState as ReamBeaconState;
use ream_lib::{file::read_file, input::InputState, input::OperationInput};

mod cli;
use cli::operation::OperationName;

// // These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// // The ELF is used for proving and the ID is used for verification.
use methods::{CONSENSUS_STF_ELF, CONSENSUS_STF_ID};

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Argument for STFs
    #[clap(flatten)]
    fork: cli::fork::ForkArgs,

    #[clap(flatten)]
    operation: cli::operation::OperationArgs,

    // Pass only partial state into the zkVM
    #[arg(short, long, default_value_t = false)]
    partial_state: bool,

    #[clap(long)]
    excluded_cases: Vec<String>,
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let test_case_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("mainnet")
        .join("tests")
        .join("mainnet");

    if !std::path::Path::new(&test_case_dir).exists() {
        error!("Error: You must first download test data via `make download`");
        std::process::exit(1);
    }

    // Parse the command line arguments.
    let args = Args::parse();

    let fork = args.fork.fork;
    let operation_name = args.operation.operation_name;
    let partial_state = args.partial_state;
    let excluded_cases = args.excluded_cases;

    // Load the test assets.
    // These assets are from consensus-specs repo.
    let base_dir = test_case_dir
        .join(format!("{}", fork))
        .join("operations")
        .join(format!("{}", operation_name))
        .join("pyspec_tests");

    let test_cases = ream_lib::file::get_test_cases(&base_dir);

    for test_case in test_cases {
        if excluded_cases.contains(&test_case) {
            info!("Skipping test case: {}", test_case);
            continue;
        }

        info!("{}", "-".repeat(50));
        info!("[{}] Test case: {}", operation_name, test_case);

        let case_dir = &base_dir.join(&test_case);
        let state: ReamBeaconState = read_file(&case_dir.join("pre.ssz_snappy"));

        let input_path = &case_dir.join(format!("{}.ssz_snappy", operation_name.to_input_name()));

        let input = match operation_name {
            OperationName::Attestation => OperationInput::Attestation(read_file(input_path)),
            OperationName::AttesterSlashing => {
                OperationInput::AttesterSlashing(read_file(input_path))
            }
            OperationName::BlockHeader => OperationInput::BeaconBlock(read_file(input_path)),
            OperationName::BLSToExecutionChange => {
                OperationInput::SignedBLSToExecutionChange(read_file(input_path))
            }
            OperationName::Deposit => OperationInput::Deposit(read_file(input_path)),
            OperationName::ExecutionPayload => {
                OperationInput::BeaconBlockBody(read_file(input_path))
            }
            OperationName::ProposerSlashing => {
                OperationInput::ProposerSlashing(read_file(input_path))
            }
            OperationName::SyncAggregate => OperationInput::SyncAggregate(read_file(input_path)),
            OperationName::VoluntaryExit => {
                OperationInput::SignedVoluntaryExit(read_file(input_path))
            }
            OperationName::Withdrawals => OperationInput::ExecutionPayload(read_file(input_path)),
        };

        //
        // Setup the executor environment and inject inputs
        //
        let env = if partial_state {
            ExecutorEnv::builder()
                .write(&InputState::PartialState)
                .unwrap()
                .write(&input)
                .unwrap()
                .build()
                .unwrap()
        } else {
            ExecutorEnv::builder()
                .write(&InputState::FullState(state.into()))
                .unwrap()
                .write(&input)
                .unwrap()
                .build()
                .unwrap()
        };

        // Execute the program
        let prover = default_prover();

        // Proof information by proving the specified ELF binary.
        // This struct contains the receipt along with statistics about execution of the guest
        let prove_info = prover.prove(env, CONSENSUS_STF_ELF).unwrap();

        // Extract the receipt.
        let receipt = prove_info.receipt;

        receipt.verify(CONSENSUS_STF_ID).unwrap();

        info!("----- Cycle Tracker End -----");

        info!("{}", "-".repeat(50));
    }
}
