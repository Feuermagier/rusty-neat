use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    genome::PrintableGenome,
    io::{self, FileType},
};
#[derive(Serialize, Deserialize)]
pub struct PrintableOrganism {
    pub genome: PrintableGenome,
    pub fitness: Option<f64>,
}

pub fn write<T: Into<PrintableOrganism>>(
    organism: T,
    path: &Path,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, organism.into(), file_type)
}

pub fn read<T: From<PrintableOrganism>>(path: &Path) -> Result<T, String> {
    io::read(path).map(|content: PrintableOrganism| content.into())
}
