| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
sync_aggregate | invalid_signature_bad_domain | 264371 | 18697243 | 15744 | 3347403300 | 237566961 | 1015 | 3603952828 | 82.775264625s |
sync_aggregate | invalid_signature_extra_participant | 264371 | 18697243 | 15744 | 3347403300 | 237566961 | 1015 | 3603952828 | 81.631695875s |
sync_aggregate | invalid_signature_infinite_signature_with_all_participants | 264371 | 18697243 | 15744 | 3346012252 | 237566961 | 1015 | 3602561780 | 80.967000583s |
sync_aggregate | invalid_signature_infinite_signature_with_single_participant | 264371 | 18697243 | 15744 | 231053423 | 237566958 | 1012 | 487602939 | 11.468912875s |
sync_aggregate | invalid_signature_missing_participant | 264371 | 18697243 | 15744 | 3335211685 | 237566961 | 1015 | 3591761213 | 80.1084955s |
sync_aggregate | invalid_signature_no_participants | 264371 | 18697243 | 15744 | 143888510 | 237566958 | 1012 | 400438026 | 9.574326s |
sync_aggregate | invalid_signature_past_block | 264351 | 18698295 | 15744 | 3347403528 | 237588186 | 1015 | 3603975313 | 83.39035075s |
sync_aggregate | random_all_but_one_participating_with_duplicates | 264371 | 18697243 | 15744 | 3347404996 | 237566961 | 1015 | 3603954524 | 82.575283042s |
sync_aggregate | random_high_participation_with_duplicates | 264371 | 18697243 | 15744 | 3152339237 | 237566961 | 1015 | 3408888765 | 78.052974375s |
sync_aggregate | random_low_participation_with_duplicates | 264371 | 18697243 | 15744 | 1652770319 | 237566961 | 1015 | 1909319847 | 43.591386042s |
sync_aggregate | random_misc_balances_and_half_participation_with_duplicates | 264371 | 18697243 | 15744 | 3249872131 | 237566961 | 1015 | 3506421659 | 88.847831459s |
sync_aggregate | random_only_one_participant_with_duplicates | 264371 | 18697243 | 15744 | 238541988 | 237566958 | 1012 | 495091504 | 11.644364208s |
sync_aggregate | sync_committee_rewards_duplicate_committee_full_participation | 264371 | 18697243 | 15744 | 3347404996 | 237566961 | 1015 | 3603954524 | 82.439476708s |
sync_aggregate | sync_committee_rewards_duplicate_committee_half_participation | 264371 | 18697243 | 15744 | 1786878035 | 237566961 | 1015 | 2043427563 | 46.919762125s |
sync_aggregate | sync_committee_rewards_duplicate_committee_max_effective_balance_only_participate_first_one | 264371 | 18697243 | 15744 | 232446193 | 237566958 | 1012 | 488995709 | 11.705250458s |
sync_aggregate | sync_committee_rewards_duplicate_committee_max_effective_balance_only_participate_second_one | 264371 | 18697243 | 15744 | 232446193 | 237566958 | 1012 | 488995709 | 11.440873416s |
sync_aggregate | sync_committee_rewards_duplicate_committee_no_participation | 264371 | 18697243 | 15744 | 2089513 | 237566908 | 1012 | 258638929 | 6.469312333s |
sync_aggregate | sync_committee_rewards_duplicate_committee_zero_balance_only_participate_first_one | 264371 | 18697243 | 15744 | 232446167 | 237566958 | 1012 | 488995683 | 11.598420916s |
sync_aggregate | sync_committee_rewards_duplicate_committee_zero_balance_only_participate_second_one | 264371 | 18697243 | 15744 | 232446167 | 237566958 | 1012 | 488995683 | 11.559565s |
sync_aggregate | sync_committee_rewards_empty_participants | 264371 | 18697243 | 15744 | 2089513 | 237566908 | 1012 | 258638929 | 6.530994459s |
sync_aggregate | sync_committee_rewards_not_full_participants | 264371 | 18697243 | 15744 | 1829549558 | 237566961 | 1015 | 2086099086 | 46.942670458s |
sync_aggregate | sync_committee_with_nonparticipating_exited_member | 264356 | 18698821 | 15744 | 3335213741 | 237597739 | 1015 | 3591795610 | 80.805634459s |
sync_aggregate | sync_committee_with_nonparticipating_withdrawable_member | 264356 | 18698821 | 15744 | 3335213741 | 237597739 | 1015 | 3591795610 | 81.310970875s |
sync_aggregate | sync_committee_with_participating_exited_member | 264356 | 18698821 | 15744 | 3347405325 | 237597739 | 1015 | 3603987194 | 81.5971275s |
sync_aggregate | sync_committee_with_participating_withdrawable_member | 264356 | 18698821 | 15744 | 3347405325 | 237597739 | 1015 | 3603987194 | 80.580136125s |
