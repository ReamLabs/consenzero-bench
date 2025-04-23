| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
deposit | correct_sig_but_forked_state | 264367 | 18696661 | 115104 | 225476889 | 901681422 | 1015 | 1146239649 | 24.844637709s |
deposit | effective_deposit_with_genesis_fork_version | 264367 | 18696661 | 115104 | 225476889 | 901681422 | 1015 | 1146239649 | 25.130453459s |
deposit | incorrect_sig_new_deposit | 264367 | 18696661 | 115104 | 144294343 | 901681422 | 1015 | 1065057103 | 23.108466667s |
deposit | incorrect_sig_top_up | 264367 | 18696661 | 115104 | 144294339 | 901681422 | 1015 | 1065057099 | 23.282466s |
deposit | incorrect_withdrawal_credentials_top_up | 264367 | 18696661 | 115104 | 144294340 | 901681422 | 1015 | 1065057100 | 23.381752458s |
deposit | ineffective_deposit_with_bad_fork_version | 264367 | 18696661 | 115104 | 225474845 | 901681422 | 1015 | 1146237605 | 24.839713083s |
deposit | ineffective_deposit_with_current_fork_version | 264367 | 18696661 | 115104 | 225474840 | 901681422 | 1015 | 1146237600 | 25.065978417s |
deposit | ineffective_deposit_with_previous_fork_version | 264367 | 18696661 | 115104 | 225474841 | 901681422 | 1015 | 1146237601 | 24.770974708s |
deposit | invalid_bad_merkle_proof | 264367 | 18696661 | 115104 | 2899276 | 901681372 | 1012 | 923661930 | 20.054657459s |
deposit | invalid_wrong_deposit_for_deposit_count | 264367 | 18696661 | 115104 | 2899273 | 901681372 | 1012 | 923661927 | 20.014518916s |
deposit | key_validate_invalid_decompression | 264367 | 18696661 | 115104 | 42425399 | 901681372 | 1012 | 963188053 | 20.819755458s |
deposit | key_validate_invalid_subgroup | 264367 | 18696661 | 115104 | 42425139 | 901681372 | 1012 | 963187793 | 20.59141625s |
deposit | new_deposit_eth1_withdrawal_credentials | 264367 | 18696661 | 115104 | 225476900 | 901681422 | 1015 | 1146239660 | 25.029690625s |
deposit | new_deposit_max | 264367 | 18696661 | 115104 | 225476889 | 901681422 | 1015 | 1146239649 | 25.125506167s |
deposit | new_deposit_non_versioned_withdrawal_credentials | 264367 | 18696661 | 115104 | 225476893 | 901681422 | 1015 | 1146239653 | 24.722752917s |
deposit | new_deposit_over_max | 264367 | 18696661 | 115104 | 225476897 | 901681422 | 1015 | 1146239657 | 25.05964075s |
deposit | new_deposit_under_max | 264367 | 18696661 | 115104 | 225476916 | 901681422 | 1015 | 1146239676 | 25.024176375s |
deposit | success_top_up_to_withdrawn_validator | 264376 | 18697769 | 115104 | 225477172 | 901745462 | 1015 | 1146305089 | 25.291873083s |
deposit | top_up__less_effective_balance | 264367 | 18696661 | 115104 | 225476885 | 901681422 | 1015 | 1146239645 | 24.857697875s |
deposit | top_up__max_effective_balance | 264367 | 18696661 | 115104 | 225476885 | 901681422 | 1015 | 1146239645 | 24.978715583s |
deposit | top_up__zero_balance | 264367 | 18696661 | 115104 | 225476885 | 901681422 | 1015 | 1146239645 | 24.912025708s |
