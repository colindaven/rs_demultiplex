#!/bin/bash
#
# test_real.sh
# Downloads real FASTQ data from EBI ENA, generates a barcode list,
# and runs rs_demultiplex to verify functionality.

set -euo pipefail

# Configuration
PROJECT_ROOT="."
TEST_DIR="$PROJECT_ROOT/tests"
ENA_URL="ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR296/024/SRR29666724/SRR29666724_1.fastq.gz"
SAMPLE_FASTQ="$TEST_DIR/sample_real2.fastq.gz"
BARCODE_FILE="$TEST_DIR/barcodes_real.txt"
BINARY="$PROJECT_ROOT/target/release/rs_demultiplex"
threads=4

# create directories
mkdir -p "$TEST_DIR"
mkdir -p "$TEST_DIR/output"
cd "$PROJECT_ROOT"

echo "===> Building rs_demultiplex in release mode..."
cargo build --release -q

echo "===> Checking sample data..."
# Check if the sample file exists and is non-empty
if [ ! -s "$SAMPLE_FASTQ" ]; then
    echo "Downloading sample data from ENA..."
    curl -L "$ENA_URL" -o "$SAMPLE_FASTQ"
else
    echo "Sample file already exists, skipping download."
fi

echo "===> Generating barcode list from real data..."
# We extract the first 8 characters from the first few sequence lines
# to ensure we have valid barcodes for testing.
# Note: 'set +o pipefail' prevents the script from exiting when 'head' closes the pipe early (SIGPIPE).
set +o pipefail
gzip -dc "$SAMPLE_FASTQ" | \
    awk 'NR % 4 == 2' | \
    head -n 50 | \
    cut -c 1-8 | \
    sort | uniq | \
    head -n 20 > "$BARCODE_FILE"
set -o pipefail

if [ ! -s "$BARCODE_FILE" ]; then
    echo "Error: Failed to generate barcode list. Check if $SAMPLE_FASTQ is a valid Gzip file." >&2
    exit 1
fi

echo "Selected $(wc -l < "$BARCODE_FILE" | tr -d ' ') barcodes for testing."

echo "===> Running demultiplexing... without -r to remove the barcodes - to enable accuracy checks later" 
gzip -dc "$SAMPLE_FASTQ" | "$BINARY" -t $threads -l "$BARCODE_FILE"

mv ????????.fastq "$TEST_DIR/output" 2>/dev/null || true
mv undeter*fastq "$TEST_DIR/output" 2>/dev/null || true

echo "===> Test complete."
echo "Summary of generated files in $TEST_DIR:"
ls -lh "$TEST_DIR/output"/*.fastq