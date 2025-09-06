# consenzero-bench

`consenzero-bench` is the benchmark of Ethereum consensus' state transition functions by using [ream](https://github.com/ReamLabs/ream) within [RISC Zero zkVM](https://github.com/succinctlabs/sp1).

## Requirements

- [Rust](https://rustup.rs/)
- [RiscZero](https://dev.risczero.com/api/getting-started)

## Running the Project

### Generate benchmarks

First, download the required test data:

```sh
cd host
make download
```

Then, run benchmarks for specific operations:

#### Block Operations
Run a specific block operation:
```sh
make run-block-<OPERATION_NAME>
```

Available block operations:
- attestation
- attester_slashing
- block_header
- bls_to_execution_change
- deposit
- execution_payload (not implemented)
- proposer_slashing
- sync_aggregate
- voluntary_exit
- withdrawals (incompatible with BeaconState workaround) 
Run all block operations:
```sh
make block-all
```

#### Epoch Operations
Run a specific epoch processing operation:
```sh
make run-epoch-<OPERATION_NAME>
```

Available epoch operations:
- justification_and_finalization
- inactivity_updates
- rewards_and_penalties
- registry_updates
- slashings
- eth1_data_reset
- pending_deposits
- pending_consolidations
- effective_balance_updates
- slashings_reset
- randao_mixes_reset
- historical_summaries_update
- participation_flag_updates

Run all epoch operations:
```sh
make epoch-all
```

#### Run All Executable Operations
Run all operations that can be executed (excludes execution_payload and withdrawals from block operations):
```sh
make all
```

### Output
- Logs are saved in `./host/logs/`
- Benchmark summaries (including cycle counts) are generated in `./host/summaries/`
