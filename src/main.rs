//Rust FASTQ demultiplexer
//Demultiplex FASTQs by index in first 8 characters in the FASTQ sequence (barcode)
//Barcode must be supplied by arg
//Usage - see Github README.md for examples.
//Usage: cat Undetermined_S0_R1.fastq | target/release/rs_demultiplex AACTCCGC > AACTCCGC.txt

extern crate bio;
extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::env;
use std::str;
use std::fs::File;
//use std::io::prelude::*;
//use std::path::Path;
use std::collections::HashMap;
use std::io;
use std::io::Write; 
use bio::io::fastq;
use bio::io::fastq::FastqRead;


// ## Changelog
// 0.14 - added bigger test script and debug parameters
// 0.13 - barcode removal
// 0.12 - add arg parsing
// 0.11 - add arg parsing basis
// 0.10 - now works for variable length oligos, was previously just 8bp oligos

fn version() ->  String {
    let version: String = str::to_string("0.13");
    eprintln!("rs-demultiplex version: {}. Reads input from std in, eg cat x.fastq | rs_demultiplex --barcode AGAG --remove > barcode.fastq ", &version);
    version
} 


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

/**
fn check_args (local_args: &Vec<String>, version: &str, -> bool) {
    let mut args_ok: bool = false;
    if local_args.len() < 2 {
        eprintln!("Version: {}. Usage: supply an input oligo, and remember to pipe in data. eg cat in.fastq | rs_demultiplex AGAGAGAG > AGAGAGAG.fastq", version);
        &args_ok = false;
        //return;
        //break/exit etc
        
    }
    else if local_args.len() >= 2 {
        eprintln!("Version: {}. Input oligo supplied {} ", version, local_args[1]);
        // Note- no return here, proceed
        &args_ok = true;
    }
    return args_ok
}
*/


fn main() {


    //Warning - if you set debug to true, errors and warnings will be sent to stderr (previously stdout). Check FASTQ is conform!
    let debug: bool = false;
    version();


    ////////////////
    // Parse input arguments

    //let mut input_file = "test.fastq".to_string();
    let mut input_oligo = "ATAT".to_string();
    let mut remove_oligo = false;
   
    {  // this block limits scope of borrows by parser.refer() method
        let mut parser = ArgumentParser::new();

        parser.refer(&mut input_oligo)
            .add_option(&["-b", "--barcode"], Store,
            "Oligo barcode eg AGGATTCC to search for. Any length.");
 
        parser.refer(&mut remove_oligo)
            .add_option(&["-r", "--remove"], StoreTrue,
                    "Set this to remove barcode sequence and quality from FASTQ file. Default: off");
        /*    
        parser.refer(&mut input_file)
            .add_option(&["-f", "--input_file"], Store,
                    "Input file FASTQ (not functional, better to just cat the file in!");
        */
        parser.parse_args_or_exit();
    } 
    //let input_csv: String = str::to_string(&input_file);
    let input_barcode: String = str::to_string(&input_oligo);
    let remove_oligo = remove_oligo;

    if debug {
        if remove_oligo {
            eprintln!("Remove parameter has been set");    
        }    
    }
    
    // Args 
    /*
    let args: Vec<String> = env::args().collect();
    let mut args_ok: bool = false;
    args_ok = check_args(&args, &version);
    if !args_ok {
        eprintln("Args not ok, exiting");
        return
    }
    */
    
    let barcode_from_args = input_barcode;
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


    while let Ok(()) = reader.read(&mut record) {    
        
        if record.is_empty() {
            let _check = record.check();
            if debug {
                eprintln!("Warning: Record empty {} ", record);
            }

            break;  
        } 
        
        // demultiplex 
        // get sequence from fastq record, then first x characters. This is the barcode from the start of the read

        let sequence = record.seq();
        let sequence_text = str::from_utf8(sequence).unwrap();

        let barcode_from_args_length = barcode_from_args.len();
        let sequence_oligo = &sequence_text[0..barcode_from_args_length]; 
        //eprintln!("barcode args {}, sequence_oligo {} ", &barcode_from_args, sequence_oligo);

        if sequence_oligo == barcode_from_args {
            if debug {
                eprintln!("Hit ! Barcode  {}, seq_oligo from read {} ", &barcode_from_args, sequence_oligo);
            }
            counts_vector[0] += 1;
            if remove_oligo {
                // Modify the fastq record seq and qual lines to remove the oligo/barcode
		        let id = record.id();
                let sequence = record.seq();
                let qual = record.qual();

		        // TODO Convert to TextSlice and u8 respectively.
               let trim_seq = &sequence[barcode_from_args_length..sequence.len()];
               let trim_qual = &qual[barcode_from_args_length..qual.len()];
               let record_trimmed = fastq::Record::with_attrs(id, record.desc(), trim_seq, trim_qual);

                if debug {
                    let mut sequence_text = str::from_utf8(sequence).unwrap();
                    let mut qual_text = str::from_utf8(qual).unwrap();
                    // now modify the oligo text and qual, convert 
                    sequence_text = &sequence_text[barcode_from_args_length..sequence_text.len()];
                    qual_text = &qual_text[barcode_from_args_length..qual_text.len()];
                    println!("Hit ! Barcode  {}, seq_oligo from read {}, seq  text oligo removed {}", &barcode_from_args, sequence_oligo, sequence_text);
                    println!("Hit ! Barcode  {}, seq_oligo from read {}, qual text oligo removed {}", &barcode_from_args, sequence_oligo, qual_text);
                    println!("sequence length: {}, qual length {}", sequence.len(), qual.len());
                    println!("trim_seq length: {}, trim_qual length {}", trim_seq.len(), trim_qual.len());
                }
		        // Do not create a new record object - performance. Just write.   write_fmt failed though.   
		        //write_fmt!(writer, "@{}\n{}\n+\n{}\n", id, trim_seq, trim_qual);
                writer.write_record(&record_trimmed);     

            }
            else {
	            //write to stdout without changing
                writer.write_record(&record);
            }
        }  

        // keep track of the total number of bases
        n_bases += record.seq().len();
        if n_bases % 1000000 == 0 {
            if debug {
                eprintln!("Number of bases read: {} ", n_bases);
            }
        } 
    
        line_counter += 1;
        //eprintln!("Line count:{}", line_counter);     


    }

    if debug {
        eprintln!("Counts {}", counts_vector[0]);
        eprintln!("There are {} bases in your file.", n_bases);
    }

}
