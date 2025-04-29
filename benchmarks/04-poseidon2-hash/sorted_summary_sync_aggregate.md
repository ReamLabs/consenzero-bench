| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
sync_aggregate | invalid_signature_bad_domain | 264371 | 18697243 | 15744 | 3349705352 | 32738871872 | 1134 | 36107560029 | 703.340604458s |
sync_aggregate | invalid_signature_extra_participant | 264371 | 18697243 | 15744 | 3349705352 | 32738871346 | 1134 | 36107559503 | 693.597455708s |
sync_aggregate | invalid_signature_infinite_signature_with_all_participants | 264371 | 18697243 | 15744 | 3348314304 | 32738873419 | 1134 | 36106170528 | 730.499367791s |
sync_aggregate | invalid_signature_infinite_signature_with_single_participant | 264371 | 18697243 | 15744 | 233355479 | 32738872119 | 1111 | 32991210354 | 657.021627166s |
sync_aggregate | invalid_signature_missing_participant | 264371 | 18697243 | 15744 | 3337513735 | 32738871191 | 1134 | 36095367731 | 663.995189208s |
sync_aggregate | invalid_signature_no_participants | 264371 | 18697243 | 15744 | 146190563 | 32738873101 | 1111 | 32904046420 | 644.822671541s |
sync_aggregate | invalid_signature_past_block | 264351 | 18698295 | 15744 | 3349706363 | 32741396148 | 1134 | 36110086348 | 694.024286875s |
sync_aggregate | random_all_but_one_participating_with_duplicates | 264371 | 18697243 | 15744 | 3349705349 | 32738873242 | 1134 | 36107561396 | 746.58031875s |
sync_aggregate | random_high_participation_with_duplicates | 264371 | 18697243 | 15744 | 3154639592 | 32738871674 | 1134 | 35912494071 | 683.771889375s |
sync_aggregate | random_low_participation_with_duplicates | 264371 | 18697243 | 15744 | 1655070668 | 32738873275 | 1111 | 34412926702 | 676.149814542s |
sync_aggregate | random_misc_balances_and_half_participation_with_duplicates | 264371 | 18697243 | 15744 | 3252173006 | 32738989149 | 1134 | 36010144960 | 689.455774917s |
sync_aggregate | random_only_one_participant_with_duplicates | 264371 | 18697243 | 15744 | 240842341 | 32738874041 | 1111 | 32998699138 | 662.019788334s |
sync_aggregate | sync_committee_rewards_duplicate_committee_full_participation | 264371 | 18697243 | 15744 | 3349705349 | 32738873242 | 1134 | 36107561396 | 624.899024834s |
sync_aggregate | sync_committee_rewards_duplicate_committee_half_participation | 264371 | 18697243 | 15744 | 1789178388 | 32738872378 | 1111 | 34547033525 | 670.26811825s |
sync_aggregate | sync_committee_rewards_duplicate_committee_max_effective_balance_only_participate_first_one | 264371 | 18697243 | 15744 | 234746537 | 32738874223 | 1111 | 32992603516 | 644.711070291s |
sync_aggregate | sync_committee_rewards_duplicate_committee_max_effective_balance_only_participate_second_one | 264371 | 18697243 | 15744 | 234746537 | 32738874103 | 1111 | 32992603396 | 656.041747042s |
sync_aggregate | sync_committee_rewards_duplicate_committee_no_participation | 264371 | 18697243 | 15744 | 4391565 | 32738873697 | 1111 | 32762247968 | 630.123026s |
sync_aggregate | sync_committee_rewards_duplicate_committee_zero_balance_only_participate_first_one | 264371 | 18697243 | 15744 | 234747496 | 32738859955 | 1111 | 32992590207 | 594.290707375s |
sync_aggregate | sync_committee_rewards_duplicate_committee_zero_balance_only_participate_second_one | 264371 | 18697243 | 15744 | 234747496 | 32738858825 | 1111 | 32992589077 | 599.247820958s |
sync_aggregate | sync_committee_rewards_empty_participants | 264371 | 18697243 | 15744 | 4391565 | 32738873697 | 1111 | 32762247968 | 694.48021525s |
sync_aggregate | sync_committee_rewards_not_full_participants | 264371 | 18697243 | 15744 | 1831849917 | 32738872900 | 1111 | 34589705576 | 669.774364125s |
sync_aggregate | sync_committee_with_nonparticipating_exited_member | 264356 | 18698821 | 15744 | 3337514927 | 32743015460 | 1134 | 36099514755 | 626.879340083s |
sync_aggregate | sync_committee_with_nonparticipating_withdrawable_member | 264356 | 18698821 | 15744 | 3337514927 | 32743177181 | 1134 | 36099676476 | 693.017715667s |
sync_aggregate | sync_committee_with_participating_exited_member | 264356 | 18698821 | 15744 | 3349706515 | 32743016535 | 1134 | 36111707418 | 627.948727834s |
sync_aggregate | sync_committee_with_participating_withdrawable_member | 264356 | 18698821 | 15744 | 3349706515 | 32743176265 | 1134 | 36111867148 | 619.545079042s |
