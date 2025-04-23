| Operation | Test Case | Read Pre-State | Read Operation | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|----------------|----------------|---------|-----------|--------|--------------|----------------|
block_header | basic_block_header | 46949445 | 46610 | 2073605 | 902102375 | 1013 | 951176268 | 21.11641875s |
block_header | invalid_multiple_blocks_single_slot | 46949445 | 46610 | 2063903 | 902102375 | 1013 | 951166566 | 21.18295775s |
block_header | invalid_parent_root | 46949445 | 46610 | 2073605 | 902102375 | 1013 | 951176268 | 21.081354625s |
block_header | invalid_proposer_index | 46949445 | 46610 | 2073605 | 902102375 | 1013 | 951176268 | 21.04036875s |
block_header | invalid_proposer_slashed | 46949446 | 46610 | 2073605 | 902102375 | 1013 | 951176269 | 21.649789541s |
block_header | invalid_slot_block_header | 46949445 | 46610 | 2063898 | 902102375 | 1013 | 951166561 | 21.064078292s |
