#!/bin/bash

# cargo run --bin deadline -- ../radar-passed-2.csv --list --lateness=-2 | tee -a radar-results-sim-ref-0-i.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=20 --lateness=-1 | tee -a radar-results-sim-ref-20-m.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=20 --lateness=0 | tee -a radar-results-sim-ref-20-0.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=20 --lateness=1 | tee -a radar-results-sim-ref-20-1.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=20 --lateness=2 | tee -a radar-results-sim-ref-20-2.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=20 --lateness=3 | tee -a radar-results-sim-ref-20-3.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=20 --lateness=4 | tee -a radar-results-sim-ref-20-4.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=20 --lateness=5 | tee -a radar-results-sim-ref-20-5.txt

cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=40 --lateness=-1 | tee -a radar-results-sim-ref-40-m.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=40 --lateness=0 | tee -a radar-results-sim-ref-40-0.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=40 --lateness=1 | tee -a radar-results-sim-ref-40-1.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=40 --lateness=2 | tee -a radar-results-sim-ref-40-2.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=40 --lateness=3 | tee -a radar-results-sim-ref-40-3.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=40 --lateness=4 | tee -a radar-results-sim-ref-40-4.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=40 --lateness=5 | tee -a radar-results-sim-ref-40-5.txt

cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=80 --lateness=-1 | tee -a radar-results-sim-ref-80-m.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=80 --lateness=0 | tee -a radar-results-sim-ref-80-0.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=80 --lateness=1 | tee -a radar-results-sim-ref-80-1.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=80 --lateness=2 | tee -a radar-results-sim-ref-80-2.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=80 --lateness=3 | tee -a radar-results-sim-ref-80-3.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=80 --lateness=4 | tee -a radar-results-sim-ref-80-4.txt
cargo run --bin deadline -- ../radar-passed-2.csv --list --rtt=80 --lateness=5 | tee -a radar-results-sim-ref-80-5.txt