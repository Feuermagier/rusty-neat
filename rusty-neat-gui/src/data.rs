use crate::genome_widget::Genome;

use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct GUIModel {
  pub current_genome: Option<Genome>
}