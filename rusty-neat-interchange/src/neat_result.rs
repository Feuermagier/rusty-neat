use crate::{
    genome::PrintableGenome,
    io::{self, FileType},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PrintableNeatResult {
    pub best_genome: PrintableGenome,
    pub best_fitness: f64,
}

pub fn write<T: Into<PrintableNeatResult>>(
    result: T,
    path: &str,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, result.into(), file_type)
}

pub fn read<T: From<PrintableNeatResult>>(path: &str, file_type: FileType) -> Result<T, String> {
    io::read(path, file_type).map(|content: PrintableNeatResult| content.into())
}
