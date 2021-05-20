use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    gene_pool::PrintableGenePool,
    io::{self, FileType},
    species::PrintableSpecies,
};

#[derive(Serialize, Deserialize)]
pub struct PrintableGeneration {
    pub generation: u32,
    pub species: Vec<PrintableSpecies>,
    pub pool: PrintableGenePool,
}

pub fn write<T: Into<PrintableGeneration>>(
    generation: T,
    path: &Path,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, generation.into(), file_type)
}

pub fn read<T: From<PrintableGeneration>>(path: &Path) -> Result<T, String> {
    io::read(path).map(|content: PrintableGeneration| content.into())
}
