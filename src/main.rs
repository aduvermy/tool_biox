//mod fasta_parser;
use tool_biox::fasta_parser::{fasta_parser, fasta_writer};

//mod bed_parser;
use tool_biox::bed_parser::{bed_parser};
use std::io;

fn main() -> Result<(), io::Error> {
    // test fasta parser
    let fa_records = fasta_parser("/home/ruanad/Documents/rust-attempt/data/fake.fasta")?;
    let _ = fasta_writer("/home/ruanad/Documents/rust-attempt/data/fake_fake.fa", &fa_records);


    // test bed parser 
    let bed_records = bed_parser("/home/ruanad/Documents/rust-attempt/data/fake.bed")?;
    for record in bed_records {
        println!("{}", record);
    }
    Ok(())
}
