| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
bls_to_execution_change | genesis_fork_version | 264367 | 18696661 | 16848 | 2072509 | 32739319588 | 1111 | 32760375321 | 661.39193975s |
bls_to_execution_change | invalid_already_0x01 | 264367 | 18696661 | 16848 | 2072576 | 32739320143 | 1111 | 32760375943 | 606.920295667s |
bls_to_execution_change | invalid_bad_signature | 264367 | 18696661 | 16848 | 2072509 | 32739319588 | 1111 | 32760375321 | 589.228854875s |
bls_to_execution_change | invalid_current_fork_version | 264367 | 18696661 | 16848 | 2072509 | 32739319588 | 1111 | 32760375321 | 600.919943167s |
bls_to_execution_change | invalid_genesis_validators_root | 264367 | 18696661 | 16848 | 2072509 | 32739319588 | 1111 | 32760375321 | 595.095485833s |
bls_to_execution_change | invalid_incorrect_from_bls_pubkey | 264367 | 18696661 | 16848 | 2072509 | 32739319588 | 1111 | 32760375321 | 604.250683333s |
bls_to_execution_change | invalid_previous_fork_version | 264367 | 18696661 | 16848 | 2072509 | 32739319588 | 1111 | 32760375321 | 608.282873583s |
bls_to_execution_change | invalid_val_index_out_of_range | 264367 | 18696661 | 16848 | 2072576 | 32739319588 | 1111 | 32760375388 | 594.045838s |
bls_to_execution_change | success | 264367 | 18696661 | 16848 | 2072509 | 32739319588 | 1111 | 32760375321 | 600.619021s |
bls_to_execution_change | success_exited | 264367 | 18696661 | 16848 | 2072509 | 32739321563 | 1111 | 32760377296 | 610.932087333s |
bls_to_execution_change | success_in_activation_queue | 264367 | 18696661 | 16848 | 2072509 | 32739327732 | 1111 | 32760383465 | 781.587731167s |
bls_to_execution_change | success_in_exit_queue | 264367 | 18696661 | 16848 | 2072509 | 32739323871 | 1111 | 32760379604 | 606.902397333s |
bls_to_execution_change | success_not_activated | 264367 | 18696661 | 16848 | 2072509 | 32739323111 | 1111 | 32760378844 | 610.300997417s |
bls_to_execution_change | success_withdrawable | 264367 | 18696661 | 16848 | 2072509 | 32739318979 | 1111 | 32760374712 | 616.002639583s |
