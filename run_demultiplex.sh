#!/bin/bash
#
# Script to run the rs_demultiplex tool in release mode for multi-barcode
# demultiplexing with trimming.
#
# Usage: ./scripts/run_demultiplex.sh <input_fastq> <barcode_list_file>
#
# Arguments:
#   <input_fastq>       : Path to the input FASTQ file (e.g., Undetermined_S0_R1.fastq).
#   <barcode_list_file> : Path to a file containing barcodes, one per line.
#
# Example:
#   ./run_demultiplex.sh input.fastq barcodes.txt
#
# This script will create output FASTQ files named after each barcode
# (e.g., AATGTTCT.fastq, AGGACATT.fastq) in the current directory.
#
# Prerequisites:
# - Rust and Cargo installed.
# - 'rs_demultiplex' binary has been built in release mode (cargo build --release).

set -euo pipefail

# Check for correct number of arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <input_fastq> <barcode_list_file>" >&2
    exit 1
fi

INPUT_FASTQ="$1"
BARCODE_LIST_FILE="$2"

# Check if input FASTQ file exists
if [ ! -f "$INPUT_FASTQ" ]; then
    echo "Error: Input FASTQ file '$INPUT_FASTQ' not found." >&2
    exit 1
fi

# Check if barcode list file exists
if [ ! -f "$BARCODE_LIST_FILE" ]; then
    echo "Error: Barcode list file '$BARCODE_LIST_FILE' not found." >&2
    exit 1
fi

echo "Building rs_demultiplex in release mode..."
cargo build --release -q

echo "Running multi-barcode demultiplexing with trimming..."
echo "Input: '$INPUT_FASTQ'"
echo "Barcodes from: '$BARCODE_LIST_FILE'"

# If input is gzipped, decompress via pipe. Otherwise cat directly.
if [[ "$INPUT_FASTQ" == *.gz ]]; then
    gzip -dc "$INPUT_FASTQ" | target/release/rs_demultiplex -l "$BARCODE_LIST_FILE" -r
else
    cat "$INPUT_FASTQ" | target/release/rs_demultiplex -l "$BARCODE_LIST_FILE" -r
fi

echo "Demultiplexing complete. Output files created for each barcode."
echo "Check current directory for .fastq files (e.g., AATGTTCT.fastq)."