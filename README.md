# rs_demultiplex

Demultiplex reads in FASTQ format from a FASTQ input file according to an oligo in the first x bases (eg Unique Molecular Identifier, UMI).

 - accept oligo of length x, eg 8bp as first command line arg, see examples below
 - read FASTQ
 - match given oligo exactly to first x bp, eg 8bp, of the read 

 - write all reads - either default unmodified, i.e. still containing oligo - to standard out
 - alternatively, use --remove to trim off the oligo from the read and shorten the fastq qual line too.

See examples below.

## Install

#### 1. Install from release (easy)
 - download the release file for amd64 architectures
 - run: `chmod a+x rs_demultiplex`
 - Finally `rs_demultiplex`



## Install: build: 

#### 1. From source: Please first install Rust and Cargo from their website. The clone the repo and build as below.

#### 2. Run one of following for testing (after source code changes):

```
bash run_test.sh

cargo build --release
cat test.fastq  | target/release/rs_demultiplex --remove --barcode ATGC > ATGC.txt
```

#### 3. fast prod version - first build a release binary
cargo build --release


#### 4. run on a whole list of barcodes

First populate barcodes.txt with one raw barcode per line (not fasta, raw format). 
Leave a blank line at the end, else the last barcode might be missed.

```
bash demux_all.sh
```

#### 5. Or avoid the bash script, and run things manually 
##### 5a. Now pipe in a FASTQ file into the release binary target/release/rs_demultiplex
##### 5b. specify an oligo barcode at the start of a read eg AACTCCGC  
##### 5c. reroute standard out output to a FASTQ file (eg AACTCCGC.txt)

```


# suggested command lines with oligo/barcode supplied as first argument
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex --remove --barcode AACTCCGC > AACTCCGC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex --remove --barcode AAGCGGTG > AAGCGGTG.txt


```


## Usage
```
--
target/release/rs_demultiplex -h
Usage:
  target/release/rs_demultiplex [OPTIONS]


Optional arguments:
  -h,--help             Show this help message and exit
  -b,--barcode BARCODE  Oligo barcode eg AGGATTCC to search for. Any length.
  -r,--remove           Set this to remove barcode sequence and quality from
                        FASTQ file. Default: off
```

