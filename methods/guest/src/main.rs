use risc0_zkvm::guest::env;
use tree_hash::{Hash256, TreeHash};

use ream_lib::{
    beacon_state::BeaconState,
    input::OperationInput,
    input::InputState,
};

use consensus_common::proof::Proof;

fn main() {
    eprintln!("{}:{}: {}", "read-pre-state", "start", env::cycle_count());
    let pre_state: InputState = env::read();
    eprintln!("{}:{}: {}", "read-pre-state", "end", env::cycle_count());

    eprintln!("{}:{}: {}", "read-operation-input", "start", env::cycle_count());
    let input: OperationInput = env::read();
    eprintln!("{}:{}: {}", "read-operation-input", "end", env::cycle_count());

    // Main logic of the program.
    // State transition of the beacon state.

    eprintln!("{}:{}: {}", "process-operation", "start", env::cycle_count());

    match (&pre_state, input) {
        //
        // Full State inputs
        //

        (InputState::FullState(state), OperationInput::Attestation(attestation)) => {
            let _ = state.clone().process_attestation(&attestation);
        }
        (InputState::FullState(state), OperationInput::AttesterSlashing(attester_slashing)) => {
            let _ = state.clone().process_attester_slashing(&attester_slashing);
        }
        (InputState::FullState(state), OperationInput::BeaconBlock(block)) => {
            let _ = state.clone().process_block_header(&block);
        }
        (InputState::FullState(state), OperationInput::SignedBLSToExecutionChange(bls_change)) => {
            let _ = state.clone().process_bls_to_execution_change(&bls_change);
        }
        (InputState::FullState(state), OperationInput::Deposit(deposit)) => {
            let _ = state.clone().process_deposit(&deposit);
        }
        (InputState::FullState(_state), OperationInput::BeaconBlockBody(_block_body)) => {
            todo!("Not implemented");
            // let _ = state.clone().process_execution_payload(&block_body);
        }
        (InputState::FullState(state), OperationInput::ProposerSlashing(proposer_slashing)) => {
            let _ = state.clone().process_proposer_slashing(&proposer_slashing);
        }
        (InputState::FullState(state), OperationInput::SyncAggregate(sync_aggregate)) => {
            let _ = state.clone().process_sync_aggregate(&sync_aggregate);
        }
        (InputState::FullState(state), OperationInput::SignedVoluntaryExit(voluntary_exit)) => {
            let _ = state.clone().process_voluntary_exit(&voluntary_exit);
        }
        (InputState::FullState(state), OperationInput::ExecutionPayload(execution_payload)) => {
            let _ = state.clone().process_withdrawals(&execution_payload);
        }

        //
        // Partial state inputs
        //

        (InputState::PartialState(_proof), OperationInput::BeaconBlock(_block)) => {
            todo!();
        }
        _ => todo!()
    }

    eprintln!("{}:{}: {}", "process-operation", "end", env::cycle_count());

    //
    // Merkleize the processed state
    //
    eprintln!("{}:{}: {}", "merkleize-operation", "start", env::cycle_count());
    let state_root = match &pre_state {
        InputState::FullState(state) => state.tree_hash_root(),
        InputState::PartialState(_) => Hash256::ZERO
    };
    eprintln!("{}:{}: {}", "merkleize-operation", "end", env::cycle_count());

    //
    // Commit new state root
    //
    eprintln!("{}:{}: {}", "commit", "start", env::cycle_count());
    env::commit(&state_root);
    eprintln!("{}:{}: {}", "commit", "end", env::cycle_count());
}
