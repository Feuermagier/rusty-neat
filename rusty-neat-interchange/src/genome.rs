use serde::{Deserialize, Serialize};

use crate::io::{self, FileType};
#[derive(Serialize, Deserialize)]
pub struct PrintableGenome {
    pub connections: Vec<PrintableConnectionGene>,
    pub nodes: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct PrintableConnectionGene {
    pub innovation: usize,
    pub weight: f64,
    pub enabled: bool,
}

pub fn write<T: Into<PrintableGenome>>(
    genome: T,
    path: &str,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, genome.into(), file_type)
}

pub fn read<T: From<PrintableGenome>>(path: &str, file_type: FileType) -> Result<T, String> {
    io::read(path, file_type).map(|content: PrintableGenome| content.into())
}
