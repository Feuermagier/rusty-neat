use std::sync::Arc;

use druid::{Data, Lens};
use im::Vector;
use rusty_neat_interchange::{gene_pool::PrintableGenePool, species::PrintableSpecies};

use super::organism::Organism;

#[derive(Clone, Lens, Data)]
pub struct Species {
    pub id: u32,
    pub representative: Arc<Organism>,
    pub organisms: Vector<Arc<Organism>>,
    pub best_organism: Arc<Organism>,
    pub average_fitness: f64,
    pub adjusted_fitness: Option<f64>,
}

impl From<(&PrintableSpecies, &PrintableGenePool)> for Species {
    fn from((species, pool): (&PrintableSpecies, &PrintableGenePool)) -> Self {
        let mut organisms: Vector<Arc<Organism>> = species
            .organisms
            .iter()
            .map(|o| Arc::new((o, pool).into()))
            .collect();
        organisms.sort_by(|x, y| x.cmp(y).reverse());
        let best_organism = Arc::clone(organisms.iter().max().unwrap());
        Self {
            id: species.id as u32,
            representative: Arc::new((&species.representative, pool).into()),
            organisms,
            best_organism,
            average_fitness: species
                .organisms
                .iter()
                .filter(|o| o.fitness.is_some())
                .map(|o| o.fitness.unwrap())
                .sum::<f64>()
                / species.organisms.len() as f64,
            adjusted_fitness: species.fitness,
        }
    }
}
