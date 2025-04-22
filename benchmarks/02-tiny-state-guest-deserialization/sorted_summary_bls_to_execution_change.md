| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
bls_to_execution_change | genesis_fork_version | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 19.820676958s |
bls_to_execution_change | invalid_already_0x01 | 264367 | 18696661 | 16848 | 2072576 | 902102373 | 1012 | 923157975 | 19.884290541s |
bls_to_execution_change | invalid_bad_signature | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 19.983190666s |
bls_to_execution_change | invalid_current_fork_version | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 19.923090833s |
bls_to_execution_change | invalid_genesis_validators_root | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 19.7894435s |
bls_to_execution_change | invalid_incorrect_from_bls_pubkey | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 20.06493175s |
bls_to_execution_change | invalid_previous_fork_version | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 20.15141525s |
bls_to_execution_change | invalid_val_index_out_of_range | 264367 | 18696661 | 16848 | 2072576 | 902102373 | 1012 | 923157975 | 20.223318s |
bls_to_execution_change | success | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 20.271619375s |
bls_to_execution_change | success_exited | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 19.843652125s |
bls_to_execution_change | success_in_activation_queue | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 20.014667625s |
bls_to_execution_change | success_in_exit_queue | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 19.833966959s |
bls_to_execution_change | success_not_activated | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 20.062973625s |
bls_to_execution_change | success_withdrawable | 264367 | 18696661 | 16848 | 2072509 | 902102373 | 1012 | 923157908 | 19.850944166s |
