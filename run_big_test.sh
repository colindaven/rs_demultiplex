#!/bin/bash
# Colin Davenport, April 2021

oligo="ATCCA"
testfile="/ngsssd1/rcug/wochenende_test/COPD/COPD_D_MG000060_2020_R1.fastq"

echo "running command for oligo $oligo"
cargo build --release
cat  $testfile  | target/release/rs_demultiplex  --remove --barcode $oligo > $oligo.txt

echo "produced this output - head"
head -n 12 $oligo.txt

echo "removing file"
#rm $oligo.txt

