use serde::{Deserialize, Serialize};

use crate::{
    io::{self, FileType},
    species::PrintableSpecies,
};

#[derive(Serialize, Deserialize)]
pub struct PrintableGeneration {
    pub generation: u32,
    pub species: Vec<PrintableSpecies>,
}

pub fn write<T: Into<PrintableGeneration>>(
    generation: T,
    path: &str,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, generation.into(), file_type)
}

pub fn read<T: From<PrintableGeneration>>(path: &str, file_type: FileType) -> Result<T, String> {
    io::read(path, file_type).map(|content: PrintableGeneration| content.into())
}
