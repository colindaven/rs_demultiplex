# rs_demultiplex

Demultiplex reads in FASTQ format from a FASTQ input file according to an oligo in the first x bases (eg Unique Molecular Identifier, UMI).

Install - please first install Rust and Cargo from their website

Usage: 

1. Run one of following for testing (after source code changes):

```
cargo run -q  && cat test.fastq  | target/debug/rs_demultiplex  > out.fastq

cargo build --release -q  && cat test.fastq  | target/release/rs_demultiplex  > out.fastq

cargo run -q  && cat test.fastq  | target/debug/rs_demultiplex OLIGO > OLIGO.txt
```


```
# 2. fast prod version - first build a release binary
cargo build --release

# 3. 
# a. Now pipe in a FASTQ file into the release binary target/release/rs_demultiplex
# b. specify an oligo barcode at the start of a read eg AACTCCGC  
# c. reroute standard out output to a FASTQ file (eg AACTCCGC.txt)

# suggested command lines with oligo/barcode supplied as first argument
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex AACTCCGC > AACTCCGC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex AAGCGGTG > AAGCGGTG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex AATGTTCT > AATGTTCT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex AGACTGAC > AGACTGAC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex AGGACATT > AGGACATT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CACTTGAG > CACTTGAG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CCTCTCTT > CCTCTCTT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CTAGTAGC > CTAGTAGC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GACTGACA > GACTGACA.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GCCAACCT > GCCAACCT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GCTCGTGC > GCTCGTGC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GGACATTA > GGACATTA.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GTCCTTCG > GTCCTTCG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GTGTAAAC > GTGTAAAC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GTTACCGA > GTTACCGA.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TAAACGCG > TAAACGCG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TATGGCAC > TATGGCAC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TCATTTCC > TCATTTCC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TCGGCCGT > TCGGCCGT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TCTCAAAG > TCTCAAAG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TGCGTCTG > TGCGTCTG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TTGTGTGG > TTGTGTGG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex ATAGCTAG > ATAGCTAG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex ATCCAGGT > ATCCAGGT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TTCCCATC > TTCCCATC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GGGAGCAG > GGGAGCAG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GTAGGGTT > GTAGGGTT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CCCACTAC > CCCACTAC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CACTTGAG > CACTTGAG.txt

```
