use std::io::{self, BufRead};
//use crate::buf_file::{bufread_file, bufwrite_file};
//use tool_biox::buf_file::{bufread_file, bufwrite_file};
use crate::buf_file::{bufread_file}; // Importez le module buf_file
use std::fmt;

pub struct BedRecord {
    chromosome: String,
    start_pos: usize,
    end_pos: usize,
}

impl fmt::Display for BedRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.chromosome, self.start_pos, self.end_pos)
    }

}

impl BedRecord {
    pub fn new(chromosome: &str, start_pos: &str, end_pos: &str) -> Self {
        BedRecord {
            chromosome: chromosome.to_string(),
            start_pos: start_pos.parse::<usize>().unwrap(), // Convertir str en usize
            end_pos: end_pos.parse::<usize>().unwrap(), // Convertir str en usize
        }
    }
}

fn add_bed_record(records: &mut Vec<BedRecord>, chromosome: &str, start_pos: &str, end_pos: &str) -> Result<(), io::Error> {
    let new_record = BedRecord::new(chromosome, start_pos, end_pos);
    records.push(new_record);
    Ok(())
}

fn parse_bed_reader(bed_reader: &mut dyn BufRead) -> Result<Vec<BedRecord>, io::Error> {
    let mut records = Vec::new();

    for line in bed_reader.lines() {
        let line = line?;
        let line_splitted: Vec<&str> = line.split('\t').collect();
        let chromosome = line_splitted[0].trim();
        let start_pos = line_splitted[1].trim();
        let end_pos = line_splitted[2].trim();
        add_bed_record(&mut records, chromosome, start_pos, end_pos)?;
    }
    
    Ok(records)
}

pub fn bed_parser(filename: &str) -> Result<Vec<BedRecord>, io::Error> {
    let mut reader = bufread_file(&filename)?;
    let bed_records = parse_bed_reader(&mut reader)?;
    Ok(bed_records)
}
