use std::fs;

use crate::genome::Genome;

pub fn store_genome(path: &str, genome: &Genome, pretty: bool) -> std::io::Result<()> {
    if pretty {
        fs::write(path, serde_json::to_string_pretty(genome)?)
    } else {
        fs::write(path, serde_json::to_string(genome)?)
    }
}

pub fn read_genome(path: &str) -> std::io::Result<Genome> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}
