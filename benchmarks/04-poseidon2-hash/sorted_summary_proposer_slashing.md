| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
proposer_slashing | basic | 264367 | 18696661 | 39296 | 2074834 | 32739319588 | 1111 | 32760400094 | 625.023075459s |
proposer_slashing | block_header_from_future | 264367 | 18696661 | 39296 | 2074834 | 32739319588 | 1111 | 32760400094 | 621.262492459s |
proposer_slashing | invalid_different_proposer_indices | 264367 | 18696661 | 39296 | 2074813 | 32739319588 | 1111 | 32760400073 | 632.243296666s |
proposer_slashing | invalid_headers_are_same_sigs_are_different | 264367 | 18696661 | 39296 | 2075534 | 32739319588 | 1111 | 32760400794 | 619.750124292s |
proposer_slashing | invalid_headers_are_same_sigs_are_same | 264367 | 18696661 | 39296 | 2075534 | 32739319588 | 1111 | 32760400794 | 629.831216833s |
proposer_slashing | invalid_incorrect_proposer_index | 264367 | 18696661 | 39296 | 2074834 | 32739319588 | 1111 | 32760400094 | 644.338601834s |
proposer_slashing | invalid_incorrect_sig_1 | 264367 | 18696661 | 39296 | 2074834 | 32739319588 | 1111 | 32760400094 | 645.197146917s |
proposer_slashing | invalid_incorrect_sig_1_and_2 | 264367 | 18696661 | 39296 | 2074834 | 32739319588 | 1111 | 32760400094 | 617.346347333s |
proposer_slashing | invalid_incorrect_sig_1_and_2_swap | 264367 | 18696661 | 39296 | 2074834 | 32739319588 | 1111 | 32760400094 | 654.626609834s |
proposer_slashing | invalid_incorrect_sig_2 | 264367 | 18696661 | 39296 | 2074834 | 32739319588 | 1111 | 32760400094 | 615.081425625s |
proposer_slashing | invalid_proposer_is_not_activated | 264367 | 18696661 | 39296 | 2074834 | 32739322013 | 1111 | 32760402519 | 663.917958208s |
proposer_slashing | invalid_proposer_is_slashed | 264367 | 18696663 | 39296 | 2074834 | 32739324012 | 1111 | 32760404520 | 613.334733875s |
proposer_slashing | invalid_proposer_is_withdrawn | 264367 | 18696661 | 39296 | 2074834 | 32739459732 | 1111 | 32760540238 | 608.999541125s |
proposer_slashing | invalid_slots_of_different_epochs | 264367 | 18696661 | 39296 | 2074804 | 32739319588 | 1111 | 32760400064 | 606.896210875s |
proposer_slashing | slashed_and_proposer_index_the_same | 264367 | 18696661 | 39296 | 2074834 | 32739319588 | 1111 | 32760400094 | 623.829820125s |
