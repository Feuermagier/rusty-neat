use self::neat::NeatModel;

use druid::{Data, Lens};

pub mod gene_pool;
pub mod generation;
pub mod genome;
pub mod neat;
pub mod organism;
pub mod result;
pub mod species;

#[derive(Clone, Data, Lens)]
pub struct GUIModel {
    pub neat: Option<NeatModel>,
}
