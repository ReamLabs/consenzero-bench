| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
bls_to_execution_change | genesis_fork_version | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.334104458s |
bls_to_execution_change | invalid_already_0x01 | 264367 | 18696661 | 16848 | 2072576 | 237557349 | 1012 | 258612951 | 6.299453208s |
bls_to_execution_change | invalid_bad_signature | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.455162125s |
bls_to_execution_change | invalid_current_fork_version | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.314707542s |
bls_to_execution_change | invalid_genesis_validators_root | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.315341292s |
bls_to_execution_change | invalid_incorrect_from_bls_pubkey | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.446508459s |
bls_to_execution_change | invalid_previous_fork_version | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.325995167s |
bls_to_execution_change | invalid_val_index_out_of_range | 264367 | 18696661 | 16848 | 2072576 | 237557349 | 1012 | 258612951 | 6.347284917s |
bls_to_execution_change | success | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.326110541s |
bls_to_execution_change | success_exited | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.342245208s |
bls_to_execution_change | success_in_activation_queue | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.301486s |
bls_to_execution_change | success_in_exit_queue | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.414096625s |
bls_to_execution_change | success_not_activated | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.32696825s |
bls_to_execution_change | success_withdrawable | 264367 | 18696661 | 16848 | 2072509 | 237557349 | 1012 | 258612884 | 6.305732167s |
