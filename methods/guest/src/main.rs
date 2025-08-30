use risc0_zkvm::guest::env;
use tree_hash::TreeHash;

use ream_consensus::electra::beacon_state::BeaconState;
use ream_lib::{
    input::OperationInput,
    ssz::from_ssz_bytes,
};

fn deserialize<T: ssz::Decode>(ssz_bytes: &[u8]) -> T {
    eprintln!("{}-{}:{}: {}", "deserialize", std::any::type_name::<T>(), "start", env::cycle_count());
    let deserialized = from_ssz_bytes(&ssz_bytes).unwrap();
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

    let mut state: BeaconState = deserialize(&pre_state_ssz_bytes);
    
    eprintln!("{}:{}: {}", "read-operation-input", "start", env::cycle_count());
    let input: OperationInput = env::read();
    eprintln!("{}:{}: {}", "read-operation-input", "end", env::cycle_count());

    // Main logic of the program.
    // State transition of the beacon state.

    eprintln!("{}:{}: {}", "process-operation", "start", env::cycle_count());
    input.process(&mut state);
    eprintln!("{}:{}: {}", "process-operation", "end", env::cycle_count());

    // Merkleize the processed state
    eprintln!("{}:{}: {}", "merkleize-operation", "start", env::cycle_count());
    let state_root = state.tree_hash_root();
    eprintln!("{}:{}: {}", "merkleize-operation", "end", env::cycle_count());

    eprintln!("{}:{}: {}", "commit", "start", env::cycle_count());
    env::commit(&state_root);
    eprintln!("{}:{}: {}", "commit", "end", env::cycle_count());
}
