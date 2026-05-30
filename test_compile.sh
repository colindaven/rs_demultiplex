#!/bin/bash
cargo build --release
cat test.fastq  | target/release/rs_demultiplex --remove -l barcodes.txt
