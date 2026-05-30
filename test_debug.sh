#!/bin/bash
#
# Script to run the rs_demultiplex tool in debug mode with a single barcode.
# This is primarily for testing during development.
#
# Usage: ./scripts/test_debug.sh
#
# Prerequisites:
# - Rust and Cargo installed.
# - 'test.fastq' file exists in the project root.
# - 'rs_demultiplex' binary has been built in debug mode (cargo build).

set -euo pipefail

# Define input and output files
INPUT_FASTQ="tests/test.fastq"
BARCODE="AATGTTCT" # Example barcode from test.fastq, present in test.fastq
OUTPUT_FASTQ="${BARCODE}.fastq"

# Check if the input FASTQ file exists
if [ ! -f "$INPUT_FASTQ" ]; then
    echo "Error: Input FASTQ file '$INPUT_FASTQ' not found." >&2
    echo "Please ensure 'test.fastq' is in the project root." >&2
    exit 1
fi

echo "Building rs_demultiplex in debug mode..."
cargo build -q

echo "Running demultiplexing for barcode '$BARCODE' (debug mode)..."
cat "$INPUT_FASTQ" | target/debug/rs_demultiplex -b "$BARCODE" > "$OUTPUT_FASTQ"

echo "Demultiplexing complete. Output written to '$OUTPUT_FASTQ'."
echo "You can inspect the output with: head '$OUTPUT_FASTQ'"