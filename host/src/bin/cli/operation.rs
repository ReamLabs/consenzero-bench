use clap::{Parser, ValueEnum};
use derive_more::Display;
use ream_lib::input::EpochProcessingType;

#[derive(Debug, Clone, Parser)]
pub struct OperationArgs {
    #[clap(long, short)]
    pub operation_name: OperationName,
}

#[derive(ValueEnum, Debug, Clone, Display)]
#[clap(rename_all = "snake_case")]
pub enum OperationName {
    #[display("attestation")]
    Attestation,
    #[display("attester_slashing")]
    AttesterSlashing,
    #[display("block_header")]
    BlockHeader,
    #[display("bls_to_execution_change")]
    BLSToExecutionChange,
    #[display("deposit")]
    Deposit,
    #[display("execution_payload")]
    ExecutionPayload,
    #[display("proposer_slashing")]
    ProposerSlashing,
    #[display("sync_aggregate")]
    SyncAggregate,
    #[display("voluntary_exit")]
    VoluntaryExit,
    #[display("withdrawals")]
    Withdrawals,
    // Epoch processing operations
    #[display("justification_and_finalization")]
    JustificationAndFinalization,
    #[display("inactivity_updates")]
    InactivityUpdates,
    #[display("rewards_and_penalties")]
    RewardsAndPenalties,
    #[display("registry_updates")]
    RegistryUpdates,
    #[display("slashings")]
    Slashings,
    #[display("eth1_data_reset")]
    Eth1DataReset,
    #[display("pending_deposits")]
    PendingDeposits,
    #[display("pending_consolidations")]
    PendingConsolidations,
    #[display("effective_balance_updates")]
    EffectiveBalanceUpdates,
    #[display("slashings_reset")]
    SlashingsReset,
    #[display("randao_mixes_reset")]
    RandaoMixesReset,
    #[display("historical_summaries_update")]
    HistoricalSummariesUpdate,
    #[display("participation_flag_updates")]
    ParticipationFlagUpdates,
}

impl OperationName {
    pub fn to_input_name(&self) -> String {
        match self {
            OperationName::Attestation => "attestation".to_string(),
            OperationName::AttesterSlashing => "attester_slashing".to_string(),
            OperationName::BlockHeader => "block".to_string(),
            OperationName::BLSToExecutionChange => "address_change".to_string(),
            OperationName::Deposit => "deposit".to_string(),
            OperationName::ExecutionPayload => "body".to_string(),
            OperationName::ProposerSlashing => "proposer_slashing".to_string(),
            OperationName::SyncAggregate => "sync_aggregate".to_string(),
            OperationName::VoluntaryExit => "voluntary_exit".to_string(),
            OperationName::Withdrawals => "execution_payload".to_string(),
            // Epoch processing operations don't need input files
            OperationName::JustificationAndFinalization => "".to_string(),
            OperationName::InactivityUpdates => "".to_string(),
            OperationName::RewardsAndPenalties => "".to_string(),
            OperationName::RegistryUpdates => "".to_string(),
            OperationName::Slashings => "".to_string(),
            OperationName::Eth1DataReset => "".to_string(),
            OperationName::PendingDeposits => "".to_string(),
            OperationName::PendingConsolidations => "".to_string(),
            OperationName::EffectiveBalanceUpdates => "".to_string(),
            OperationName::SlashingsReset => "".to_string(),
            OperationName::RandaoMixesReset => "".to_string(),
            OperationName::HistoricalSummariesUpdate => "".to_string(),
            OperationName::ParticipationFlagUpdates => "".to_string(),
        }
    }

    pub fn is_epoch_processing(&self) -> bool {
        matches!(self,
            OperationName::JustificationAndFinalization |
            OperationName::InactivityUpdates |
            OperationName::RewardsAndPenalties |
            OperationName::RegistryUpdates |
            OperationName::Slashings |
            OperationName::Eth1DataReset |
            OperationName::PendingDeposits |
            OperationName::PendingConsolidations |
            OperationName::EffectiveBalanceUpdates |
            OperationName::SlashingsReset |
            OperationName::RandaoMixesReset |
            OperationName::HistoricalSummariesUpdate |
            OperationName::ParticipationFlagUpdates 
        )
    }

    pub fn to_epoch_processing_type(&self) -> Option<EpochProcessingType> {
        match self {
            OperationName::JustificationAndFinalization => Some(EpochProcessingType::JustificationAndFinalization),
            OperationName::InactivityUpdates => Some(EpochProcessingType::InactivityUpdates),
            OperationName::RewardsAndPenalties => Some(EpochProcessingType::RewardsAndPenalties),
            OperationName::RegistryUpdates => Some(EpochProcessingType::RegistryUpdates),
            OperationName::Slashings => Some(EpochProcessingType::Slashings),
            OperationName::Eth1DataReset => Some(EpochProcessingType::Eth1DataReset),
            OperationName::PendingDeposits => Some(EpochProcessingType::PendingDeposits),
            OperationName::PendingConsolidations => Some(EpochProcessingType::PendingConsolidations),
            OperationName::EffectiveBalanceUpdates => Some(EpochProcessingType::EffectiveBalanceUpdates),
            OperationName::SlashingsReset => Some(EpochProcessingType::SlashingsReset),
            OperationName::RandaoMixesReset => Some(EpochProcessingType::RandaoMixesReset),
            OperationName::HistoricalSummariesUpdate => Some(EpochProcessingType::HistoricalSummariesUpdate),
            OperationName::ParticipationFlagUpdates => Some(EpochProcessingType::ParticipationFlagUpdates),
            _ => None,
        }
    }
}
