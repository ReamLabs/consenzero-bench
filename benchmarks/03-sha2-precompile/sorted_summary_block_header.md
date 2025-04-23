| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
block_header | basic_block_header | 264367 | 18696661 | 93392 | 2102509 | 237557349 | 1012 | 258719428 | 6.311404458s |
block_header | invalid_multiple_blocks_single_slot | 264367 | 18696661 | 93392 | 2100295 | 237557349 | 1012 | 258717214 | 6.311324459s |
block_header | invalid_parent_root | 264367 | 18696661 | 93392 | 2102509 | 237557349 | 1012 | 258719428 | 6.378614042s |
block_header | invalid_proposer_index | 264367 | 18696661 | 93392 | 2102509 | 237557349 | 1012 | 258719428 | 6.333767958s |
block_header | invalid_proposer_slashed | 264367 | 18696663 | 93392 | 2102509 | 237557349 | 1012 | 258719430 | 6.324116417s |
block_header | invalid_slot_block_header | 264367 | 18696661 | 93392 | 2100290 | 237557349 | 1012 | 258717209 | 6.336270417s |
