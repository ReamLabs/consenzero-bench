| Operation | Test Case | Read Pre-State | Read Operation | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|----------------|----------------|---------|-----------|--------|--------------|----------------|
proposer_slashing | basic | 46949445 | 12061 | 2063517 | 902102375 | 1013 | 951131631 | 21.503641042s |
proposer_slashing | block_header_from_future | 46949445 | 12061 | 2063517 | 902102375 | 1013 | 951131631 | 21.968104833s |
proposer_slashing | invalid_different_proposer_indices | 46949445 | 12061 | 2063496 | 902102375 | 1013 | 951131610 | 21.802385917s |
proposer_slashing | invalid_headers_are_same_sigs_are_different | 46949445 | 12061 | 2064217 | 902102375 | 1013 | 951132331 | 21.083256125s |
proposer_slashing | invalid_headers_are_same_sigs_are_same | 46949445 | 12061 | 2064217 | 902102375 | 1013 | 951132331 | 22.201083625s |
proposer_slashing | invalid_incorrect_proposer_index | 46949445 | 12061 | 2063517 | 902102375 | 1013 | 951131631 | 21.032088292s |
proposer_slashing | invalid_incorrect_sig_1 | 46949445 | 12064 | 2063517 | 902102375 | 1013 | 951131634 | 21.429993458s |
proposer_slashing | invalid_incorrect_sig_1_and_2 | 46949445 | 12067 | 2063517 | 902102375 | 1013 | 951131637 | 21.610915291s |
proposer_slashing | invalid_incorrect_sig_1_and_2_swap | 46949445 | 12061 | 2063517 | 902102375 | 1013 | 951131631 | 21.180215083s |
proposer_slashing | invalid_incorrect_sig_2 | 46949445 | 12064 | 2063517 | 902102375 | 1013 | 951131634 | 21.61398s |
proposer_slashing | invalid_proposer_is_not_activated | 46949445 | 12061 | 2063517 | 902102375 | 1013 | 951131631 | 21.573017s |
proposer_slashing | invalid_proposer_is_slashed | 46949446 | 12061 | 2063517 | 902102375 | 1013 | 951131632 | 21.186657167s |
proposer_slashing | invalid_proposer_is_withdrawn | 46949445 | 12061 | 2063517 | 902102375 | 1013 | 951131631 | 21.592727375s |
proposer_slashing | invalid_slots_of_different_epochs | 46949445 | 12061 | 2063487 | 902102375 | 1013 | 951131601 | 21.879408708s |
proposer_slashing | slashed_and_proposer_index_the_same | 46949445 | 12061 | 2063517 | 902102375 | 1013 | 951131631 | 21.194662375s |
