#!/bin/bash
# Colin Davenport, April 2021

oligo="ATC"
echo "running command for oligo $oligo"
cargo build
#cargo run -q  && cat test.fastq  | target/debug/rs_demultiplex  --remove --barcode $oligo > $oligo.txt
cat test.fastq  | target/debug/rs_demultiplex  --remove --barcode $oligo > $oligo.txt

echo "produced this output - head"
head -n 12 $oligo.txt

echo "removing file"
rm $oligo.txt

