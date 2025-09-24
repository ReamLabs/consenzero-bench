use clap::{Parser, Subcommand, ValueEnum};
use derive_more::Display;
use ream_lib::{
    file::ssz_from_file,
    input::{
        BlockOperationType, BlockOperationWrapper, EpochOperationType, EpochOperationWrapper,
        OperationInput,
    },
};
use std::path::PathBuf;
#[derive(Debug, Clone, Parser)]
pub struct OperationArgs {
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    Block {
        #[clap(value_enum)]
        operation: BlockOperation,
    },
    Epoch {
        #[clap(value_enum)]
        operation: EpochOperation,
    },
}

#[derive(ValueEnum, Debug, Clone, Display)]
#[clap(rename_all = "snake_case")]
pub enum BlockOperation {
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
}

#[derive(ValueEnum, Debug, Clone, Display)]
#[clap(rename_all = "snake_case")]
pub enum EpochOperation {
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

// Generic traits for operation handling
pub trait OperationHandler: std::fmt::Display {
    fn prepare_input(&self, case_dir: &PathBuf) -> ream_lib::input::OperationInput;
    fn load_test_cases(&self, fork: &crate::cli::fork::Fork) -> (PathBuf, Vec<String>);
    fn get_operation_category(&self) -> &'static str;
}

// Block operation trait implementation
impl OperationHandler for BlockOperation {
    fn prepare_input(&self, case_dir: &PathBuf) -> ream_lib::input::OperationInput {
        let input_path = case_dir.join(format!("{}.ssz_snappy", self.get_input_filename()));
        let ssz_bytes = ssz_from_file(&input_path);

        OperationInput::Block(BlockOperationWrapper {
            operation_type: BlockOperationType::from(self.clone()),
            ssz_bytes,
        })
    }

    fn load_test_cases(&self, fork: &crate::cli::fork::Fork) -> (PathBuf, Vec<String>) {
        let test_case_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("mainnet")
            .join("tests")
            .join("mainnet");

        let base_dir = test_case_dir
            .join(format!("{}", fork))
            .join(self.get_operation_category())
            .join(self.to_string())
            .join("pyspec_tests");

        let test_cases = ream_lib::file::get_test_cases(&base_dir);
        (base_dir, test_cases)
    }

    fn get_operation_category(&self) -> &'static str {
        "operations"
    }
}

// Epoch operation trait implementation
impl OperationHandler for EpochOperation {
    fn prepare_input(&self, _case_dir: &PathBuf) -> ream_lib::input::OperationInput {
        OperationInput::Epoch(EpochOperationWrapper {
            operation_type: EpochOperationType::from(self.clone()),
        })
    }

    fn load_test_cases(&self, fork: &crate::cli::fork::Fork) -> (PathBuf, Vec<String>) {
        let test_case_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("mainnet")
            .join("tests")
            .join("mainnet");

        let base_dir = test_case_dir
            .join(format!("{}", fork))
            .join(self.get_operation_category())
            .join(self.to_string())
            .join("pyspec_tests");

        let test_cases = ream_lib::file::get_test_cases(&base_dir);
        (base_dir, test_cases)
    }

    fn get_operation_category(&self) -> &'static str {
        "epoch_processing"
    }
}

// Block operation specific methods
impl BlockOperation {
    fn get_input_filename(&self) -> &'static str {
        match self {
            BlockOperation::Attestation => "attestation",
            BlockOperation::AttesterSlashing => "attester_slashing",
            BlockOperation::BlockHeader => "block",
            BlockOperation::BLSToExecutionChange => "address_change",
            BlockOperation::Deposit => "deposit",
            BlockOperation::ExecutionPayload => "body",
            BlockOperation::ProposerSlashing => "proposer_slashing",
            BlockOperation::SyncAggregate => "sync_aggregate",
            BlockOperation::VoluntaryExit => "voluntary_exit",
            BlockOperation::Withdrawals => "execution_payload",
        }
    }
}

// Convert BlockOperation to BlockOperationType using From trait
impl From<BlockOperation> for BlockOperationType {
    fn from(operation: BlockOperation) -> Self {
        match operation {
            BlockOperation::Attestation => BlockOperationType::Attestation,
            BlockOperation::AttesterSlashing => BlockOperationType::AttesterSlashing,
            BlockOperation::BlockHeader => BlockOperationType::BlockHeader,
            BlockOperation::BLSToExecutionChange => BlockOperationType::BLSToExecutionChange,
            BlockOperation::Deposit => BlockOperationType::Deposit,
            BlockOperation::ExecutionPayload => BlockOperationType::ExecutionPayload,
            BlockOperation::ProposerSlashing => BlockOperationType::ProposerSlashing,
            BlockOperation::SyncAggregate => BlockOperationType::SyncAggregate,
            BlockOperation::VoluntaryExit => BlockOperationType::VoluntaryExit,
            BlockOperation::Withdrawals => BlockOperationType::Withdrawals,
        }
    }
}

// Convert EpochOperation to EpochOperationType using From trait
impl From<EpochOperation> for EpochOperationType {
    fn from(operation: EpochOperation) -> Self {
        match operation {
            EpochOperation::JustificationAndFinalization => {
                EpochOperationType::JustificationAndFinalization
            }
            EpochOperation::InactivityUpdates => EpochOperationType::InactivityUpdates,
            EpochOperation::RewardsAndPenalties => EpochOperationType::RewardsAndPenalties,
            EpochOperation::RegistryUpdates => EpochOperationType::RegistryUpdates,
            EpochOperation::Slashings => EpochOperationType::Slashings,
            EpochOperation::Eth1DataReset => EpochOperationType::Eth1DataReset,
            EpochOperation::PendingDeposits => EpochOperationType::PendingDeposits,
            EpochOperation::PendingConsolidations => EpochOperationType::PendingConsolidations,
            EpochOperation::EffectiveBalanceUpdates => EpochOperationType::EffectiveBalanceUpdates,
            EpochOperation::SlashingsReset => EpochOperationType::SlashingsReset,
            EpochOperation::RandaoMixesReset => EpochOperationType::RandaoMixesReset,
            EpochOperation::HistoricalSummariesUpdate => {
                EpochOperationType::HistoricalSummariesUpdate
            }
            EpochOperation::ParticipationFlagUpdates => {
                EpochOperationType::ParticipationFlagUpdates
            }
        }
    }
}
