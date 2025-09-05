use ream_consensus::electra::beacon_state::BeaconState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum OperationInput {
    Block(BlockOperationWrapper),
    Epoch(EpochOperationWrapper),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockOperationWrapper {
    pub operation_type: BlockOperationType,
    pub ssz_bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpochOperationWrapper {
    pub operation_type: EpochOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BlockOperationType {
    Attestation,
    AttesterSlashing,
    BlockHeader,
    BLSToExecutionChange,
    Deposit,
    ExecutionPayload,
    ProposerSlashing,
    SyncAggregate,
    VoluntaryExit,
    Withdrawals,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EpochOperationType {
    JustificationAndFinalization,
    InactivityUpdates,
    RewardsAndPenalties,
    RegistryUpdates,
    Slashings,
    Eth1DataReset,
    PendingDeposits,
    PendingConsolidations,
    EffectiveBalanceUpdates,
    SlashingsReset,
    RandaoMixesReset,
    HistoricalSummariesUpdate,
    ParticipationFlagUpdates,
}

impl OperationInput {
    /// Process the operation on the beacon state.
    pub fn process(&self, state: &mut BeaconState) {
        match self {
            OperationInput::Block(wrapper) => {
                let _ = wrapper.process_operation(state);
            }
            OperationInput::Epoch(wrapper) => {
                let _ = wrapper.process_operation(state);
            }
        }
    }
}

impl BlockOperationWrapper {
    pub fn process_operation(&self, state: &mut BeaconState) {
        use crate::ssz::from_ssz_bytes;

        match self.operation_type {
            BlockOperationType::Attestation => {
                let op: ream_consensus::attestation::Attestation =
                    from_ssz_bytes(&self.ssz_bytes).unwrap();
                let _ = state.process_attestation(&op);
            }
            BlockOperationType::AttesterSlashing => {
                let op: ream_consensus::attester_slashing::AttesterSlashing =
                    from_ssz_bytes(&self.ssz_bytes).unwrap();
                let _ = state.process_attester_slashing(&op);
            }
            BlockOperationType::BlockHeader => {
                let op: ream_consensus::electra::beacon_block::BeaconBlock =
                    from_ssz_bytes(&self.ssz_bytes).unwrap();
                let _ = state.process_block_header(&op);
            }
            BlockOperationType::BLSToExecutionChange => {
                let op: ream_consensus::bls_to_execution_change::SignedBLSToExecutionChange =
                    from_ssz_bytes(&self.ssz_bytes).unwrap();
                let _ = state.process_bls_to_execution_change(&op);
            }
            BlockOperationType::Deposit => {
                let op: ream_consensus::deposit::Deposit = from_ssz_bytes(&self.ssz_bytes).unwrap();
                let _ = state.process_deposit(&op);
            }
            BlockOperationType::ExecutionPayload => {
                panic!("Not implemented");
            }
            BlockOperationType::ProposerSlashing => {
                let op: ream_consensus::proposer_slashing::ProposerSlashing =
                    from_ssz_bytes(&self.ssz_bytes).unwrap();
                let _ = state.process_proposer_slashing(&op);
            }
            BlockOperationType::SyncAggregate => {
                let op: ream_consensus::sync_aggregate::SyncAggregate =
                    from_ssz_bytes(&self.ssz_bytes).unwrap();
                let _ = state.process_sync_aggregate(&op);
            }
            BlockOperationType::VoluntaryExit => {
                let op: ream_consensus::voluntary_exit::SignedVoluntaryExit =
                    from_ssz_bytes(&self.ssz_bytes).unwrap();
                let _ = state.process_voluntary_exit(&op);
            }
            BlockOperationType::Withdrawals => {
                let op: ream_consensus::electra::execution_payload::ExecutionPayload =
                    from_ssz_bytes(&self.ssz_bytes).unwrap();
                let _ = state.process_withdrawals(&op);
            }
        }
    }
}

impl EpochOperationWrapper {
    pub fn process_operation(&self, state: &mut BeaconState) {
        match self.operation_type {
            EpochOperationType::JustificationAndFinalization => {
                let _ = state.process_justification_and_finalization();
            }
            EpochOperationType::InactivityUpdates => {
                let _ = state.process_inactivity_updates();
            }
            EpochOperationType::RewardsAndPenalties => {
                let _ = state.process_rewards_and_penalties();
            }
            EpochOperationType::RegistryUpdates => {
                let _ = state.process_registry_updates();
            }
            EpochOperationType::Slashings => {
                let _ = state.process_slashings();
            }
            EpochOperationType::Eth1DataReset => {
                let _ = state.process_eth1_data_reset();
            }
            EpochOperationType::PendingDeposits => {
                let _ = state.process_pending_deposits();
            }
            EpochOperationType::PendingConsolidations => {
                let _ = state.process_pending_consolidations();
            }
            EpochOperationType::EffectiveBalanceUpdates => {
                let _ = state.process_effective_balance_updates();
            }
            EpochOperationType::SlashingsReset => {
                let _ = state.process_slashings_reset();
            }
            EpochOperationType::RandaoMixesReset => {
                let _ = state.process_randao_mixes_reset();
            }
            EpochOperationType::HistoricalSummariesUpdate => {
                let _ = state.process_historical_summaries_update();
            }
            EpochOperationType::ParticipationFlagUpdates => {
                let _ = state.process_participation_flag_updates();
            }
        }
    }
}
