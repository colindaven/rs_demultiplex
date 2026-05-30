//Rust FASTQ demultiplexer

extern crate bio;
extern crate argparse;
extern crate rayon;

use argparse::{ArgumentParser, Store, StoreTrue, Print};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use bio::io::fastq;
use bio::io::fastq::FastqRead;


// ## Changelog
// 0.1.7 - Added multi-threading
// 0.1.6 - Added multi-barcode support via --list. 
// 0.1.5 - add bash script for reading barcode lists
// 0.1.4 - added bigger test script and debug parameters
// 0.1.3 - barcode removal
// 0.1.2 - add arg parsing
// 0.1.1 - add arg parsing basis
// 0.1.0 - now works for variable length oligos, was previously just 8bp oligos


fn main() {
    //Warning - if you set debug to true, errors and warnings will be sent to stderr (previously stdout). Check FASTQ is conform!
    let mut debug: bool = false;
    let version = env!("CARGO_PKG_VERSION");

    ////////////////
    // Parse input arguments
    let mut input_oligo = "ATAT".to_string();
    let mut barcode_file_path = "".to_string();
    let mut remove_oligo = false;
    let mut num_threads: usize = 0;
   
    {  // this block limits scope of borrows by parser.refer() method
        let mut parser = ArgumentParser::new();
        parser.set_description("FASTQ multi-barcode prefix demultiplexer.");
        
        parser.add_option(&["-v", "--version"],
            Print(format!("rs_demultiplex version {}", version)), "Show version");

        parser.refer(&mut input_oligo)
            .add_option(&["-b", "--barcode"], Store,
            "Single oligo barcode to search for (ignored if --list is used).");
 
        parser.refer(&mut debug)
            .add_option(&["-d", "--debug"], StoreTrue, "Enable debug logging to stderr");

        parser.refer(&mut barcode_file_path)
            .add_option(&["-l", "--list"], Store,
            "Path to a file containing a list of barcodes (one per line).");

        parser.refer(&mut remove_oligo)
            .add_option(&["-r", "--remove"], StoreTrue,
                    "Set this to remove barcode sequence and quality from FASTQ file. Default: off");

        parser.refer(&mut num_threads)
            .add_option(&["-t", "--threads"], Store,
            "Number of threads for parallel processing (default: 0, which uses all available cores).");

        parser.parse_args_or_exit();
    } 
    
    // Configure Rayon thread pool if specified
    if num_threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .expect("Failed to initialize thread pool");
    }

    // Collect barcodes from file or single argument
    let mut barcodes: Vec<String> = Vec::new();
    if !barcode_file_path.is_empty() {
        let file = File::open(&barcode_file_path).expect("Could not open barcode list file");
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let b = line.expect("Could not read line from barcode file").trim().to_string();
            // Skip empty lines, comment lines (#), or header lines (>)
            if b.is_empty() || b.starts_with('#') || b.starts_with('>') {
                continue;
            }
            // Validate DNA characters (A,C,G,T,N) and normalize to uppercase
            if b.chars().all(|c| matches!(c, 'A' | 'C' | 'G' | 'T' | 'N' | 'a' | 'c' | 'g' | 't' | 'n')) {
                barcodes.push(b.to_uppercase());
            } else {
                eprintln!("Warning: Skipping invalid barcode '{}'. It must only contain A, T, G, C, or N.", b);
            }
        }
    } else {
        let b = input_oligo.trim().to_uppercase();
        if b.chars().all(|c| matches!(c, 'A' | 'C' | 'G' | 'T' | 'N')) {
            barcodes.push(b);
        } else {
            eprintln!("Error: The provided barcode '{}' is invalid. It must only contain A, T, G, C, or N.", input_oligo);
            std::process::exit(1);
        }
    }

    // Setup FASTQ readers
    let stdin = io::stdin();
    let reader_handle: Box<dyn io::Read> = Box::new(stdin);
    let mut reader = fastq::Reader::new(reader_handle);
    
    // Prepare output writers and hit counters
    let mut writers: HashMap<String, fastq::Writer<Box<dyn Write>>> = HashMap::new();
    let mut hit_counts: HashMap<String, u64> = HashMap::new();

    for bc in &barcodes {
        let output_name = format!("{}.fastq", bc);
        let file = File::create(Path::new(&output_name))
            .expect(&format!("Could not create output file: {}", output_name));
        let writer_inner: Box<dyn Write> = Box::new(file);
        writers.insert(bc.clone(), fastq::Writer::new(writer_inner));
        hit_counts.insert(bc.clone(), 0);
    }

    let undetermined_suffix = "undetermined.fastq";
    let undetermined_file = File::create(Path::new(undetermined_suffix))
        .expect(&format!("Could not create output file: {}", undetermined_suffix));
    let mut undetermined_writer = fastq::Writer::new(Box::new(undetermined_file) as Box<dyn Write>);
    let mut undetermined_count = 0;

    if debug {
        eprintln!("rs-demultiplex version: {}. Barcodes loaded: {}", version, barcodes.len());
        if remove_oligo {
            eprintln!("Barcode trimming is enabled.");    
        }
    }
    
    let mut n_bases = 0;
    let mut total_hits = 0;
    let mut last_reported_bases = 0;
    let batch_size = 20000; // Increased batch size for better thread saturation

    loop {
        let mut batch = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            let mut rec = fastq::Record::new();
            if reader.read(&mut rec).is_err() || rec.is_empty() {
                break;
            }
            batch.push(rec);
        }

        if batch.is_empty() {
            break;
        }

        // Parallel barcode matching
        let processed_batch: Vec<(fastq::Record, Option<usize>)> = batch
            .into_par_iter()
            .map(|record| {
                let match_idx = barcodes.iter().position(|bc| record.seq().starts_with(bc.as_bytes()));
                
                if let Some(idx) = match_idx {
                    if remove_oligo {
                        let bc_len = barcodes[idx].len();
                        let trimmed = fastq::Record::with_attrs(
                            record.id(),
                            record.desc(),
                            &record.seq()[bc_len..],
                            &record.qual()[bc_len..]
                        );
                        return (trimmed, Some(idx));
                    }
                }
                (record, match_idx)
            })
            .collect();

        // Sequential writing and statistics updates
        for (record, match_idx) in processed_batch {
            n_bases += record.seq().len();
            match match_idx {
                Some(idx) => {
                    let barcode = &barcodes[idx];
                    total_hits += 1;
                    *hit_counts.get_mut(barcode).unwrap() += 1;
                    let writer = writers.get_mut(barcode).unwrap();
                    // Note: trimming was already handled in the parallel map block
                    writer.write_record(&record).expect("Failed to write record");
                }
                None => {
                    undetermined_count += 1;
                    undetermined_writer.write_record(&record).expect("Failed to write undetermined record");
                }
            }
        }

        if debug && n_bases >= last_reported_bases + 1_000_000 {
            eprintln!("Number of bases read: {} ", n_bases);
            last_reported_bases = n_bases;
        }
    }

    // Summary statistics
    if total_hits > 0 || undetermined_count > 0 || debug {
        eprintln!("\n--- Demultiplexing Summary ---");
        for barcode in &barcodes {
            eprintln!("Barcode {}: {} matches", barcode, hit_counts[barcode]);
        }
        eprintln!("Undetermined: {} reads", undetermined_count);
        eprintln!("------------------------------");
        eprintln!("Total Matched: {}", total_hits);
        eprintln!("Total bases processed: {}", n_bases);
        eprintln!("Processed into {} barcode files and {}.", writers.len(), undetermined_suffix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bio::io::fastq;

    #[test]
    fn test_barcode_prefix_match() {
        let barcode = b"AATGTTCT";
        let matching_seq = b"AATGTTCTTTGTGGAAAGGACGAA";
        let non_matching_seq = b"AGGACATTTTGTGGAAAGGACGAA";
        
        assert!(matching_seq.starts_with(barcode));
        assert!(!non_matching_seq.starts_with(barcode));
    }

    #[test]
    fn test_record_trimming_logic() {
        let id = "test_read";
        let desc = Some("description");
        let seq = b"AATGTTCTACGTACGT";
        let qual = b"AAAAAAAA########";
        let bc_len = 8;

        let record = fastq::Record::with_attrs(id, desc, seq, qual);
        
        // Simulate the trimming logic in main loop
        let trimmed_seq = &record.seq()[bc_len..];
        let trimmed_qual = &record.qual()[bc_len..];

        assert_eq!(trimmed_seq, b"ACGTACGT");
        assert_eq!(trimmed_qual, b"########");
        assert_eq!(trimmed_seq.len(), trimmed_qual.len());
    }

    #[test]
    fn test_exact_length_match() {
        let barcode = b"AATGTTCT";
        let seq = b"AATGTTCT";
        let qual = b"AAAAAAAA";
        
        assert!(seq.starts_with(barcode));
        
        let trimmed_seq = &seq[barcode.len()..];
        let trimmed_qual = &qual[barcode.len()..];
        
        assert!(trimmed_seq.is_empty());
        assert!(trimmed_qual.is_empty());
    }

    #[test]
    fn test_record_empty() {
        let record = fastq::Record::new();
        assert!(record.is_empty());
    }
}
