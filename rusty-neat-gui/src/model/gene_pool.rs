use druid::{Data, Lens};
use rusty_neat_interchange::gene_pool::PrintableGenePool;

/// Only statistics
#[derive(Clone, Lens, Data)]
pub struct GenePool {
  pub nodes: u64,
  pub connections: u64
}

impl From<PrintableGenePool> for GenePool {
    fn from(printable: PrintableGenePool) -> Self {
        Self {
            nodes: printable.nodes.len() as u64,
            connections: printable.connections.len() as u64
        }
    }
}