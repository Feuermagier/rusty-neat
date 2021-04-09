use serde::{Deserialize, Serialize};

use crate::{
    io::{self, FileType},
    organism::PrintableOrganism,
};

#[derive(Serialize, Deserialize)]
pub struct PrintableSpecies {
    pub representative: PrintableOrganism,
    pub organisms: Vec<PrintableOrganism>,
    pub fitness: Option<f64>,
}

pub fn write<T: Into<PrintableSpecies>>(
    species: T,
    path: &str,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, species.into(), file_type)
}

pub fn read<T: From<PrintableSpecies>>(path: &str, file_type: FileType) -> Result<T, String> {
    io::read(path, file_type).map(|content: PrintableSpecies| content.into())
}
