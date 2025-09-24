TARGET = mainnet.tar.gz
EXTRACT_DIR = mainnet
LOGS_DIR = logs
SUMMARIES_DIR = summaries

DOWNLOAD_SCRIPT = ./subscripts/download_ef_data.sh
PARSE_SCRIPT = ./subscripts/parse_log_to_table.sh
SORT_SCRIPT = ./subscripts/sort_table.sh

BLOCK_OPERATIONS = attestation attester_slashing block_header bls_to_execution_change deposit execution_payload proposer_slashing sync_aggregate voluntary_exit withdrawals
EPOCH_OPERATIONS = justification_and_finalization inactivity_updates rewards_and_penalties registry_updates slashings eth1_data_reset pending_deposits pending_consolidations effective_balance_updates slashings_reset randao_mixes_reset historical_summaries_update participation_flag_updates

RISC0_DEV_MODE = 1
RUST_BACKTRACE = full

.PHONY: all download run clean $(addprefix run-block-, $(BLOCK_OPERATIONS)) $(addprefix run-epoch-, $(EPOCH_OPERATIONS)) block-all epoch-all

# run-block-execution_payload (not implemented) and run-block-withdrawals (incompatible with BeaconState workaround) are excluded
all: download $(addprefix run-block-, $(filter-out execution_payload withdrawals, $(BLOCK_OPERATIONS))) $(addprefix run-epoch-, $(EPOCH_OPERATIONS))

# Run all block processing benchmarks
block-all: $(addprefix run-block-, $(BLOCK_OPERATIONS))

# Run all epoch processing benchmarks
epoch-all: $(addprefix run-epoch-, $(EPOCH_OPERATIONS))

download:
	@echo "Running download script..."
	@chmod +x $(DOWNLOAD_SCRIPT)
	@$(DOWNLOAD_SCRIPT)

run:
	@echo "Specify a block operation: $(BLOCK_OPERATIONS)"
	@echo "Specify an epoch operation: $(EPOCH_OPERATIONS)"
	@echo "Use: make run-block-<operation> or make run-epoch-<operation>"
	@exit 1

$(addprefix run-block-, $(BLOCK_OPERATIONS)): run-block-%: $(EXTRACT_DIR)
	@mkdir -p $(LOGS_DIR)
	@mkdir -p $(SUMMARIES_DIR)
	@echo "##################################################"
	@echo "Running block benchmarks for $*..."
	@echo "##################################################"
	@NO_COLOR=1 RISC0_DEV_MODE=$(RISC0_DEV_MODE) RUST_BACKTRACE=$(RUST_BACKTRACE) \
		cargo run --release -- \
			--excluded-cases multi_proposer_index_iterations \
			--excluded-cases random_with_exits_with_duplicates \
			block $* \
			2>&1 | tee $(LOGS_DIR)/execution_block_$*.log
	@echo "Execution complete for block $*."
	@$(PARSE_SCRIPT) block_$*
	@$(SORT_SCRIPT) $(SUMMARIES_DIR)/summary_block_$*.md

$(addprefix run-epoch-, $(EPOCH_OPERATIONS)): run-epoch-%: $(EXTRACT_DIR)
	@mkdir -p $(LOGS_DIR)
	@mkdir -p $(SUMMARIES_DIR)
	@echo "##################################################"
	@echo "Running epoch processing benchmarks for $*..."
	@echo "##################################################"
	@NO_COLOR=1 RISC0_DEV_MODE=$(RISC0_DEV_MODE) RUST_BACKTRACE=$(RUST_BACKTRACE) \
		cargo run --release -- \
			epoch $* \
			2>&1 | tee $(LOGS_DIR)/execution_epoch_$*.log
	@echo "Execution complete for epoch $*."
	@$(PARSE_SCRIPT) epoch_$*
	@$(SORT_SCRIPT) $(SUMMARIES_DIR)/summary_epoch_$*.md

clean:
	@echo "Cleaning up downloaded/execution files..."
	@rm -f $(TARGET)
	@rm -rf $(EXTRACT_DIR)
	@rm -rf $(LOGS_DIR)
	@echo "Clean up complete."
