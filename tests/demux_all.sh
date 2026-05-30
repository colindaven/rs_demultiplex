#!/bin/bash
# Colin Davenport, Dec 2021
echo "INFO: Usage: bash demux_all.sh fastq"

input_barcodes_file="barcodes.txt"
fastq=$1
cp target/release/rs_demultiplex .

echo "INFO: Starting demux"

# Read all barcodes from file listed above, run rs_demultiplex for each barcode
# Produce fastq file named barcode.fastq for each barcode
while read bc; do
  echo "$bc"
  cat $fastq  | rs_demultiplex --remove --barcode $bc > $bc.fastq

done < $input_barcodes_file


