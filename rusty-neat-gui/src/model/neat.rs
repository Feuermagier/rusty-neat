use std::sync::Arc;

use im::Vector;

use super::{generation::Generation, genome::Genome, result::NeatResult, species::Species};

use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct NeatModel {
    pub generations: Vector<Arc<Generation>>,
    pub current_generation: Option<Arc<Generation>>,
    pub current_species: Option<Arc<Species>>,
    pub current_genome: Option<Arc<Genome>>,
    pub result: Option<NeatResult>,
}
