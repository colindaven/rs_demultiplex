# rs_demultiplex

Install - please first install Rust and Cargo from their website

Usage: 

1. Run one of following:

```
cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex  > out.fastq

cargo build --release -q  && cat test_big.fastq  | target/release/rs_demultiplex  > out.fastq

cargo run -q  && cat test_big.fastq  | target/debug/rs_demultiplex OLIGO > OLIGO.txt
```


```
cargo build --release




# prod - built on hpc-rc09
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
echo "done"
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex ATTAGACC > ATTAGACC.txt   # not in list of 29 below
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex CAAAGTGT > CAAAGTGT.txt   # not in list of 29 below
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex GCGGTGAA > GCGGTGAA.txt   # not in list of 29 below
cat Undetermined_S0_R1.fastq  | target/release/rs_demultiplex TAGCTAGA > TAGCTAGA.txt   # not in list of 29 below
#33 in total - 4 listed = 29 - CORRECT !

# final list correct 29
TTGTGTGG
AGGACATT
AAGCGGTG
GCTCGTGC
AATGTTCT
GACTGACA
CCTCTCTT
TCGGCCGT
GGACATTA
GTCCTTCG
GTTACCGA
GCCAACCT
GTGTAAAC
TCATTTCC
TGCGTCTG
CTAGTAGC
AGACTGAC
TAAACGCG
ATAGCTAG
ATCCAGGT
TTCCCATC
GGGAGCAG
GTAGGGTT
TATGGCAC
CCCACTAC
CACTTGAG
TCTCAAAG
AACTCCGC


/mnt/ngsnfs/tools/dev/rs_demultiplex$ ls -lh *.txt

/ngsssd1/rcug/2020_030434/fastq$ lh *.txt
-rw-rw-r-- 1 rcug rcug 2.8G Jan 19 09:44 AACTCCGC.txt
-rw-rw-r-- 1 rcug rcug 4.2G Jan 19 09:48 AAGCGGTG.txt
-rw-rw-r-- 1 rcug rcug 4.3G Jan 19 09:53 AATGTTCT.txt
-rw-rw-r-- 1 rcug rcug 1.6G Jan 19 09:56 AGACTGAC.txt
-rw-rw-r-- 1 rcug rcug 3.9G Jan 19 09:59 AGGACATT.txt
-rw-rw-r-- 1 rcug rcug  14K Jan 19 10:01 ATTAGACC.txt
-rw-rw-r-- 1 rcug rcug 2.5M Jan 19 10:04 CAAAGTGT.txt
-rw-rw-r-- 1 rcug rcug 3.7G Jan 19 10:07 CACTTGAG.txt
-rw-rw-r-- 1 rcug rcug 3.6G Jan 19 10:09 CCTCTCTT.txt
-rw-rw-r-- 1 rcug rcug 1.8G Jan 19 10:12 CTAGTAGC.txt
-rw-rw-r-- 1 rcug rcug 3.8G Jan 19 10:15 GACTGACA.txt
-rw-rw-r-- 1 rcug rcug 1.9G Jan 19 10:18 GCCAACCT.txt
-rw-rw-r-- 1 rcug rcug 3.6K Jan 19 10:20 GCGGTGAA.txt
-rw-rw-r-- 1 rcug rcug 3.1G Jan 19 10:23 GCTCGTGC.txt
-rw-rw-r-- 1 rcug rcug 1.4G Jan 19 10:26 GGACATTA.txt
-rw-rw-r-- 1 rcug rcug 3.5G Jan 19 10:28 GTCCTTCG.txt
-rw-rw-r-- 1 rcug rcug 2.6G Jan 19 10:31 GTGTAAAC.txt
-rw-rw-r-- 1 rcug rcug 3.3G Jan 19 10:34 GTTACCGA.txt
-rw-rw-r-- 1 rcug rcug 2.1G Jan 19 10:36 TAAACGCG.txt
-rw-rw-r-- 1 rcug rcug 1.7K Jan 19 10:39 TAGCTAGA.txt
-rw-rw-r-- 1 rcug rcug 3.2G Jan 19 10:42 TATGGCAC.txt
-rw-rw-r-- 1 rcug rcug 3.0G Jan 19 10:45 TCATTTCC.txt
-rw-rw-r-- 1 rcug rcug 2.6G Jan 19 10:47 TCGGCCGT.txt
-rw-rw-r-- 1 rcug rcug 2.8G Jan 19 10:50 TCTCAAAG.txt
-rw-rw-r-- 1 rcug rcug 1.8G Jan 19 10:53 TGCGTCTG.txt
-rw-rw-r-- 1 rcug rcug    0 Jan 19 09:31 TTGTGTGG.txt



debug version - warning old oligos
 
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
