| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
deposit | correct_sig_but_forked_state | 264367 | 18696661 | 115104 | 224701881 | 237495822 | 1012 | 481279035 | 11.386882625s |
deposit | effective_deposit_with_genesis_fork_version | 264367 | 18696661 | 115104 | 224701881 | 237495822 | 1012 | 481279035 | 11.974608333s |
deposit | incorrect_sig_new_deposit | 264367 | 18696661 | 115104 | 143519335 | 237495822 | 1012 | 400096489 | 9.798482083s |
deposit | incorrect_sig_top_up | 264367 | 18696661 | 115104 | 143519331 | 237495822 | 1012 | 400096485 | 9.468958041s |
deposit | incorrect_withdrawal_credentials_top_up | 264367 | 18696661 | 115104 | 143519332 | 237495822 | 1012 | 400096486 | 9.494817875s |
deposit | ineffective_deposit_with_bad_fork_version | 264367 | 18696661 | 115104 | 224699837 | 237495822 | 1012 | 481276991 | 11.397402541s |
deposit | ineffective_deposit_with_current_fork_version | 264367 | 18696661 | 115104 | 224699832 | 237495822 | 1012 | 481276986 | 11.843974625s |
deposit | ineffective_deposit_with_previous_fork_version | 264367 | 18696661 | 115104 | 224699833 | 237495822 | 1012 | 481276987 | 11.332107666s |
deposit | invalid_bad_merkle_proof | 264367 | 18696661 | 115104 | 2240332 | 237495772 | 1012 | 258817386 | 6.497760209s |
deposit | invalid_wrong_deposit_for_deposit_count | 264367 | 18696661 | 115104 | 2240329 | 237495772 | 1012 | 258817383 | 6.404089417s |
deposit | key_validate_invalid_decompression | 264367 | 18696661 | 115104 | 41650391 | 237495772 | 1012 | 298227445 | 11.812278958s |
deposit | key_validate_invalid_subgroup | 264367 | 18696661 | 115104 | 41650131 | 237495772 | 1012 | 298227185 | 7.252804042s |
deposit | new_deposit_eth1_withdrawal_credentials | 264367 | 18696661 | 115104 | 224701892 | 237495822 | 1012 | 481279046 | 11.534015083s |
deposit | new_deposit_max | 264367 | 18696661 | 115104 | 224701881 | 237495822 | 1012 | 481279035 | 11.756385584s |
deposit | new_deposit_non_versioned_withdrawal_credentials | 264367 | 18696661 | 115104 | 224701885 | 237495822 | 1012 | 481279039 | 11.688733s |
deposit | new_deposit_over_max | 264367 | 18696661 | 115104 | 224701889 | 237495822 | 1012 | 481279043 | 12.043976084s |
deposit | new_deposit_under_max | 264367 | 18696661 | 115104 | 224701908 | 237495822 | 1012 | 481279062 | 11.770155125s |
deposit | success_top_up_to_withdrawn_validator | 264376 | 18697769 | 115104 | 224702164 | 237514934 | 1012 | 481299547 | 11.312151625s |
deposit | top_up__less_effective_balance | 264367 | 18696661 | 115104 | 224701877 | 237495822 | 1012 | 481279031 | 11.455359792s |
deposit | top_up__max_effective_balance | 264367 | 18696661 | 115104 | 224701877 | 237495822 | 1012 | 481279031 | 11.3574125s |
deposit | top_up__zero_balance | 264367 | 18696661 | 115104 | 224701877 | 237495822 | 1012 | 481279031 | 11.323476708s |
