#!/bin/bash

OPERATION=$1

LOG_FILE="logs/execution_$OPERATION.log"
OUTPUT_FILE="summaries/summary_$OPERATION.md"

# Table Header
echo '| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |' > $OUTPUT_FILE
echo '|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|' >> $OUTPUT_FILE

awk '
BEGIN {
    op = "";
    test_case = "";
    read_pre_state_ssz_start = 0;
    read_pre_state_ssz_end = 0;
    deserialize_pre_state_ssz_start = 0;
    deserialize_pre_state_ssz_end = 0;
    read_operation_input_start = 0;
    read_operation_input_end = 0;
    process_operation_start = 0;
    process_operation_end = 0;
    merkleize_operation_start = 0;
    merkleize_operation_end = 0;
    commit_start = 0;
    commit_end = 0;
    execution_time = 0;
}

/\[.*\] Test case:/ {
    op = $4;
    gsub(/[\[\]]/, "", op)
    test_case = $NF;
}

/read-pre-state-ssz:start:/ {
    read_pre_state_ssz_start = $NF;
}

/read-pre-state-ssz:end:/ {
    read_pre_state_ssz_end = $NF;
}

/deserialize-ream_consensus::electra::beacon_state::BeaconState:start:/ {
    deserialize_pre_state_ssz_start = $NF;
}

/deserialize-ream_consensus::electra::beacon_state::BeaconState:end:/ {
    deserialize_pre_state_ssz_end = $NF;
}

/read-operation-input:start:/ {
    read_operation_input_start = $NF;
}

/read-operation-input:end:/ {
    read_operation_input_end = $NF;
}

/process-operation:start:/ {
    process_operation_start = $NF;
}

/process-operation:end:/ {
    process_operation_end = $NF;
}

/merkleize-operation:start:/ {
    merkleize_operation_start = $NF;
}

/merkleize-operation:end:/ {
    merkleize_operation_end = $NF;
}

/commit:start:/ {
    commit_start = $NF;
}

/commit:end:/ {
    commit_end = $NF;
}

/execution time:/ {
    execution_time = $NF;
}

/----- Cycle Tracker End -----/ {
    printf "%s | %s | %d | %d | %d | %d | %d | %d | %d | %s |\n", op, test_case, read_pre_state_ssz_end-read_pre_state_ssz_start, deserialize_pre_state_ssz_end-deserialize_pre_state_ssz_start, read_operation_input_end-read_operation_input_start, process_operation_end-process_operation_start, merkleize_operation_end-merkleize_operation_start, commit_end-commit_start, commit_end, execution_time >> "'$OUTPUT_FILE'"

    # Re-initialize for next log
    op = "";
    test_case = "";
    read_pre_state_ssz_start = 0;
    read_pre_state_ssz_end = 0;
    deserialize_pre_state_ssz_start = 0;
    deserialize_pre_state_ssz_end = 0;
    read_operation_input_start = 0;
    read_operation_input_end = 0;
    process_operation_start = 0;
    process_operation_end = 0;
    merkleize_operation_start = 0;
    merkleize_operation_end = 0;
    commit_start = 0;
    commit_end = 0;
    execution_time = 0;
}
' $LOG_FILE
