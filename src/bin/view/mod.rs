pub mod genome_widget;

use druid::Widget;

use genome_widget::GenomeWidget;

use self::genome_widget::Genome;

pub(crate) fn build_ui() -> impl Widget<Genome> {
  GenomeWidget { }
}