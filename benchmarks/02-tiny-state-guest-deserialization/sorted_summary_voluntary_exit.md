| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
voluntary_exit | basic | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.489605291s |
voluntary_exit | default_exit_epoch_subsequent_exit | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.147377417s |
voluntary_exit | invalid_incorrect_signature | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.077684792s |
voluntary_exit | invalid_validator_already_exited | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 19.917552917s |
voluntary_exit | invalid_validator_exit_in_future | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.463996375s |
voluntary_exit | invalid_validator_incorrect_validator_index | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 19.948661625s |
voluntary_exit | invalid_validator_not_active | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 19.943563667s |
voluntary_exit | invalid_validator_not_active_long_enough | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.299958667s |
voluntary_exit | invalid_voluntary_exit_with_current_fork_version_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.065889667s |
voluntary_exit | invalid_voluntary_exit_with_current_fork_version_not_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.669697709s |
voluntary_exit | invalid_voluntary_exit_with_genesis_fork_version_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.476021s |
voluntary_exit | invalid_voluntary_exit_with_genesis_fork_version_not_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.259410875s |
voluntary_exit | success_exit_queue__min_churn | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.590021167s |
voluntary_exit | voluntary_exit_with_previous_fork_version_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.114296209s |
voluntary_exit | voluntary_exit_with_previous_fork_version_not_is_before_fork_epoch | 264367 | 18696661 | 11328 | 2069183 | 902102373 | 1012 | 923149062 | 20.487362333s |
