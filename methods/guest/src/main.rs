use risc0_zkvm::guest::env;
use tree_hash::TreeHash;

use ream_lib::{
    beacon_state::BeaconState,
    input::OperationInput
};

fn main() {
    // Read an input to the program.

    eprintln!("{}:{}: {}", "read-pre-state", "start", env::cycle_count());
    let pre_state: BeaconState = env::read();
    eprintln!("{}:{}: {}", "read-pre-state", "end", env::cycle_count());

    eprintln!("{}:{}: {}", "read-operation-input", "start", env::cycle_count());
    let input: OperationInput = env::read();
    eprintln!("{}:{}: {}", "read-operation-input", "end", env::cycle_count());

    // Main logic of the program.
    // State transition of the beacon state.

    eprintln!("{}:{}: {}", "process-operation", "start", env::cycle_count());

    match input {
        OperationInput::Attestation(attestation) => {
            let _ = pre_state.clone().process_attestation(&attestation);
        }
        OperationInput::AttesterSlashing(attester_slashing) => {
            let _ = pre_state.clone().process_attester_slashing(&attester_slashing);
        }
        OperationInput::BeaconBlock(block) => {
            let _ = pre_state.clone().process_block_header(&block);
        }
        OperationInput::SignedBLSToExecutionChange(bls_change) => {
            let _ = pre_state.clone().process_bls_to_execution_change(&bls_change);
        }
        OperationInput::Deposit(deposit) => {
            let _ = pre_state.clone().process_deposit(&deposit);
        }
        OperationInput::BeaconBlockBody(_block_body) => {
            panic!("Not implemented");
            // let _ = pre_state.clone().process_execution_payload(&block_body);
        }
        OperationInput::ProposerSlashing(proposer_slashing) => {
            let _ = pre_state.clone().process_proposer_slashing(&proposer_slashing);
        }
        OperationInput::SyncAggregate(sync_aggregate) => {
            let _ = pre_state.clone().process_sync_aggregate(&sync_aggregate);
        }
        OperationInput::SignedVoluntaryExit(voluntary_exit) => {
            let _ = pre_state.clone().process_voluntary_exit(&voluntary_exit);
        }
        OperationInput::ExecutionPayload(execution_payload) => {
            let _ = pre_state.clone().process_withdrawals(&execution_payload);
        }
    }

    eprintln!("{}:{}: {}", "process-operation", "end", env::cycle_count());

    // Merkleize the processed state
    eprintln!("{}:{}: {}", "merkleize-operation", "start", env::cycle_count());
    let state_root = pre_state.tree_hash_root();
    eprintln!("{}:{}: {}", "merkleize-operation", "end", env::cycle_count());

    eprintln!("{}:{}: {}", "commit", "start", env::cycle_count());
    env::commit(&state_root);
    eprintln!("{}:{}: {}", "commit", "end", env::cycle_count());
}
