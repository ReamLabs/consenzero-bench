use risc0_zkvm::guest::env;
use tree_hash::{Hash256, TreeHash};

use ream_lib::{
    input::OperationInput,
    input::InputState,
};

use consensus_common::proof::Proof;

fn main() {
    eprintln!("{}:{}: {}", "read-pre-state", "start", env::cycle_count());
    let pre_state: InputState = env::read();
    eprintln!("{}:{}: {}", "read-pre-state", "end", env::cycle_count());

    match &pre_state {
        //
        // Process with full state
        //
        InputState::FullState(state) => {
            // Read inputs
            eprintln!("{}:{}: {}", "read-operation-input", "start", env::cycle_count());
            let input: OperationInput = env::read();
            eprintln!("{}:{}: {}", "read-operation-input", "end", env::cycle_count());

            // Process
            eprintln!("{}:{}: {}", "process-operation", "start", env::cycle_count());
            match input {
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
            }
            eprintln!("{}:{}: {}", "process-operation", "end", env::cycle_count());
        }

        //
        // Process with partial state
        //
        InputState::PartialState => {
            // Read inputs
            eprintln!("{}:{}: {}", "read-operation-input", "start", env::cycle_count());

            // Read and verify slot
            let slot: u64 = env::read();
            let slot_proof: Proof = env::read();
            slot_proof.verify().unwrap();

            // Read and verify latest_block_header_ssz
            let latest_block_header_ssz: Vec<u8> = env::read();
            let latest_block_header: BeaconBlockHeader =
                BeaconBlockHeader::deserialize(&mut latest_block_header_ssz.as_slice()).unwrap();
            let latest_block_header_proof: Proof = env::read();
            latest_block_header_proof.verify().unwrap();

            // Read validator_slashed
            let validator_slashed: bool = env::read();

            // Read and verify proposer_index
            let proposer_index: u64 = env::read();

            // Read and verify block
            let block_ssz: Vec<u8> = env::read();
            let block = BeaconBlock::deserialize(&mut block_ssz.as_slice()).unwrap();

            eprintln!("{}:{}: {}", "read-operation-input", "end", env::cycle_count());

            // Process
            eprintln!("{}:{}: {}", "process-operation", "start", env::cycle_count());
            match input {
                OperationInput::BeaconBlock(_block) => todo!(),
                _ => todo!()
            }
            eprintln!("{}:{}: {}", "process-operation", "end", env::cycle_count());
        }
    }

    //
    // Merkleize the processed state
    //
    eprintln!("{}:{}: {}", "merkleize-operation", "start", env::cycle_count());
    let state_root = match &pre_state {
        InputState::FullState(state) => state.tree_hash_root(),
        InputState::PartialState => Hash256::ZERO
    };
    eprintln!("{}:{}: {}", "merkleize-operation", "end", env::cycle_count());

    //
    // Commit new state root
    //
    eprintln!("{}:{}: {}", "commit", "start", env::cycle_count());
    env::commit(&state_root);
    eprintln!("{}:{}: {}", "commit", "end", env::cycle_count());
}
