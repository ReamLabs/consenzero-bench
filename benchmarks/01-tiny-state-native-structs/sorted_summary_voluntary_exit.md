| Operation | Test Case | Read Pre-State | Read Operation | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|----------------|----------------|---------|-----------|--------|--------------|----------------|
voluntary_exit | basic | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.33522075s |
voluntary_exit | default_exit_epoch_subsequent_exit | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.310506584s |
voluntary_exit | invalid_incorrect_signature | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 20.981767084s |
voluntary_exit | invalid_validator_already_exited | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.547303166s |
voluntary_exit | invalid_validator_exit_in_future | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.511602709s |
voluntary_exit | invalid_validator_incorrect_validator_index | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.641980166s |
voluntary_exit | invalid_validator_not_active | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.12842925s |
voluntary_exit | invalid_validator_not_active_long_enough | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.341232584s |
voluntary_exit | invalid_voluntary_exit_with_current_fork_version_is_before_fork_epoch | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.684668875s |
voluntary_exit | invalid_voluntary_exit_with_current_fork_version_not_is_before_fork_epoch | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 22.05674275s |
voluntary_exit | invalid_voluntary_exit_with_genesis_fork_version_is_before_fork_epoch | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.970330833s |
voluntary_exit | invalid_voluntary_exit_with_genesis_fork_version_not_is_before_fork_epoch | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 22.193422958s |
voluntary_exit | success_exit_queue__min_churn | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.3201755s |
voluntary_exit | voluntary_exit_with_previous_fork_version_is_before_fork_epoch | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.497968s |
voluntary_exit | voluntary_exit_with_previous_fork_version_not_is_before_fork_epoch | 46949445 | 4077 | 2063266 | 902102375 | 1013 | 951123396 | 21.486320291s |
