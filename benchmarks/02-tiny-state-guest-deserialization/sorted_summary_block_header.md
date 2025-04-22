| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
block_header | basic_block_header | 264367 | 18696661 | 93392 | 2109997 | 902102373 | 1012 | 923271940 | 19.907453708s |
block_header | invalid_multiple_blocks_single_slot | 264367 | 18696661 | 93392 | 2100295 | 902102373 | 1012 | 923262238 | 19.809696s |
block_header | invalid_parent_root | 264367 | 18696661 | 93392 | 2109997 | 902102373 | 1012 | 923271940 | 19.71908425s |
block_header | invalid_proposer_index | 264367 | 18696661 | 93392 | 2109997 | 902102373 | 1012 | 923271940 | 19.841570709s |
block_header | invalid_proposer_slashed | 264367 | 18696663 | 93392 | 2109997 | 902102373 | 1012 | 923271942 | 19.97571625s |
block_header | invalid_slot_block_header | 264367 | 18696661 | 93392 | 2100290 | 902102373 | 1012 | 923262233 | 19.677046667s |
