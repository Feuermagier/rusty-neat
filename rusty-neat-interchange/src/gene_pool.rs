use serde::{Deserialize, Serialize};

use crate::io::{self, FileType};
#[derive(Serialize, Deserialize)]
pub struct PrintableGenePool {
    pub nodes: Vec<PrintableNode>,
    pub connections: Vec<PrintableConnection>,
}

#[derive(Serialize, Deserialize)]
pub struct PrintableNode {
    pub id: usize,
    pub node_type: PrintableNodeType,
    pub depth: f64,
    pub vertical_placement: f64,
}

#[derive(Serialize, Deserialize)]
pub struct PrintableConnection {
    pub innovation: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PrintableNodeType {
    Input(usize),
    Hidden,
    Output(usize),
}

pub fn write<T: Into<PrintableGenePool>>(
    pool: T,
    path: &str,
    file_type: FileType,
) -> Result<(), String> {
    io::write(path, pool.into(), file_type)
}

pub fn read<T: From<PrintableGenePool>>(path: &str, file_type: FileType) -> Result<T, String> {
    io::read(path, file_type).map(|content: PrintableGenePool| content.into())
}
