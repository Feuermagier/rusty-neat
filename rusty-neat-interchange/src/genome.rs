use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::io::{self, FileType};
#[derive(Serialize, Deserialize)]
pub struct PrintableGenome {
    pub connections: Vec<PrintableConnectionGene>,
    pub nodes: Vec<u64>,
    pub id: u64,
    pub generation: u32
}

#[derive(Serialize, Deserialize)]
pub struct PrintableConnectionGene {
    pub innovation: u64,
    pub weight: f64,
    pub enabled: bool,
}

pub fn write<T: Into<PrintableGenome>>(
    genome: T,
    path: &Path,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, genome.into(), file_type)
}

pub fn read<T: From<PrintableGenome>>(path: &Path) -> Result<T, String> {
    io::read(path).map(|content: PrintableGenome| content.into())
}
