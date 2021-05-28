//Rust FASTQ demultiplexer
//Demultiplex FASTQs by index in first 8 characters in the FASTQ sequence (barcode)
//Barcode must be supplied by arg
//Usage - see Github README.md for examples.
//Usage: cat Undetermined_S0_R1.fastq | target/release/rs_demultiplex AACTCCGC > AACTCCGC.txt

extern crate bio;
use std::env;
use std::str;
use std::fs::File;
//use std::io::prelude::*;
//use std::path::Path;
use std::collections::HashMap;
use std::io;
use bio::io::fastq;
use bio::io::fastq::FastqRead;

// ## Changelog
//0.10 - now works for variable length oligos, was previously just 8bp oligos



fn read_barcodes () -> Vec<String> {
    
    // TODO - can replace this with file reading code (OR move to an arguments based model, parse and demultiplex only one oligomer at a time..... )
    // THIS 
    let barcodes = vec![
        "TCTCAAAG".to_string(),
        "AACTCCGC".into(),
        "CACTTGAG".into(),  // THIS IS WRONG, keep as example only
        "TATGGCAC".into(),
        "TAAACGCG".into()
        ];
        //println!("Initial vector: {:?}", barcodes);
        return barcodes
} 

fn build_file_map(barcodes: &[String]) -> HashMap<String, File> {
    let files = HashMap::new();

    for barcode in barcodes {
        //let filename = Path::new(barcode).with_extension("txt");
        //let file = File::create(filename).expect("failed to create output file");
        //files.insert(barcode.clone(), file);
    }

    files
}

fn check_args ((local_args: &Vec<String>, version: &str), -> bool) {
    let mut args_ok: bool = false;
    if local_args.len() < 2 {
        eprintln!("Version: {}. Usage: supply an input oligo, and remember to pipe in data. eg cat in.fastq | rs_demultiplex AGAGAGAG > AGAGAGAG.fastq", version);
        &args_ok = false;
        //return;
        
    }
    else if local_args.len() >= 2 {
        eprintln!("Version: {}. Input oligo supplied {} ", version, local_args[1]);
        // Note- no return here, proceed
        &args_ok = true;
    }
    return args_ok
}


fn main() {


    let version = "0.10";
    // Args 
    let args: Vec<String> = env::args().collect();
    let mut args_ok: bool = false;
    args_ok = check_args(&args, &version);
    if !args_ok {
        eprintln("Args not ok, exiting");
        return
    }
    
    let barcode_from_args = &args[1];
    //let barcodes_vector: Vec<String> = read_barcodes();

    /*
    // create a file vector of same length of barcodes, for output
    //let mut outfile_vector: Vec<File>;
    let file_map = build_file_map(&barcodes_vector);


    let mut counts_vector: [i32; 30] = [0; 30];
    let mut n_bases = 0;
    let mut n_valid_kmers = 0;
    let mut reader = parse_fastx_file(&filename).expect("Not a valid path/file");
    */

    // Setup FASTQ readers, writers and variables

    let mut reader = fastq::Reader::new(io::stdin());
    let mut writer = fastq::Writer::new(io::stdout());
    let mut record = fastq::Record::new();
    let mut n_bases = 0;
    let mut line_counter = 0;
    let mut counts_vector: [i32; 30] = [0; 30];
    //let file_map = build_file_map(&barcodes_vector);

    //Warning - if you set debug to true, errors and warnings will be sent to stdout and fastq will not be format conform
    let debug: bool = false;

    while let Ok(()) = reader.read(&mut record) {    
        
        if record.is_empty() {
            let _check = record.check();
            if debug {
                println!("Warning: Record empty {} ", record);
            }

            break;  
        } 
        
        // demultiplex 
        // get sequence from fastq record, then first x characters. This is the barcode from the start of the read

        let sequence = record.seq();
        let sequence_text = str::from_utf8(sequence).unwrap();

        let barcode_from_args_length = barcode_from_args.len();
        let sequence_oligo = &sequence_text[0..barcode_from_args_length]; 
        //println!("barcode args {}, sequence_oligo {} ", &barcode_from_args, sequence_oligo);

        if sequence_oligo == barcode_from_args {
            if debug {
                println!("Hit ! Barcode  {}, seq_oligo from read {} ", &barcode_from_args, sequence_oligo);
            }
            counts_vector[0] += 1;
	        //write to stdout
            writer.write_record(&record);
        }  

        // keep track of the total number of bases
        n_bases += record.seq().len();
        if n_bases % 1000000 == 0 {
            if debug {
                println!("Number of bases read: {} ", n_bases);
            }
        } 
    
        line_counter += 1;
        //println!("Line count:{}", line_counter);     


    }

    if debug {
        println!("Counts {}", counts_vector[0]);
        println!("There are {} bases in your file.", n_bases);
    }

}
