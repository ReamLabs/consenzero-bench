#!/bin/bash

OPERATION=$1

LOG_FILE="logs/execution_$OPERATION.log"
OUTPUT_FILE="summaries/summary_$OPERATION.md"

# Table Header
echo '| Operation | Test Case | Read Pre-State | Read Operation | Process | Execution Time |' > $OUTPUT_FILE
echo '|-----------|-----------|----------------|----------------|---------|----------------|' >> $OUTPUT_FILE

awk '
BEGIN {
    op = "";
    test_case = "";
    read_pre_state_start = 0;
    read_pre_state_end = 0;
    read_operation_input_start = 0;
    read_operation_input_end = 0;
    process_operation_start = 0;
    process_operation_end = 0;
    execution_time = 0;
}

/\[.*\] Test case:/ {
    op = $4;
    gsub(/[\[\]]/, "", op)
    test_case = $NF;
}

/read-pre-state:start:/ {
    read_pre_state_start = $NF;
}

/read-pre-state:end:/ {
    read_pre_state_end = $NF;
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

/execution time:/ {
    execution_time = $NF;
}

/----- Cycle Tracker End -----/ {
    printf "%s | %s | %d | %d | %d | %s |\n", op, test_case, read_pre_state_end-read_pre_state_start, read_operation_input_end-read_operation_input_start, process_operation_end-process_operation_start, execution_time >> "'$OUTPUT_FILE'"

    # Re-initialize for next log
    op = "";
    test_case = "";
    read_pre_state_start = 0;
    read_pre_state_end = 0;
    read_operation_input_start = 0;
    read_operation_input_end = 0;
    process_operation_start = 0;
    process_operation_end = 0;
    execution_time = 0;
}
' $LOG_FILE
