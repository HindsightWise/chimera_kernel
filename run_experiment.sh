#!/bin/bash
cd /Users/zerbytheboss/Monad/monad
cargo run --bin experiment_theorem_11 > /Users/zerbytheboss/Monad/experiment_output.log 2>&1
echo "Finished running experiment!" > /Users/zerbytheboss/Monad/experiment_done.log
