use self::neat::NeatModel;

use druid::{Data, Lens};

pub mod species;
pub mod result;
pub mod generation;
pub mod neat;
pub mod genome;
pub mod gene_pool;
pub mod organism;

#[derive(Clone, Data, Lens)]
pub struct GUIModel {
    pub neat: Option<NeatModel>
}