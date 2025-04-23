| Operation | Test Case | Read Pre-State | Read Operation | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|----------------|----------------|---------|-----------|--------|--------------|----------------|
sync_aggregate | invalid_signature_bad_domain | 46951041 | 9156 | 3347483065 | 902134451 | 1081 | 4296582135 | 96.562556541s |
sync_aggregate | invalid_signature_extra_participant | 46951041 | 9156 | 3347483065 | 902134451 | 1081 | 4296582135 | 97.637566958s |
sync_aggregate | invalid_signature_infinite_signature_with_all_participants | 46951041 | 9156 | 3346092017 | 902134451 | 1081 | 4295191087 | 98.577910958s |
sync_aggregate | invalid_signature_infinite_signature_with_single_participant | 46951041 | 9412 | 231133188 | 902134448 | 1016 | 1180232378 | 26.609227792s |
sync_aggregate | invalid_signature_missing_participant | 46951041 | 9156 | 3335291450 | 902134451 | 1016 | 4284390390 | 97.009781958s |
sync_aggregate | invalid_signature_no_participants | 46951041 | 9415 | 143968275 | 902134448 | 1016 | 1093067468 | 25.080489959s |
sync_aggregate | invalid_signature_past_block | 46954157 | 9156 | 3347483293 | 902208092 | 1081 | 4296659120 | 96.319392792s |
sync_aggregate | random_all_but_one_participating_with_duplicates | 46951041 | 9156 | 3347484761 | 902134451 | 1081 | 4296583831 | 96.928115416s |
sync_aggregate | random_high_participation_with_duplicates | 46951041 | 9168 | 3152419002 | 902134451 | 1016 | 4101517954 | 94.153150125s |
sync_aggregate | random_low_participation_with_duplicates | 46951041 | 9332 | 1652850084 | 902134451 | 1016 | 2601949200 | 59.720043709s |
sync_aggregate | random_misc_balances_and_half_participation_with_duplicates | 46951041 | 9172 | 3249951896 | 902134451 | 1016 | 4199050852 | 95.030014792s |
sync_aggregate | random_only_one_participant_with_duplicates | 46951041 | 9412 | 238621753 | 902134448 | 1016 | 1187720943 | 27.212126416s |
sync_aggregate | sync_committee_rewards_duplicate_committee_full_participation | 46951041 | 9156 | 3347484761 | 902134451 | 1081 | 4296583831 | 99.778350458s |
sync_aggregate | sync_committee_rewards_duplicate_committee_half_participation | 46951041 | 9284 | 1786957800 | 902134451 | 1016 | 2736056868 | 61.069198083s |
sync_aggregate | sync_committee_rewards_duplicate_committee_max_effective_balance_only_participate_first_one | 46951041 | 9412 | 232525958 | 902134448 | 1016 | 1181625148 | 27.398276334s |
sync_aggregate | sync_committee_rewards_duplicate_committee_max_effective_balance_only_participate_second_one | 46951041 | 9412 | 232525958 | 902134448 | 1016 | 1181625148 | 27.963330542s |
sync_aggregate | sync_committee_rewards_duplicate_committee_no_participation | 46951041 | 9412 | 2098142 | 902134398 | 1013 | 951197226 | 21.241754708s |
sync_aggregate | sync_committee_rewards_duplicate_committee_zero_balance_only_participate_first_one | 46951041 | 9412 | 232525932 | 902134448 | 1016 | 1181625122 | 26.432450958s |
sync_aggregate | sync_committee_rewards_duplicate_committee_zero_balance_only_participate_second_one | 46951041 | 9412 | 232525932 | 902134448 | 1016 | 1181625122 | 26.584343166s |
sync_aggregate | sync_committee_rewards_empty_participants | 46951041 | 9412 | 2098142 | 902134398 | 1013 | 951197226 | 21.953464292s |
sync_aggregate | sync_committee_rewards_not_full_participants | 46951041 | 9308 | 1829629323 | 902134451 | 1016 | 2778728415 | 63.962741167s |
sync_aggregate | sync_committee_with_nonparticipating_exited_member | 46955715 | 9160 | 3335293506 | 902240109 | 1016 | 4284502782 | 99.512823083s |
sync_aggregate | sync_committee_with_nonparticipating_withdrawable_member | 46955715 | 9160 | 3335293506 | 902240109 | 1016 | 4284502782 | 99.893888958s |
sync_aggregate | sync_committee_with_participating_exited_member | 46955715 | 9156 | 3347485090 | 902240109 | 1081 | 4296694492 | 96.436304625s |
sync_aggregate | sync_committee_with_participating_withdrawable_member | 46955715 | 9156 | 3347485090 | 902240109 | 1081 | 4296694492 | 97.489207417s |
