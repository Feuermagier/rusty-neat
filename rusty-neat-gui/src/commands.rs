use std::sync::Arc;

use druid::Selector;

use crate::model::{generation::Generation, genome::Genome, species::Species};

pub const SELECT_GENERATION: Selector<Arc<Generation>> = Selector::new("select_generation");
pub const SELECT_SPECIES: Selector<Arc<Species>> = Selector::new("select_species");
pub const SELECT_GENOME: Selector<Arc<Genome>> = Selector::new("select_genome");

pub const RECENTER_GENOME: &str = "recenter_genome";
