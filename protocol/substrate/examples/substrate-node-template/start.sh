#!/bin/bash

# Note substrate only keep the last 256 blocks, choosing to set pruning to archive will keep all the blocks in

cargo run --release -- --dev --pruning=archive
