//use crate::buf_file::{bufread_file, bufwrite_file};
//use tool_biox::buf_file::{bufread_file, bufwrite_file};
use crate::buf_file::{bufread_file, bufwrite_file}; // Importez le module buf_file
use std::io::{self, BufRead, Write};
use std::fmt;

pub struct FastaRecord {
    header: String,
    sequence: String,
}

impl fmt::Display for FastaRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ">{}\n{}", self.header, self.sequence)
    }

}

impl FastaRecord {
    pub fn new(header: &str, sequence: &str) -> Self {
        FastaRecord {
            header: header.to_string(), // Convertir la référence de str en String
            sequence: sequence.to_string(), // Convertir la référence de str en String
        }
    }

}


fn add_fasta_record(records: &mut Vec<FastaRecord>, header: &str, sequence: &str) -> Result<(), io::Error> {
    if header.is_empty(){
        return Ok(());
    }
    // Créer un nouveau FastaRecord en utilisant les arguments header_str et sequence_str
    let new_record = FastaRecord::new(header, sequence) ;

    // Ajouter le nouveau FastaRecord à la liste records
    records.push(new_record);
    // Renvoyer Ok pour indiquer que l'opération s'est bien déroulée
    Ok(())
}

fn parse_fasta_reader(fasta_reader: &mut dyn BufRead) -> Result<Vec<FastaRecord>, io::Error> {
    let mut new_header = String::new();
    let mut new_sequence = String::new();
    let mut records = Vec::new();
    // Parcourir chaque ligne du fichier FASTA
    for line in fasta_reader.lines() {
        // Lire chaque ligne et ajouter son contenu à la chaîne de caractères 'content'
        let line = line?;
        if line.starts_with('>') {
            add_fasta_record(&mut records, &new_header, &new_sequence)?;
            new_header = line[1..].trim().to_string();
            new_sequence.clear();
        } else {
            new_sequence.push_str(&line.trim());
        } 
    }
    // Ajouter le dernier enregistrement après la fin de la boucle
    add_fasta_record(&mut records, &new_header, &new_sequence)?;
    // Renvoyer le contenu complet du fichier FASTA
    Ok(records)
}


pub fn fasta_parser(filename: &str)  -> Result<Vec<FastaRecord> , io::Error> {
    let mut reader = bufread_file(&filename)?;
    let fa_records = parse_fasta_reader(&mut reader)?;
    Ok(fa_records)
}

pub fn fasta_writer(filename: &str, records: &Vec<FastaRecord>) -> Result<(), io::Error> {
    let mut writer = bufwrite_file(&filename)?;
    for record in records {
        write!(writer, ">{}\n{}\n", record.header, record.sequence)?;
    }
    Ok(())
}


