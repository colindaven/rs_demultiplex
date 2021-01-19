//Rust FASTQ demultiplexer
//Demultiplex FASTQs by index in first 8 characters in the FASTQ sequence (barcode)
//Barcode must be supplied, currently in source code

extern crate bio;
use std::env;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::io;
use bio::io::fastq;
use bio::io::fastq::FastqRead;

fn read_barcodes () -> Vec<String> {
    
    // TODO - can replace this with file reading code (OR move to an arguments based model, parse and demultiplex only one oligomer at a time..... )
    // THIS 
    let barcodes = vec![
        "TCTCAAAG".to_string(),
        "AACTCCGC".into(),
        "CACTTGAG".into(),  // THIS IS WRONG, old version, only 26 not 29 barcodes
        "TATGGCAC".into(),
        "CCTCTCTT".into(),
        "TCGGCCGT".into(),
        "GGACATTA".into(),
        "GTCCTTCG".into(),
        "CAAAGTGT".into(),
        "GTTACCGA".into(),
        "TTGTGTGG".into(),
        "AGGACATT".into(),
        "TCATTTCC".into(),
        "GCTCGTGC".into(),
        "AATGTTCT".into(),
        "GACTGACA".into(),
        "GCCAACCT".into(),
        "GTGTAAAC".into(),
        "AAGCGGTG".into(),
        "TGCGTCTG".into(),
        "CTAGTAGC".into(),
        "AGACTGAC".into(),
        "GCGGTGAA".into(),
        "ATTAGACC".into(),
        "TAGCTAGA".into(),
        "TAAACGCG".into()
        ];
        //println!("Initial vector: {:?}", barcodes);
        return barcodes
} 

fn build_file_map(barcodes: &[String]) -> HashMap<String, File> {
    let mut files = HashMap::new();

    for barcode in barcodes {
        //let filename = Path::new(barcode).with_extension("txt");
        //let file = File::create(filename).expect("failed to create output file");
        //files.insert(barcode.clone(), file);
    }

    files
}



fn main() {

    // Args TODO
    
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: supply an input oligo");
        return;
    }
    else if args.len() >= 2 {
        eprintln!("Input oligo supplied {} ", args[1]);
        // Note- no return here, proceed
    }
    
    // Ignore, now using stdin
    //let filename = "test5m.fastq";
    //let filename = "test_big.fastq";
    //let filename = "test.fastq";
    //let filename = "Undetermined_S0_R1.fastq";
    //let barcodes_filename = "barcodes.txt";

    //println!("Usage2: cargo run && cat test.fastq  | target/debug/rs_demultiplex");
    //println!("Fastq filename: {} ", filename);
    //println!("Barcodes filename: {} ", barcodes_filename);

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

    let mut reader = fastq::Reader::new(io::stdin());
    let mut writer = fastq::Writer::new(io::stdout());
    let mut record = fastq::Record::new();
    let mut n_bases = 0;
    let mut line_counter = 0;
    let mut counts_vector: [i32; 30] = [0; 30];
    //let file_map = build_file_map(&barcodes_vector);

    while let Ok(()) = reader.read(&mut record) {    
        
        if record.is_empty() {
            let check = record.check();
            break;  
        } 
        

        // demultiplex 
        // get sequence, then first 8 characters. This is the barcode from the start of the read
        let sequence = record.seq();
        let sequence_text = str::from_utf8(sequence).unwrap();
        //println!("seq:{}", sequence_text);     
        let sequence_oligo = &sequence_text[0..8]; 
        //println!("barcode args {}, sequence_oligo {} ", &barcode_from_args, sequence_oligo);


        //if sequence_oligo == barcodes_vector[0]{
        if sequence_oligo == barcode_from_args {

            //println!("Hit ! Barcode  {}, seq_oligo from read {} ", &barcode_from_args, sequence_oligo);
            counts_vector[0] =  counts_vector[0] + 1;
            /*
            let file = file_map.get(&barcodes_vector[0] ).expect("barcode not in file map");
            let barcode = barcodes_vector[0].clone();
            let file = file_map.get(&barcode).expect("barcode not in file map");
            */
            // write to stdout
            writer.write_record(&record);
        }  

        // keep track of the total number of bases
        n_bases += record.seq().len();
        if n_bases % 1000000 == 0 {
            //println!("Number of bases read: {} ", n_bases);
        } 
    
        line_counter = line_counter + 1;
        //println!("Line count:{}", line_counter);     


    }


    //println!("Counts {}", counts_vector[0]);
    //println!("There are {} bases in your file.", n_bases);

}