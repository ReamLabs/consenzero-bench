| Operation | Test Case | Read Pre-State SSZ | Deserialize Pre-State SSZ | Read Operation Input | Process | Merkleize | Commit | Total Cycles | Execution Time |
|-----------|-----------|--------------------|---------------------------|----------------------|---------|-----------|--------|--------------|----------------|
block_header | basic_block_header | 264367 | 18696661 | 93392 | 3680183 | 32737761634 | 1111 | 32760501585 | 630.412062208s |
block_header | invalid_multiple_blocks_single_slot | 264367 | 18696661 | 93392 | 2100295 | 32739334441 | 1111 | 32760494504 | 617.998534416s |
block_header | invalid_parent_root | 264367 | 18696661 | 93392 | 3680183 | 32737761634 | 1111 | 32760501585 | 607.564563042s |
block_header | invalid_proposer_index | 264367 | 18696661 | 93392 | 3680183 | 32737761634 | 1111 | 32760501585 | 604.85921825s |
block_header | invalid_proposer_slashed | 264367 | 18696663 | 93392 | 3680183 | 32737757868 | 1111 | 32760497821 | 612.705429583s |
block_header | invalid_slot_block_header | 264367 | 18696661 | 93392 | 2100290 | 32739332005 | 1111 | 32760492063 | 620.914332292s |
