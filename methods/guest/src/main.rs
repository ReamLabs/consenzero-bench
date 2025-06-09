use risc0_zkvm::guest::env;
use tree_hash::TreeHash;

use ream_consensus::{
    attestation::Attestation,
    attester_slashing::AttesterSlashing,
    bls_to_execution_change::SignedBLSToExecutionChange,
    electra::{
        beacon_block::BeaconBlock,
        beacon_state::BeaconState,
        execution_payload::ExecutionPayload,
    },
    deposit::Deposit,
    proposer_slashing::ProposerSlashing,
    sync_aggregate::SyncAggregate,
    voluntary_exit::SignedVoluntaryExit,
};
use ream_lib::{
    input::OperationInput,
    ssz::to_ssz,
};

fn deserialize<T: ssz::Decode>(ssz_bytes: &[u8]) -> T {
    eprintln!("{}-{}:{}: {}", "deserialize", std::any::type_name::<T>(), "start", env::cycle_count());
    let deserialized = to_ssz(&ssz_bytes).unwrap();
    eprintln!("{}-{}:{}: {}", "deserialize", std::any::type_name::<T>(), "end", env::cycle_count());

    deserialized
}

fn main() {
    // Read inputs to the program.

    eprintln!("{}:{}: {}", "read-pre-state-ssz", "start", env::cycle_count());
    let pre_state_len: usize = env::read();
    let mut pre_state_ssz_bytes = vec![0u8; pre_state_len];
    env::read_slice(&mut pre_state_ssz_bytes);
    eprintln!("{}:{}: {}", "read-pre-state-ssz", "end", env::cycle_count());

    let pre_state: BeaconState = deserialize(&pre_state_ssz_bytes);

    eprintln!("{}:{}: {}", "read-operation-input", "start", env::cycle_count());
    let input: OperationInput = env::read();
    eprintln!("{}:{}: {}", "read-operation-input", "end", env::cycle_count());

    // Main logic of the program.
    // State transition of the beacon state.

    eprintln!("{}:{}: {}", "process-operation", "start", env::cycle_count());

    match input {
        OperationInput::Attestation(ssz_bytes) => {
            let attestation: Attestation = deserialize(&ssz_bytes);
            let _ = pre_state.clone().process_attestation(&attestation);
        }
        OperationInput::AttesterSlashing(ssz_bytes) => {
            let attester_slashing: AttesterSlashing = deserialize(&ssz_bytes);
            let _ = pre_state.clone().process_attester_slashing(&attester_slashing);
        }
        OperationInput::BeaconBlock(ssz_bytes) => {
            let block: BeaconBlock = deserialize(&ssz_bytes);
            let _ = pre_state.clone().process_block_header(&block);
        }
        OperationInput::SignedBLSToExecutionChange(ssz_bytes) => {
            let bls_change: SignedBLSToExecutionChange = deserialize(&ssz_bytes);
            let _ = pre_state.clone().process_bls_to_execution_change(&bls_change);
        }
        OperationInput::Deposit(ssz_bytes) => {
            let deposit: Deposit = deserialize(&ssz_bytes);
            let _ = pre_state.clone().process_deposit(&deposit);
        }
        OperationInput::BeaconBlockBody(_ssz_bytes) => {
            panic!("Not implemented");
            // let block_body: BeaconBlockBody = deserialize(&ssz_bytes);
            // let _ = pre_state.clone().process_execution_payload(&block_body);
        }
        OperationInput::ProposerSlashing(ssz_bytes) => {
            let proposer_slashing: ProposerSlashing = deserialize(&ssz_bytes);
            let _ = pre_state.clone().process_proposer_slashing(&proposer_slashing);
        }
        OperationInput::SyncAggregate(ssz_bytes) => {
            let sync_aggregate: SyncAggregate = deserialize(&ssz_bytes);
            let _ = pre_state.clone().process_sync_aggregate(&sync_aggregate);
        }
        OperationInput::SignedVoluntaryExit(ssz_bytes) => {
            let voluntary_exit: SignedVoluntaryExit = deserialize(&ssz_bytes);
            let _ = pre_state.clone().process_voluntary_exit(&voluntary_exit);
        }
        OperationInput::ExecutionPayload(ssz_bytes) => {
            let execution_payload: ExecutionPayload = deserialize(&ssz_bytes);
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
