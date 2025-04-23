| Operation | Test Case | Read Pre-State | Read Operation | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|----------------|----------------|---------|-----------|--------|--------------|----------------|
deposit | correct_sig_but_forked_state | 46949445 | 23395 | 225462389 | 901681424 | 1016 | 1174120942 | 27.276786125s |
deposit | effective_deposit_with_genesis_fork_version | 46949445 | 23395 | 225462389 | 901681424 | 1016 | 1174120942 | 26.875900417s |
deposit | incorrect_sig_new_deposit | 46949445 | 23398 | 144279843 | 901681424 | 1016 | 1092938399 | 25.229244542s |
deposit | incorrect_sig_top_up | 46949445 | 23398 | 144279839 | 901681424 | 1016 | 1092938395 | 24.811698167s |
deposit | incorrect_withdrawal_credentials_top_up | 46949445 | 23398 | 144279840 | 901681424 | 1016 | 1092938396 | 25.22502s |
deposit | ineffective_deposit_with_bad_fork_version | 46949445 | 23395 | 225460345 | 901681424 | 1016 | 1174118898 | 27.572586084s |
deposit | ineffective_deposit_with_current_fork_version | 46949445 | 23395 | 225460340 | 901681424 | 1016 | 1174118893 | 26.794718583s |
deposit | ineffective_deposit_with_previous_fork_version | 46949445 | 23395 | 225460341 | 901681424 | 1016 | 1174118894 | 26.421791917s |
deposit | invalid_bad_merkle_proof | 46949445 | 23395 | 2884776 | 901681374 | 1013 | 951543223 | 21.729907667s |
deposit | invalid_wrong_deposit_for_deposit_count | 46949445 | 23395 | 2884773 | 901681374 | 1013 | 951543220 | 21.842717334s |
deposit | key_validate_invalid_decompression | 46949445 | 23395 | 42410899 | 901681374 | 1013 | 991069346 | 22.734327s |
deposit | key_validate_invalid_subgroup | 46949445 | 23398 | 42410639 | 901681374 | 1013 | 991069089 | 22.172943416s |
deposit | new_deposit_eth1_withdrawal_credentials | 46949445 | 23395 | 225462400 | 901681424 | 1016 | 1174120953 | 26.628118625s |
deposit | new_deposit_max | 46949445 | 23395 | 225462389 | 901681424 | 1016 | 1174120942 | 27.105776458s |
deposit | new_deposit_non_versioned_withdrawal_credentials | 46949445 | 23395 | 225462393 | 901681424 | 1016 | 1174120946 | 26.193545583s |
deposit | new_deposit_over_max | 46949445 | 23395 | 225462397 | 901681424 | 1016 | 1174120950 | 27.031295709s |
deposit | new_deposit_under_max | 46949445 | 23395 | 225462416 | 901681424 | 1016 | 1174120969 | 26.643293084s |
deposit | success_top_up_to_withdrawn_validator | 46952599 | 23395 | 225462672 | 901745464 | 1016 | 1174188419 | 26.575861084s |
deposit | top_up__less_effective_balance | 46949445 | 23395 | 225462385 | 901681424 | 1016 | 1174120938 | 26.670978958s |
deposit | top_up__max_effective_balance | 46949445 | 23395 | 225462385 | 901681424 | 1016 | 1174120938 | 26.478716958s |
deposit | top_up__zero_balance | 46949445 | 23395 | 225462385 | 901681424 | 1016 | 1174120938 | 26.977444542s |
