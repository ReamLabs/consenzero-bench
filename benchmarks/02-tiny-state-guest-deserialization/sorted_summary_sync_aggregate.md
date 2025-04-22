| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
sync_aggregate | invalid_signature_bad_domain | 264371 | 18697243 | 15744 | 3347489412 | 902134449 | 1015 | 4268606428 | 96.743061791s |
sync_aggregate | invalid_signature_extra_participant | 264371 | 18697243 | 15744 | 3347489412 | 902134449 | 1015 | 4268606428 | 95.851502375s |
sync_aggregate | invalid_signature_infinite_signature_with_all_participants | 264371 | 18697243 | 15744 | 3346098364 | 902134449 | 1015 | 4267215380 | 95.952109583s |
sync_aggregate | invalid_signature_infinite_signature_with_single_participant | 264371 | 18697243 | 15744 | 231139535 | 902134446 | 1015 | 1152256545 | 26.088414416s |
sync_aggregate | invalid_signature_missing_participant | 264371 | 18697243 | 15744 | 3335297797 | 902134449 | 1015 | 4256414813 | 95.981459334s |
sync_aggregate | invalid_signature_no_participants | 264371 | 18697243 | 15744 | 143974622 | 902134446 | 1015 | 1065091632 | 24.175140833s |
sync_aggregate | invalid_signature_past_block | 264351 | 18698295 | 15744 | 3347489640 | 902208090 | 1015 | 4268681329 | 94.650496542s |
sync_aggregate | random_all_but_one_participating_with_duplicates | 264371 | 18697243 | 15744 | 3347491108 | 902134449 | 1015 | 4268608124 | 95.433098583s |
sync_aggregate | random_high_participation_with_duplicates | 264371 | 18697243 | 15744 | 3152425349 | 902134449 | 1015 | 4073542365 | 92.295001542s |
sync_aggregate | random_low_participation_with_duplicates | 264371 | 18697243 | 15744 | 1652856431 | 902134449 | 1015 | 2573973447 | 57.216946416s |
sync_aggregate | random_misc_balances_and_half_participation_with_duplicates | 264371 | 18697243 | 15744 | 3249958243 | 902134449 | 1015 | 4171075259 | 95.728113042s |
sync_aggregate | random_only_one_participant_with_duplicates | 264371 | 18697243 | 15744 | 238628100 | 902134446 | 1015 | 1159745110 | 25.764885458s |
sync_aggregate | sync_committee_rewards_duplicate_committee_full_participation | 264371 | 18697243 | 15744 | 3347491108 | 902134449 | 1015 | 4268608124 | 96.836592834s |
sync_aggregate | sync_committee_rewards_duplicate_committee_half_participation | 264371 | 18697243 | 15744 | 1786964147 | 902134449 | 1015 | 2708081163 | 60.377957875s |
sync_aggregate | sync_committee_rewards_duplicate_committee_max_effective_balance_only_participate_first_one | 264371 | 18697243 | 15744 | 232532305 | 902134446 | 1015 | 1153649315 | 26.134315s |
sync_aggregate | sync_committee_rewards_duplicate_committee_max_effective_balance_only_participate_second_one | 264371 | 18697243 | 15744 | 232532305 | 902134446 | 1015 | 1153649315 | 25.32494725s |
sync_aggregate | sync_committee_rewards_duplicate_committee_no_participation | 264371 | 18697243 | 15744 | 2104489 | 902134396 | 1012 | 923221393 | 20.189801125s |
sync_aggregate | sync_committee_rewards_duplicate_committee_zero_balance_only_participate_first_one | 264371 | 18697243 | 15744 | 232532279 | 902134446 | 1015 | 1153649289 | 25.264866333s |
sync_aggregate | sync_committee_rewards_duplicate_committee_zero_balance_only_participate_second_one | 264371 | 18697243 | 15744 | 232532279 | 902134446 | 1015 | 1153649289 | 25.058878417s |
sync_aggregate | sync_committee_rewards_empty_participants | 264371 | 18697243 | 15744 | 2104489 | 902134396 | 1012 | 923221393 | 20.489710625s |
sync_aggregate | sync_committee_rewards_not_full_participants | 264371 | 18697243 | 15744 | 1829635670 | 902134449 | 1015 | 2750752686 | 61.987022959s |
sync_aggregate | sync_committee_with_nonparticipating_exited_member | 264356 | 18698821 | 15744 | 3335299853 | 902240107 | 1015 | 4256524090 | 95.391166583s |
sync_aggregate | sync_committee_with_nonparticipating_withdrawable_member | 264356 | 18698821 | 15744 | 3335299853 | 902240107 | 1015 | 4256524090 | 96.942018959s |
sync_aggregate | sync_committee_with_participating_exited_member | 264356 | 18698821 | 15744 | 3347491437 | 902240107 | 1015 | 4268715674 | 95.989765542s |
sync_aggregate | sync_committee_with_participating_withdrawable_member | 264356 | 18698821 | 15744 | 3347491437 | 902240107 | 1015 | 4268715674 | 94.684080208s |
