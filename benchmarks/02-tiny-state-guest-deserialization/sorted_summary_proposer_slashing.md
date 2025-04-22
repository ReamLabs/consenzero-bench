| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
proposer_slashing | basic | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 20.0263815s |
proposer_slashing | block_header_from_future | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 19.916860209s |
proposer_slashing | invalid_different_proposer_indices | 264367 | 18696661 | 39296 | 2074813 | 902102373 | 1012 | 923182660 | 19.9419255s |
proposer_slashing | invalid_headers_are_same_sigs_are_different | 264367 | 18696661 | 39296 | 2075534 | 902102373 | 1012 | 923183381 | 19.998303959s |
proposer_slashing | invalid_headers_are_same_sigs_are_same | 264367 | 18696661 | 39296 | 2075534 | 902102373 | 1012 | 923183381 | 19.759848459s |
proposer_slashing | invalid_incorrect_proposer_index | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 20.085364375s |
proposer_slashing | invalid_incorrect_sig_1 | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 20.083221292s |
proposer_slashing | invalid_incorrect_sig_1_and_2 | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 20.287910667s |
proposer_slashing | invalid_incorrect_sig_1_and_2_swap | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 20.1843075s |
proposer_slashing | invalid_incorrect_sig_2 | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 19.848813584s |
proposer_slashing | invalid_proposer_is_not_activated | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 20.115485375s |
proposer_slashing | invalid_proposer_is_slashed | 264367 | 18696663 | 39296 | 2074834 | 902102373 | 1012 | 923182683 | 19.955324125s |
proposer_slashing | invalid_proposer_is_withdrawn | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 20.114658958s |
proposer_slashing | invalid_slots_of_different_epochs | 264367 | 18696661 | 39296 | 2074804 | 902102373 | 1012 | 923182651 | 19.77137775s |
proposer_slashing | slashed_and_proposer_index_the_same | 264367 | 18696661 | 39296 | 2074834 | 902102373 | 1012 | 923182681 | 20.24930425s |
