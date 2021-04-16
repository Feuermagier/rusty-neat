use std::sync::Arc;

use druid::Widget;

use crate::{genome_widget::GenomeWidget, model::genome::Genome};

pub fn genome() -> impl Widget<Arc<Genome>> {
    GenomeWidget::new()
}
