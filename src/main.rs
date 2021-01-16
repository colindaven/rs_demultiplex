//Rust FASTQ demultiplexer
//Demultiplex FASTQs by index in first 8 characters in the FASTQ sequence (barcode)
//Barcode must be supplied, currently in source code

extern crate needletail;
use std::env;
//use needletail::{parse_fastx_file, Sequence, FastxReader};
use needletail::{parse_fastx_file, Sequence};
use needletail::parser::write_fastq;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn read_barcodes () -> Vec<String> {
    
    // TODO - can replace this with file reading code (OR move to an arguments based model, parse and demultiplex only one oligomer at a time..... )
    let barcodes = vec![
        "TCTCAAAG".to_string(),
        "AACTCCGC".into(),
        "CACTTGAG".into(),
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
        println!("Initial vector: {:?}", barcodes);
        return barcodes
} 

fn build_file_map(barcodes: &[String]) -> HashMap<String, File> {
    let mut files = HashMap::new();

    for barcode in barcodes {
        let filename = Path::new(barcode).with_extension("txt");
        let file = File::create(filename).expect("failed to create output file");
        files.insert(barcode.clone(), file);
    }

    files
}

fn read_file(filename: &str){

        // Create a path to the desired file
        let path = Path::new("horse.txt");
        let display = path.display();
    
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
    
        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} File reading success, contains:\n{}", display, s),
        }
    
    // file auto closed when out of scope..    
} 

fn write_file(filename: &str){

    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    static LOREM_IPSUM: &str =
    "Lorem ipsum .
    ";

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

}


fn main() {

    // Args TODO
    /*
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: supply an input oligo {} and input file {} ", args[0], args[1] );
        return;
    }
    */

    //let filename = "test5m.fastq";
    let filename = "test_big.fastq";
    //let filename = "Undetermined_S0_R1.fastq";
    //let barcodes_filename = "barcodes.txt";
    println!("Fastq filename: {} ", filename);
    //println!("Barcodes filename: {} ", barcodes_filename);

    let barcodes_vector: Vec<String> = read_barcodes();
    // create a file vector of same length of barcodes, for output
    //let mut outfile_vector: Vec<File>;
    let file_map = build_file_map(&barcodes_vector);


    let mut counts_vector: [i32; 30] = [0; 30];
    let mut n_bases = 0;
    let mut n_valid_kmers = 0;
    let mut reader = parse_fastx_file(&filename).expect("Not a valid path/file");

    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        
        // demultiplex 
        // get sequence
        let sequenceBytes = seqrec.normalize(false);
        let sequenceText = str::from_utf8(&sequenceBytes).unwrap();
        //println!("Seq: {} ", &sequenceText);

        // get first 8 chars (8chars)
        let sequenceOligo = &sequenceText[0..8]; 
        //println!("barcode vector {}, seqOligo {} ", &barcodes_vector[0], sequenceOligo);

        if sequenceOligo == barcodes_vector[0]{
            //println!("Hit ! Barcode vector {}, seqOligo {} ", &barcodes_vector[0], sequenceOligo);
            counts_vector[0] =  counts_vector[0] + 1;
            let file = file_map.get(&barcodes_vector[0] ).expect("barcode not in file map");
            
            let barcode = barcodes[0];
            let file = file_map.get(&barcode).expect("barcode not in file map");
            // write to file
            //setup writer
            write_fastq(seqrec.id(), &seqrec.seq(), seqrec.qual(), writer, LineEnding::Unix).expect("Failed to write record.");

        }  

        /*
        match &sequenceOligo () {
            &barcodes_vector[0] => println!("0"),
            &barcodes_vector[1] => println!("1"),
            &barcodes_vector[2] => println!("2"),
            _ => println!("something else!"),
        };
        */

        // keep track of the total number of bases
        n_bases += seqrec.num_bases();
        //if (n_bases % 10000000 == 0) {
        if n_bases % 1000000 == 0 {
            println!("Number of bases read: {} ", n_bases);
        } 

    }

    // read, write files
    //read_file("bla.txt");
    //write_file("out.txt");

    println!("Counts {}", counts_vector[0]);
    println!("There are {} bases in your file.", n_bases);
    println!("There are {} AAAAs in your file.", n_valid_kmers);
}