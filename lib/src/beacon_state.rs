use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::{
    typenum::{U16777216, U2048, U4, U536870912, U65536, U8192},
    BitVector, FixedVector, VariableList,
};
use std::sync::Arc;
use tree_hash_derive::TreeHash;

use ream_consensus::{
    attestation::Attestation,
    attester_slashing::AttesterSlashing,
    beacon_block_header::BeaconBlockHeader,
    bls_to_execution_change::SignedBLSToExecutionChange,
    checkpoint::Checkpoint,
    deneb::{
        beacon_block::BeaconBlock, beacon_state::BeaconState as ReamBeaconState,
        execution_payload::ExecutionPayload, execution_payload_header::ExecutionPayloadHeader,
    },
    deposit::Deposit,
    eth_1_data::Eth1Data,
    fork::Fork,
    historical_summary::HistoricalSummary,
    proposer_slashing::ProposerSlashing,
    sync_aggregate::SyncAggregate,
    sync_committee::SyncCommittee,
    validator::Validator,
    voluntary_exit::SignedVoluntaryExit,
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Encode, Decode, TreeHash)]
pub struct BeaconState {
    // Versioning
    pub genesis_time: u64,
    pub genesis_validators_root: B256,
    pub slot: u64,
    pub fork: Fork,

    // History
    pub latest_block_header: BeaconBlockHeader,
    pub block_roots: FixedVector<B256, U8192>,
    pub state_roots: FixedVector<B256, U8192>,
    // Frozen in Capella, replaced by historical_summaries
    pub historical_roots: VariableList<B256, U16777216>,

    // Eth1
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: VariableList<Eth1Data, U2048>,
    pub eth1_deposit_index: u64,

    // Registry
    // Using U536870912 (2^29) because risc0 guest fails for U4294967296 (2^32) and above,
    // and U2147483648 (2^31) and U1073741824 (2^30) fails to merkleize
    pub validators: VariableList<Validator, U536870912>,
    pub balances: VariableList<u64, U536870912>,

    // Randomness
    pub randao_mixes: FixedVector<B256, U65536>,

    // Slashings
    pub slashings: FixedVector<u64, U8192>,

    // // Participation
    pub previous_epoch_participation: VariableList<u8, U536870912>,
    pub current_epoch_participation: VariableList<u8, U536870912>,

    // Finality
    pub justification_bits: BitVector<U4>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,

    // // Inactivity
    pub inactivity_scores: VariableList<u64, U536870912>,

    // Sync
    pub current_sync_committee: Arc<SyncCommittee>,
    pub next_sync_committee: Arc<SyncCommittee>,

    // Execution
    pub latest_execution_payload_header: ExecutionPayloadHeader,

    // Withdrawals
    pub next_withdrawal_index: u64,
    pub next_withdrawal_validator_index: u64,

    // Deep history valid from Capella onwards.
    pub historical_summaries: VariableList<HistoricalSummary, U16777216>,
}

impl BeaconState {
    pub fn process_attestation(self, attestation: &Attestation) -> anyhow::Result<()> {
        ReamBeaconState::process_attestation(&mut self.into(), attestation)
    }

    pub fn process_attester_slashing(
        self,
        attester_slashing: &AttesterSlashing,
    ) -> anyhow::Result<()> {
        ReamBeaconState::process_attester_slashing(&mut self.into(), attester_slashing)
    }

    pub fn process_block_header(self, block: &BeaconBlock) -> anyhow::Result<()> {
        ReamBeaconState::process_block_header(&mut self.into(), block)
    }

    pub fn process_bls_to_execution_change(
        self,
        signed_address_change: &SignedBLSToExecutionChange,
    ) -> anyhow::Result<()> {
        ReamBeaconState::process_bls_to_execution_change(&mut self.into(), signed_address_change)
    }

    pub fn process_deposit(self, deposit: &Deposit) -> anyhow::Result<()> {
        ReamBeaconState::process_deposit(&mut self.into(), deposit)
    }

    pub fn process_proposer_slashing(
        self,
        proposer_slashing: &ProposerSlashing,
    ) -> anyhow::Result<()> {
        ReamBeaconState::process_proposer_slashing(&mut self.into(), proposer_slashing)
    }

    pub fn process_sync_aggregate(self, sync_aggregate: &SyncAggregate) -> anyhow::Result<()> {
        ReamBeaconState::process_sync_aggregate(&mut self.into(), sync_aggregate)
    }

    pub fn process_voluntary_exit(
        self,
        signed_voluntary_exit: &SignedVoluntaryExit,
    ) -> anyhow::Result<()> {
        ReamBeaconState::process_voluntary_exit(&mut self.into(), signed_voluntary_exit)
    }

    pub fn process_withdrawals(self, payload: &ExecutionPayload) -> anyhow::Result<()> {
        ReamBeaconState::process_withdrawals(&mut self.into(), payload)
    }
}

impl From<ReamBeaconState> for BeaconState {
    fn from(state: ReamBeaconState) -> Self {
        BeaconState {
            // Versioning
            genesis_time: state.genesis_time,
            genesis_validators_root: state.genesis_validators_root,
            slot: state.slot,
            fork: state.fork,

            // History
            latest_block_header: state.latest_block_header,
            block_roots: state.block_roots,
            state_roots: state.state_roots,
            // Frozen in Capella, replaced by historical_summaries
            historical_roots: state.historical_roots,

            // Eth1
            eth1_data: state.eth1_data,
            eth1_data_votes: state.eth1_data_votes,
            eth1_deposit_index: state.eth1_deposit_index,

            // Registry
            validators: VariableList::<Validator, U536870912>::new(state.validators.to_vec())
                .unwrap(),
            balances: VariableList::<u64, U536870912>::new(state.balances.to_vec()).unwrap(),

            // Randomness
            randao_mixes: state.randao_mixes,

            // Slashings
            slashings: state.slashings,

            // // Participation
            previous_epoch_participation: VariableList::<u8, U536870912>::new(
                state.previous_epoch_participation.to_vec(),
            )
            .unwrap(),
            current_epoch_participation: VariableList::<u8, U536870912>::new(
                state.current_epoch_participation.to_vec(),
            )
            .unwrap(),

            // Finality
            justification_bits: state.justification_bits,
            previous_justified_checkpoint: state.previous_justified_checkpoint,
            current_justified_checkpoint: state.current_justified_checkpoint,
            finalized_checkpoint: state.finalized_checkpoint,

            // // Inactivity
            inactivity_scores: VariableList::<u64, U536870912>::new(
                state.inactivity_scores.to_vec(),
            )
            .unwrap(),

            // Sync
            current_sync_committee: state.current_sync_committee,
            next_sync_committee: state.next_sync_committee,

            // Execution
            latest_execution_payload_header: state.latest_execution_payload_header,

            // Withdrawals
            next_withdrawal_index: state.next_withdrawal_index,
            next_withdrawal_validator_index: state.next_withdrawal_validator_index,

            // Deep history valid from Capella onwards.
            historical_summaries: state.historical_summaries,
        }
    }
}

impl From<BeaconState> for ReamBeaconState {
    fn from(state: BeaconState) -> Self {
        ReamBeaconState {
            // Versioning
            genesis_time: state.genesis_time,
            genesis_validators_root: state.genesis_validators_root,
            slot: state.slot,
            fork: state.fork,

            // History
            latest_block_header: state.latest_block_header,
            block_roots: state.block_roots,
            state_roots: state.state_roots,
            // Frozen in Capella, replaced by historical_summaries
            historical_roots: state.historical_roots,

            // Eth1
            eth1_data: state.eth1_data,
            eth1_data_votes: state.eth1_data_votes,
            eth1_deposit_index: state.eth1_deposit_index,

            // // Registry
            validators: VariableList::default(),
            balances: VariableList::default(),

            // Randomness
            randao_mixes: state.randao_mixes,

            // Slashings
            slashings: state.slashings,

            // // Participation
            previous_epoch_participation: VariableList::default(),
            current_epoch_participation: VariableList::default(),

            // Finality
            justification_bits: state.justification_bits,
            previous_justified_checkpoint: state.previous_justified_checkpoint,
            current_justified_checkpoint: state.current_justified_checkpoint,
            finalized_checkpoint: state.finalized_checkpoint,

            // // Inactivity
            inactivity_scores: VariableList::default(),

            // Sync
            current_sync_committee: state.current_sync_committee,
            next_sync_committee: state.next_sync_committee,

            // Execution
            latest_execution_payload_header: state.latest_execution_payload_header,

            // Withdrawals
            next_withdrawal_index: state.next_withdrawal_index,
            next_withdrawal_validator_index: state.next_withdrawal_validator_index,

            // Deep history valid from Capella onwards.
            historical_summaries: state.historical_summaries,
        }
    }
}
