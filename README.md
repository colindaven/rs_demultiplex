# rs_demultiplex

Coauthored with Google Gemini.

A high-performance FASTQ demultiplexer written in Rust, designed for speed, memory efficiency, and seamless integration into bioinformatics pipelines.

`rs_demultiplex` sorts sequencing reads based on an **exact prefix match** of a genetic barcode (oligo). It is optimized for high-throughput data, allowing you to demultiplex an entire flowcell against dozens of barcodes in a single pass.

## Features
- **Exact Prefix Matching**: Rapidly identifies barcodes at the start of the sequence.
- **Single-Pass Batch Mode**: Demultiplex a list of barcodes from a file in one go.
- **Barcode Trimming**: Optionally strip the matched barcode and its quality scores from the output FASTQ records (useful for downstream mapping).
- **Pipe-Friendly Architecture**: Reads from `stdin` and writes logs to `stderr`, making it easy to chain with `zcat`, `bwa`, or `samtools`.
- **Performance Tracking**: Real-time summary statistics provided on `stderr` after processing.

## Installation

### From Source
Ensure you have the Rust toolchain installed.

```bash
git clone https://github.com/your-repo/rs_demultiplex.git
cd rs_demultiplex
cargo build --release
```
The optimized binary will be located at `target/release/rs_demultiplex`.

## Usage

`rs_demultiplex` operates as a Unix-style filter. It reads FASTQ data from `stdin` and writes log information/statistics to `stderr`. For every matched barcode, it creates (or overwrites) an output file in the current directory.

### Data Formats

#### 1. Input FASTQ
The tool expects a standard 4-line FASTQ format. Matching is performed on the first characters of the sequence line (the prefix).

#### 2. Barcode List (`--list`)
A plain text file containing one barcode per line. Leading and trailing whitespace is automatically trimmed. Empty lines are ignored.

#### 3. Output FASTQ
Output files are named `{barcode}.fastq`. If the `--remove` flag is used, the barcode prefix is stripped from both the sequence and the quality string, keeping the records valid for downstream alignment.

#### 4. Summary Statistics
Statistics are printed to `stderr` upon completion, including match counts per barcode and total bases processed.

---

### Command-line Arguments
```
rs_demultiplex [OPTIONS]
```
- `-b`, `--barcode <barcode>`: Single oligo barcode to search for (e.g., `AGGATTCC`). Ignored if `--list` is used.
- `-l`, `--list <barcode_list_file>`: Path to a file containing a list of barcodes, one per line.
- `-r`, `--remove`: Set this to remove the barcode sequence and quality from the FASTQ record. Default: `off`.
- `-d`, `--debug`: Enable debug logging to stderr.
- `-z`, `--gzip`: Compress all output FASTQ files using Gzip.
- `--gz-input`: Specify that the input from stdin is already gzipped.
- `-v`, `--version`: Show version information.

---

## Detailed Examples

### Single Barcode with Pipe
```bash
cat data.fastq | target/release/rs_demultiplex -b AATGTTCT -r
```

### Processing Gzipped Data
Bioinformatics data is often compressed. You can stream directly into the tool:
```bash
zcat large_data.fastq.gz | target/release/rs_demultiplex -l barcodes.txt -r
```

---

## Bash Scripts

Two helper scripts are provided in the `src/` directory to automate common tasks:
Several helper scripts are provided to automate common tasks and testing:

### 1. Development/Testing: `src/test_debug.sh`
Used for rapid testing during development.
- **Action**: Rebuilds the project in `debug` mode.
- **Data**: Uses the included `test.fastq`.
- **Logic**: Filters for the `AATGTTCT` barcode and outputs to `AATGTTCT.fastq`.
### 1. `test_debug.sh`
Designed for rapid testing during development.
- **Action**: Rebuilds the project in `debug` mode and runs a test against a single barcode.
- **Data**: Uses the included `tests/test.fastq`.
- **Usage**:
```bash
./src/test_debug.sh
./test_debug.sh
```

### 2. Production: `src/run_demultiplex.sh`
Used for processing real datasets with multiple barcodes.
- **Action**: Rebuilds the project in `--release` mode for maximum performance.
- **Arguments**: Requires path to input FASTQ and path to barcode list.
- **Logic**: Runs demultiplexing with the `--remove` flag enabled to trim barcodes.
```bash
./src/run_demultiplex.sh input.fastq barcodes.txt
```

---

## Example `barcodes.txt`
```
AATGTTCT
AGGACATT
```
