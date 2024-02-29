

    use std::fs::File;
    use std::io::{self, BufWriter};

    pub fn bufread_file(filename: &str) -> Result<io::BufReader<File>, io::Error> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        Ok(reader)
    }

    pub fn bufwrite_file(filename: &str) -> Result<io::BufWriter<File>, io::Error> {
        let file = File::create(filename)?; // Crée un nouveau fichier ou écrase le fichier existant
        let writer = BufWriter::new(file);
        Ok(writer)
    }

    



