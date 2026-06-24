# rs_demultiplex

Coauthored with Google Gemini.

**A high-performance FASTQ demultiplexer written in Rust**, designed for speed, memory efficiency, and seamless integration into bioinformatics pipelines.

`rs_demultiplex` sorts sequencing reads based on an **exact prefix match** of a genetic barcode (oligo). It is optimized for high-throughput data, allowing you to demultiplex entire flowcells of sequencing data with minimal memory overhead and maximum speed.

---

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [Advanced Examples](#advanced-examples)
- [Helper Scripts](#helper-scripts)
- [Performance](#performance)
- [Data Formats](#data-formats)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)

---

## Features

- **⚡ Exact Prefix Matching**: Rapidly identifies barcodes at the start of the sequence using efficient string matching.
- **🔄 Single-Pass Batch Mode**: Demultiplex a list of barcodes from a file in one efficient pass.
- **✂️ Barcode Trimming**: Optionally strip the matched barcode and its quality scores from the output FASTQ records (useful for downstream mapping).
- **🔗 Pipe-Friendly Architecture**: Reads from `stdin` and writes logs to `stderr`, making it easy to chain with `zcat`, `bwa`, `samtools`, or other Unix tools.
- **📊 Performance Tracking**: Real-time summary statistics provided on `stderr` after processing, including match counts per barcode and total bases processed.
- **💾 Gzip Support**: Compress output FASTQ files on-the-fly with the `--gzip` flag, or handle gzipped input with `--gz-input`.
- **🧠 Memory Efficient**: Designed for processing large sequencing datasets without excessive memory consumption.

---

## Installation

### Prerequisites
Ensure you have the Rust toolchain installed. If you don't have it, install it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### From Source

```bash
git clone https://github.com/colindaven/rs_demultiplex.git
cd rs_demultiplex
cargo build --release
```

The optimized binary will be located at `target/release/rs_demultiplex`. You can optionally add it to your PATH:

```bash
export PATH="$PATH:$(pwd)/target/release"
```

---

## Quick Start

### Basic Usage: Single Barcode
```bash
cat data.fastq | ./target/release/rs_demultiplex -b AATGTTCT
```

This will output matched reads to `AATGTTCT.fastq`.

### Multiple Barcodes from a File
```bash
zcat data.fastq.gz | ./target/release/rs_demultiplex -l barcodes.txt -r -z
```

This will:
- Read gzipped FASTQ from stdin
- Match against barcodes in `barcodes.txt`
- **Remove** barcode sequences from output (`-r`)
- **Gzip** the output files (`-z`)

---

## Usage

`rs_demultiplex` operates as a Unix-style filter. It reads FASTQ data from `stdin` and writes log information/statistics to `stderr`. For every matched barcode, it creates (or overwrites) an output FASTQ file named `{barcode}.fastq`.

### Command-line Arguments

```
rs_demultiplex [OPTIONS]
```

| Option | Short | Long | Description |
|--------|-------|------|-------------|
| Barcode | `-b` | `--barcode <SEQ>` | Single oligo barcode to search for (e.g., `AGGATTCC`). Ignored if `--list` is provided. |
| Barcode List | `-l` | `--list <FILE>` | Path to a file containing a list of barcodes, one per line. |
| Remove | `-r` | `--remove` | Remove the barcode sequence and quality from the FASTQ record. Default: disabled. |
| Debug | `-d` | `--debug` | Enable debug logging to stderr for troubleshooting. |
| Gzip Output | `-z` | `--gzip` | Compress all output FASTQ files using Gzip. |
| Gzip Input | | `--gz-input` | Specify that input from stdin is already gzipped. |
| Version | `-v` | `--version` | Show version information. |

---

## Advanced Examples

### Processing with Trimming and Compression
```bash
zcat large_dataset.fastq.gz | ./target/release/rs_demultiplex \
  -l barcodes.txt \
  --remove \
  --gzip \
  2>demux.log
```

Save statistics to `demux.log` while processing gzipped input, trimming barcodes, and compressing output.

### Single Barcode with Direct Piping to Aligner
```bash
zcat reads.fastq.gz | ./target/release/rs_demultiplex \
  -b AATGTTCT \
  --remove \
  2>/dev/null | \
  bwa mem -p reference.fa - | \
  samtools view -bh -o aligned.bam
```

Demultiplex, trim barcode, and pipe directly to `bwa` for alignment in a single pipeline.

### Debug Mode for Troubleshooting
```bash
cat small_test.fastq | ./target/release/rs_demultiplex \
  -b AATGTTCT \
  --debug \
  2>debug.log
```

Enable debug output to identify unexpected behavior.

### Batch Processing Multiple Libraries
```bash
for library in lib1.fastq.gz lib2.fastq.gz lib3.fastq.gz; do
  echo "Processing $library..."
  zcat "$library" | ./target/release/rs_demultiplex \
    -l barcodes.txt \
    --remove \
    --gzip \
    2>"${library%.fastq.gz}.log"
done
```

---

## Helper Scripts

Two helper scripts are provided in the `src/` directory to automate common tasks:

### 1. `test_debug.sh` - Development/Testing
Designed for rapid testing during development.
- **Action**: Rebuilds the project in `debug` mode and runs a test against a single barcode.
- **Data**: Uses the included `tests/test.fastq`.
- **Usage**:
  ```bash
  ./src/test_debug.sh
  ```

### 2. `run_demultiplex.sh` - Production Processing
Used for processing real datasets with multiple barcodes.
- **Action**: Rebuilds the project in `--release` mode for maximum performance.
- **Arguments**: Takes path to input FASTQ and path to barcode list as arguments.
- **Logic**: Runs demultiplexing with the `--remove` flag enabled to trim barcodes.
- **Usage**:
  ```bash
  ./src/run_demultiplex.sh input.fastq barcodes.txt
  ```

---

## Performance

`rs_demultiplex` is optimized for throughput and minimal memory footprint. Key performance characteristics:

- **Throughput**: Processes millions of reads per minute on modern hardware.
- **Memory Usage**: Minimal memory overhead, suitable for processing large datasets.
- **I/O Efficiency**: Leverages Unix pipes and streaming I/O for seamless integration with other tools.
- **Compression**: Gzip compression (`-z`) adds minimal overhead while reducing output file sizes by ~80%.

### Benchmarking Tips
To measure performance on your system:

```bash
time zcat large_data.fastq.gz | ./target/release/rs_demultiplex \
  -l barcodes.txt \
  --remove \
  --gzip \
  2>stats.log > /dev/null
```

The `time` command will show wall-clock time, CPU time, and memory usage.

---

## Data Formats

### 1. Input FASTQ
The tool expects a standard 4-line FASTQ format:
```
@read_id
SEQUENCE
+
QUALITY
```

Matching is performed on the first characters of the sequence line (the prefix). The entire read is output if the prefix matches any barcode.

### 2. Barcode List (`--list`)
A plain text file containing one barcode per line. Leading and trailing whitespace is automatically trimmed. Empty lines are ignored.

**Example `barcodes.txt`:**
```
AATGTTCT
AGGACATT
TTGGATCC
CCTAGGAA
```

### 3. Output FASTQ
Output files are named `{barcode}.fastq` (or `{barcode}.fastq.gz` if `--gzip` is used). If the `--remove` flag is used, the barcode prefix is stripped from both the sequence and the quality string, keeping the records valid for downstream alignment.

**Example output before trimming:**
```
@read_001
AATGTTCTAGGCTAGCTAGC
+
IIIIIIIIHJJJJJJJJJJJ
```

**Example output after trimming (with `-r`):**
```
@read_001
AGGCTAGCTAGC
+
HJJJJJJJJJJJ
```

### 4. Summary Statistics
Statistics are printed to `stderr` upon completion, including:
- Match counts per barcode
- Total bases processed
- Number of unmatched reads
- Processing time and throughput estimates

**Example stderr output:**
```
Demultiplexing complete!
Barcode AATGTTCT: 150,234 reads
Barcode AGGACATT: 98,567 reads
Barcode TTGGATCC: 125,891 reads
Unmatched: 25,308 reads
Total bases processed: 45,230,450
```

---

## Troubleshooting

### No Output Files Generated
- **Check**: Are your barcodes present in the input FASTQ?
- **Solution**: Use `--debug` mode to see matching details. Verify barcode sequences are correct.

### Memory Issues with Large Files
- **Solution**: The tool is designed to stream data efficiently. If experiencing issues, ensure you have sufficient RAM and try processing in batches.

### Slow Performance
- **Solution**: Use `--release` build (the default with our scripts). Ensure you're not piping through unnecessary tools.

### Gzip Compression Not Working
- **Check**: Ensure output directory is writable.
- **Solution**: Verify file permissions and disk space availability.

---

## Examples Walkthrough

### Scenario: Demultiplex a 10x Genomics Library

**Input:**
- `raw_library.fastq.gz` (gzipped FASTQ)
- `10x_barcodes.txt` (list of 10x cell barcodes)

**Command:**
```bash
zcat raw_library.fastq.gz | ./target/release/rs_demultiplex \
  -l 10x_barcodes.txt \
  --remove \
  --gzip \
  2>demux_stats.log
```

**Output:**
- One gzipped file per barcode: `{barcode}.fastq.gz`
- Statistics in `demux_stats.log`

### Scenario: Quality Control - Find Unexpected Barcodes

**Command:**
```bash
zcat raw_library.fastq.gz | ./target/release/rs_demultiplex \
  -b UNEXPECTED_BARCODE \
  2>unexpected.log
```

This creates `UNEXPECTED_BARCODE.fastq` containing all reads with this prefix, helping you identify contamination.

---

## Contributing

Contributions are welcome! Please feel free to submit issues, fork the repository, and create pull requests for any improvements.

### Building from Source (Development)
```bash
cargo build          # Debug build
cargo build --release # Optimized release build
cargo test           # Run tests
cargo fmt            # Format code
cargo clippy         # Lint code
```

---

## License

[Specify your license here if applicable]

---

## Citation

If you use `rs_demultiplex` in your research, please cite this repository:

```
@software{rs_demultiplex,
  author = {Colin Daven},
  title = {rs_demultiplex: High-Performance FASTQ Demultiplexer},
  url = {https://github.com/colindaven/rs_demultiplex},
  year = {2024}
}
```

---

## Support & Contact

For issues, questions, or suggestions, please open an issue on [GitHub](https://github.com/colindaven/rs_demultiplex/issues).
