use std::path::Path;

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
    pub id: usize,
}

pub fn write<T: Into<PrintableSpecies>>(
    species: T,
    path: &Path,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, species.into(), file_type)
}

pub fn read<T: From<PrintableSpecies>>(path: &Path) -> Result<T, String> {
    io::read(path).map(|content: PrintableSpecies| content.into())
}
