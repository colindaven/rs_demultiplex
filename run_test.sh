#!/bin/bash
# Colin Davenport, Dec 2021

cargo build
oligo="ATC"
#cargo run -q  && cat test.fastq  | target/debug/rs_demultiplex  --remove --barcode $oligo > $oligo.txt

oligo="ATC"
echo "INFO: running command for oligo $oligo, note --remove is given. Read and qual should be len($oligo) bp shorter"
cat test.fastq  | target/debug/rs_demultiplex  --remove --barcode $oligo > $oligo.txt

echo "INFO: produced this output - head -n 12"
head -n 12 $oligo.txt

oligo="AAT"
echo "INFO: running command for oligo $oligo, note --remove not given. Reads should be original length."
cat test.fastq  | target/debug/rs_demultiplex --barcode $oligo > $oligo.txt

echo "INFO: produced this output - head -n 12"
head -n 12 $oligo.txt

#echo "removing file"
#rm $oligo.txt

