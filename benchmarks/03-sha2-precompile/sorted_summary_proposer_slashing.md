| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
proposer_slashing | basic | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.638426792s |
proposer_slashing | block_header_from_future | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.379886334s |
proposer_slashing | invalid_different_proposer_indices | 264367 | 18696661 | 39296 | 2074813 | 237557349 | 1012 | 258637636 | 6.45457375s |
proposer_slashing | invalid_headers_are_same_sigs_are_different | 264367 | 18696661 | 39296 | 2075534 | 237557349 | 1012 | 258638357 | 7.168071792s |
proposer_slashing | invalid_headers_are_same_sigs_are_same | 264367 | 18696661 | 39296 | 2075534 | 237557349 | 1012 | 258638357 | 6.420814125s |
proposer_slashing | invalid_incorrect_proposer_index | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.785445959s |
proposer_slashing | invalid_incorrect_sig_1 | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.4592335s |
proposer_slashing | invalid_incorrect_sig_1_and_2 | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.374093667s |
proposer_slashing | invalid_incorrect_sig_1_and_2_swap | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.511987292s |
proposer_slashing | invalid_incorrect_sig_2 | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.425079667s |
proposer_slashing | invalid_proposer_is_not_activated | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.354021417s |
proposer_slashing | invalid_proposer_is_slashed | 264367 | 18696663 | 39296 | 2074834 | 237557349 | 1012 | 258637659 | 6.454848667s |
proposer_slashing | invalid_proposer_is_withdrawn | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.675543375s |
proposer_slashing | invalid_slots_of_different_epochs | 264367 | 18696661 | 39296 | 2074804 | 237557349 | 1012 | 258637627 | 6.38515725s |
proposer_slashing | slashed_and_proposer_index_the_same | 264367 | 18696661 | 39296 | 2074834 | 237557349 | 1012 | 258637657 | 6.574142083s |
