use alloy_primitives::FixedBytes;
use serde::{Deserialize, Serialize};
use ssz_rs::proofs::Proof;

use consensus_common::{
    beacon_block::BeaconBlock,
    beacon_block_header::BeaconBlockHeader,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum PartialInput {
    BeaconBlock(BeaconBlockInput)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeaconBlockInput {
    // Pre-state inputs
    witness: FixedBytes<32>,
    slot: u64,
    slot_proof: Proof,
    latest_block_header: BeaconBlockHeader,
    latest_block_header_proof: Proof,
    validator_slashed: bool,
    validator_slashed_proof: Proof,
    proposer_index: u64,

    // Actual input
    block: BeaconBlock
}
