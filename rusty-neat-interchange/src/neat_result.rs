use std::path::Path;

use crate::{gene_pool::PrintableGenePool, genome::PrintableGenome, io::{self, FileType}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PrintableNeatResult {
    pub best_genome: PrintableGenome,
    pub best_fitness: f64,
    pub final_pool: PrintableGenePool
}

pub fn write<T: Into<PrintableNeatResult>>(
    result: T,
    path: &Path,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, result.into(), file_type)
}

pub fn read<T: From<PrintableNeatResult>>(path: &Path) -> Result<T, String> {
    io::read(path).map(|content: PrintableNeatResult| content.into())
}
