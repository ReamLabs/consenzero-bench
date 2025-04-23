| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
voluntary_exit | basic | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.355421042s |
voluntary_exit | default_exit_epoch_subsequent_exit | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.32578s |
voluntary_exit | invalid_incorrect_signature | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.360862708s |
voluntary_exit | invalid_validator_already_exited | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.36893525s |
voluntary_exit | invalid_validator_exit_in_future | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.380827792s |
voluntary_exit | invalid_validator_incorrect_validator_index | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.382127583s |
voluntary_exit | invalid_validator_not_active | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.341196s |
voluntary_exit | invalid_validator_not_active_long_enough | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.434448292s |
voluntary_exit | invalid_voluntary_exit_with_current_fork_version_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.371820667s |
voluntary_exit | invalid_voluntary_exit_with_current_fork_version_not_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.344041041s |
voluntary_exit | invalid_voluntary_exit_with_genesis_fork_version_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.317412792s |
voluntary_exit | invalid_voluntary_exit_with_genesis_fork_version_not_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.3718095s |
voluntary_exit | success_exit_queue__min_churn | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.415620334s |
voluntary_exit | voluntary_exit_with_previous_fork_version_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.408546459s |
voluntary_exit | voluntary_exit_with_previous_fork_version_not_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 237557349 | 1012 | 258604038 | 6.334561375s |
