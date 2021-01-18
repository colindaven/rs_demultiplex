# rs_demultiplex

Usage: 

1. Run one of following:

cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex  > out.fastq

or 
cargo build --release -q  && cat test_big.fastq  | target/release/rs_demultiplex  > out.fastq

cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex OLIGO > OLIGO.txt


```
cargo build --release

cat test_big.fastq  | target/debug/rs_demultiplex AACTCCGC > AACTCCGC.txt
cat test_big.fastq  | target/debug/rs_demultiplex AAGCGGTG > AAGCGGTG.txt
cat test_big.fastq  | target/debug/rs_demultiplex AATGTTCT > AATGTTCT.txt
cat test_big.fastq  | target/release/rs_demultiplex AGACTGAC > AGACTGAC.txt
cat test_big.fastq  | target/release/rs_demultiplex AGGACATT > AGGACATT.txt
cat test_big.fastq  | target/release/rs_demultiplex ATTAGACC > ATTAGACC.txt
cat test_big.fastq  | target/release/rs_demultiplex CAAAGTGT > CAAAGTGT.txt
cat test_big.fastq  | target/release/rs_demultiplex CACTTGAG > CACTTGAG.txt
cat test_big.fastq  | target/release/rs_demultiplex CCTCTCTT > CCTCTCTT.txt
cat test_big.fastq  | target/release/rs_demultiplex CTAGTAGC > CTAGTAGC.txt
cat test_big.fastq  | target/release/rs_demultiplex GACTGACA > GACTGACA.txt
cat test_big.fastq  | target/release/rs_demultiplex GCCAACCT > GCCAACCT.txt
cat test_big.fastq  | target/release/rs_demultiplex GCGGTGAA > GCGGTGAA.txt
cat test_big.fastq  | target/release/rs_demultiplex GCTCGTGC > GCTCGTGC.txt
cat test_big.fastq  | target/release/rs_demultiplex GGACATTA > GGACATTA.txt
cat test_big.fastq  | target/release/rs_demultiplex GTCCTTCG > GTCCTTCG.txt
cat test_big.fastq  | target/release/rs_demultiplex GTGTAAAC > GTGTAAAC.txt
cat test_big.fastq  | target/release/rs_demultiplex GTTACCGA > GTTACCGA.txt
cat test_big.fastq  | target/release/rs_demultiplex TAAACGCG > TAAACGCG.txt
cat test_big.fastq  | target/release/rs_demultiplex TAGCTAGA > TAGCTAGA.txt
cat test_big.fastq  | target/release/rs_demultiplex TATGGCAC > TATGGCAC.txt
cat test_big.fastq  | target/release/rs_demultiplex TCATTTCC > TCATTTCC.txt
cat test_big.fastq  | target/release/rs_demultiplex TCGGCCGT > TCGGCCGT.txt
cat test_big.fastq  | target/release/rs_demultiplex TCTCAAAG > TCTCAAAG.txt
cat test_big.fastq  | target/release/rs_demultiplex TGCGTCTG > TGCGTCTG.txt

#test5m
cat test5m.fastq  | target/debug/rs_demultiplex AACTCCGC > AACTCCGC.txt
cat test5m.fastq  | target/debug/rs_demultiplex AAGCGGTG > AAGCGGTG.txt
cat test5m.fastq  | target/debug/rs_demultiplex AATGTTCT > AATGTTCT.txt
cat test5m.fastq  | target/release/rs_demultiplex AGACTGAC > AGACTGAC.txt
cat test5m.fastq  | target/release/rs_demultiplex AGGACATT > AGGACATT.txt
cat test5m.fastq  | target/release/rs_demultiplex ATTAGACC > ATTAGACC.txt
cat test5m.fastq  | target/release/rs_demultiplex CAAAGTGT > CAAAGTGT.txt
cat test5m.fastq  | target/release/rs_demultiplex CACTTGAG > CACTTGAG.txt
cat test5m.fastq  | target/release/rs_demultiplex CCTCTCTT > CCTCTCTT.txt
cat test5m.fastq  | target/release/rs_demultiplex CTAGTAGC > CTAGTAGC.txt
cat test5m.fastq  | target/release/rs_demultiplex GACTGACA > GACTGACA.txt
cat test5m.fastq  | target/release/rs_demultiplex GCCAACCT > GCCAACCT.txt
cat test5m.fastq  | target/release/rs_demultiplex GCGGTGAA > GCGGTGAA.txt
cat test5m.fastq  | target/release/rs_demultiplex GCTCGTGC > GCTCGTGC.txt
cat test5m.fastq  | target/release/rs_demultiplex GGACATTA > GGACATTA.txt
cat test5m.fastq  | target/release/rs_demultiplex GTCCTTCG > GTCCTTCG.txt
cat test5m.fastq  | target/release/rs_demultiplex GTGTAAAC > GTGTAAAC.txt
cat test5m.fastq  | target/release/rs_demultiplex GTTACCGA > GTTACCGA.txt
cat test5m.fastq  | target/release/rs_demultiplex TAAACGCG > TAAACGCG.txt
cat test5m.fastq  | target/release/rs_demultiplex TAGCTAGA > TAGCTAGA.txt
cat test5m.fastq  | target/release/rs_demultiplex TATGGCAC > TATGGCAC.txt
cat test5m.fastq  | target/release/rs_demultiplex TCATTTCC > TCATTTCC.txt
cat test5m.fastq  | target/release/rs_demultiplex TCGGCCGT > TCGGCCGT.txt
cat test5m.fastq  | target/release/rs_demultiplex TCTCAAAG > TCTCAAAG.txt
cat test5m.fastq  | target/release/rs_demultiplex TGCGTCTG > TGCGTCTG.txt

# prod - built on hpc-rc09
cat Undetermined_S0_R1.fastq  | target/debug/rs_demultiplex AACTCCGC > AACTCCGC.txt
cat Undetermined_S0_R1.fastq  | target/debug/rs_demultiplex AAGCGGTG > AAGCGGTG.txt
cat Undetermined_S0_R1.fastq  | target/debug/rs_demultiplex AATGTTCT > AATGTTCT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex AGACTGAC > AGACTGAC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex AGGACATT > AGGACATT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex ATTAGACC > ATTAGACC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CAAAGTGT > CAAAGTGT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CACTTGAG > CACTTGAG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CCTCTCTT > CCTCTCTT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CTAGTAGC > CTAGTAGC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GACTGACA > GACTGACA.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GCCAACCT > GCCAACCT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GCGGTGAA > GCGGTGAA.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GCTCGTGC > GCTCGTGC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GGACATTA > GGACATTA.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GTCCTTCG > GTCCTTCG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GTGTAAAC > GTGTAAAC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GTTACCGA > GTTACCGA.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TAAACGCG > TAAACGCG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TAGCTAGA > TAGCTAGA.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TATGGCAC > TATGGCAC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TCATTTCC > TCATTTCC.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TCGGCCGT > TCGGCCGT.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TCTCAAAG > TCTCAAAG.txt
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TGCGTCTG > TGCGTCTG.txt


/mnt/ngsnfs/tools/dev/rs_demultiplex$ ls -lh *.txt
-rw-rw-r-- 1 rcug rcug    0 Jan 18 16:27 AACTCCGC.txt
-rw-rw-r-- 1 rcug rcug    0 Jan 18 16:27 AAGCGGTG.txt
-rw-rw-r-- 1 rcug rcug  11M Jan 18 16:27 AATGTTCT.txt
-rw-rw-r-- 1 rcug rcug 4.0M Jan 18 16:27 AGACTGAC.txt
-rw-rw-r-- 1 rcug rcug 9.9M Jan 18 16:27 AGGACATT.txt
-rw-rw-r-- 1 rcug rcug    0 Jan 18 16:27 ATTAGACC.txt
-rw-rw-r-- 1 rcug rcug 7.5K Jan 18 16:27 CAAAGTGT.txt
-rw-rw-r-- 1 rcug rcug 9.4M Jan 18 16:27 CACTTGAG.txt
-rw-rw-r-- 1 rcug rcug 8.9M Jan 18 16:27 CCTCTCTT.txt
-rw-rw-r-- 1 rcug rcug 4.4M Jan 18 16:27 CTAGTAGC.txt
-rw-rw-r-- 1 rcug rcug 9.8M Jan 18 16:28 GACTGACA.txt
-rw-rw-r-- 1 rcug rcug 4.8M Jan 18 16:28 GCCAACCT.txt
-rw-rw-r-- 1 rcug rcug    0 Jan 18 16:28 GCGGTGAA.txt
-rw-rw-r-- 1 rcug rcug 7.9M Jan 18 16:28 GCTCGTGC.txt
-rw-rw-r-- 1 rcug rcug 3.5M Jan 18 16:28 GGACATTA.txt
-rw-rw-r-- 1 rcug rcug 8.8M Jan 18 16:28 GTCCTTCG.txt
-rw-rw-r-- 1 rcug rcug 6.6M Jan 18 16:28 GTGTAAAC.txt
-rw-rw-r-- 1 rcug rcug 8.2M Jan 18 16:28 GTTACCGA.txt
-rw-rw-r-- 1 rcug rcug 5.2M Jan 18 16:28 TAAACGCG.txt
-rw-rw-r-- 1 rcug rcug    0 Jan 18 16:28 TAGCTAGA.txt
-rw-rw-r-- 1 rcug rcug 8.2M Jan 18 16:28 TATGGCAC.txt
-rw-rw-r-- 1 rcug rcug 7.7M Jan 18 16:28 TCATTTCC.txt
-rw-rw-r-- 1 rcug rcug 5.8M Jan 18 16:28 TCGGCCGT.txt
-rw-rw-r-- 1 rcug rcug 7.0M Jan 18 16:28 TCTCAAAG.txt
-rw-rw-r-- 1 rcug rcug 4.3M Jan 18 16:28 TGCGTCTG.txt
-rw-rw-r-- 1 rcug rcug    0 Jan 18 16:27 TTGTGTGG.txt


cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex AACTCCGC > AACTCCGC.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex AAGCGGTG > AAGCGGTG.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex AATGTTCT > AATGTTCT.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex AGACTGAC > AGACTGAC.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex AGGACATT > AGGACATT.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex ATTAGACC > ATTAGACC.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex CAAAGTGT > CAAAGTGT.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex CACTTGAG > CACTTGAG.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex CCTCTCTT > CCTCTCTT.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex CTAGTAGC > CTAGTAGC.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex GACTGACA > GACTGACA.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex GCCAACCT > GCCAACCT.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex GCGGTGAA > GCGGTGAA.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex GCTCGTGC > GCTCGTGC.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex GGACATTA > GGACATTA.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex GTCCTTCG > GTCCTTCG.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex GTGTAAAC > GTGTAAAC.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex GTTACCGA > GTTACCGA.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex TAAACGCG > TAAACGCG.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex TAGCTAGA > TAGCTAGA.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex TATGGCAC > TATGGCAC.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex TCATTTCC > TCATTTCC.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex TCGGCCGT > TCGGCCGT.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex TCTCAAAG > TCTCAAAG.txt
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex TGCGTCTG > TGCGTCTG.txt

```
