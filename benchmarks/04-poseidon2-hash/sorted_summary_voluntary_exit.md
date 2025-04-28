| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
voluntary_exit | basic | 264367 | 18696661 | 11328 | 2069183 | 32739323602 | 1111 | 32760370489 | 638.3658955s |
voluntary_exit | default_exit_epoch_subsequent_exit | 264367 | 18696661 | 11328 | 2069183 | 32739322571 | 1111 | 32760369458 | 699.715048s |
voluntary_exit | invalid_incorrect_signature | 264367 | 18696661 | 11328 | 2069183 | 32739323602 | 1111 | 32760370489 | 644.503697875s |
voluntary_exit | invalid_validator_already_exited | 264367 | 18696661 | 11328 | 2069183 | 32739321550 | 1111 | 32760368437 | 646.749377458s |
voluntary_exit | invalid_validator_exit_in_future | 264367 | 18696661 | 11328 | 2069183 | 32739323602 | 1111 | 32760370489 | 658.469302166s |
voluntary_exit | invalid_validator_incorrect_validator_index | 264367 | 18696661 | 11328 | 2069183 | 32739323602 | 1111 | 32760370489 | 661.694951584s |
voluntary_exit | invalid_validator_not_active | 264367 | 18696661 | 11328 | 2069183 | 32739320643 | 1111 | 32760367530 | 665.623669041s |
voluntary_exit | invalid_validator_not_active_long_enough | 264367 | 18696661 | 11328 | 2069183 | 32739319588 | 1111 | 32760366475 | 636.220335833s |
voluntary_exit | invalid_voluntary_exit_with_current_fork_version_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 32739462694 | 1111 | 32760509581 | 651.44556225s |
voluntary_exit | invalid_voluntary_exit_with_current_fork_version_not_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 32739462694 | 1111 | 32760509581 | 662.946970292s |
voluntary_exit | invalid_voluntary_exit_with_genesis_fork_version_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 32739462694 | 1111 | 32760509581 | 643.525671041s |
voluntary_exit | invalid_voluntary_exit_with_genesis_fork_version_not_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 32739462694 | 1111 | 32760509581 | 646.434175625s |
voluntary_exit | success_exit_queue__min_churn | 264367 | 18696661 | 11328 | 2069183 | 32739316866 | 1111 | 32760363753 | 644.869609709s |
voluntary_exit | voluntary_exit_with_previous_fork_version_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 32739462694 | 1111 | 32760509581 | 657.441499542s |
voluntary_exit | voluntary_exit_with_previous_fork_version_not_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 32739462694 | 1111 | 32760509581 | 651.727058833s |
