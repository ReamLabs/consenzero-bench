TARGET = mainnet.tar.gz
EXTRACT_DIR = mainnet
LOGS_DIR = logs
SUMMARIES_DIR = summaries

DOWNLOAD_SCRIPT = ./subscripts/download_ef_data.sh
PARSE_SCRIPT = ./subscripts/parse_log_to_table.sh
SORT_SCRIPT = ./subscripts/sort_table.sh

OPERATIONS = attestation attester_slashing block_header bls_to_execution_change deposit execution_payload proposer_slashing sync_aggregate voluntary_exit withdrawals

RISC0_DEV_MODE = 1
RUST_BACKTRACE = 0

.PHONY: all download run clean $(addprefix run-, $(OPERATIONS))

all: download run-attestation run-attester_slashing run-block_header run-bls_to_execution_change run-deposit run-execution_payload run-proposer_slashing run-sync_aggregate run-voluntary_exit run-withdrawals

download:
	@echo "Running download script..."
	@chmod +x $(DOWNLOAD_SCRIPT)
	@$(DOWNLOAD_SCRIPT)

run:
	@echo "Specify an operation: $(OPERATIONS)"
	@exit 1

$(addprefix run-, $(OPERATIONS)): run-%: $(EXTRACT_DIR)
	@mkdir -p $(LOGS_DIR)
	@mkdir -p $(SUMMARIES_DIR)
	@echo "##############################"
	@echo "Running benchmarks for $*..."
	@echo "##############################"
	@NO_COLOR=1 RISC0_DEV_MODE=$(RISC0_DEV_MODE) RUST_BACKTRACE=$(RUST_BACKTRACE) \
		cargo run --release -- -o $* \
			--excluded-cases multi_proposer_index_iterations \
			--excluded-cases random_with_exits_with_duplicates \
			&> $(LOGS_DIR)/execution_$*.log
	@echo "Execution complete for $*."
	@$(PARSE_SCRIPT) $*
	@$(SORT_SCRIPT) $(SUMMARIES_DIR)/summary_$*.md

clean:
	@echo "Cleaning up downloaded/execution files..."
	@rm -f $(TARGET)
	@rm -rf $(EXTRACT_DIR)
	@rm -rf $(LOGS_DIR)
	@echo "Clean up complete."
