use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum OperationInput {
    Attestation(Vec<u8>),
    AttesterSlashing(Vec<u8>),
    BeaconBlock(Vec<u8>),
    SignedBLSToExecutionChange(Vec<u8>),
    Deposit(Vec<u8>),
    BeaconBlockBody(Vec<u8>),
    ProposerSlashing(Vec<u8>),
    SyncAggregate(Vec<u8>),
    SignedVoluntaryExit(Vec<u8>),
    ExecutionPayload(Vec<u8>),
}
