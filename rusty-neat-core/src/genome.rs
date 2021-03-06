use core::fmt;
use serde::{Deserialize, Serialize};
use std::{
    cmp::max,
    collections::BTreeMap,
    usize,
};

use crate::{
    activation::Activation,
    config_util::{self, NormalDistribution},
    gene_pool::{Connection, GenePool, NodeType},
};
use hashbrown::HashMap;
use rand::{prelude::SliceRandom, Rng};
use rand_distr::Distribution;
use rusty_neat_interchange::genome::{PrintableConnectionGene, PrintableGenome};

pub(crate) struct GenomeIdGenerator {
    next_id: u64,
}

impl GenomeIdGenerator {
    pub(crate) fn new() -> Self {
        GenomeIdGenerator { next_id: 0 }
    }

    pub(crate) fn next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }
}

#[derive(Debug, Clone)]
pub struct Genome {
    id: u64,
    input_node_count: usize,
    output_node_count: usize,
    connections: Vec<ConnectionGene>,
    connection_mappings: BTreeMap<usize, usize>, // innovation -> Index in connections
    nodes: Vec<NodeGene>, // Die ersten Nodes sind die Input Nodes, darauf folgen die Output Nodes, und dann die Hidden nodes
    node_mappings: HashMap<usize, usize>, // node_id -> gene_id
    input_nodes: Vec<usize>, // Index der Inputs in nodes (in der gleichen Reihenfolge wie bei evaluate)
    output_nodes: Vec<usize>, // Index der Outputs in nodes (in der gleichen Reihenfolge wie bei evaluate)
    next_iteration: u64,
    generation: u32, // Die Generation, in der das Genom erstellt wurde
}

impl Genome {
    pub fn new(
        id: u64,
        generation: u32,
        input_node_count: usize,
        output_node_count: usize,
    ) -> Genome {
        let mut genome = Genome {
            id,
            input_node_count,
            output_node_count,
            connections: Vec::new(),
            connection_mappings: BTreeMap::new(),
            nodes: Vec::new(),
            node_mappings: HashMap::new(),
            next_iteration: 1,
            input_nodes: Vec::new(),
            output_nodes: Vec::new(),
            generation,
        };
        for i in 0..input_node_count + output_node_count {
            genome.add_node(i);
        }

        genome
    }

    pub fn from_genome(genome: &Genome, id: u64, generation: u32) -> Genome {
        let mut genome = genome.clone();
        genome.id = id;
        genome.generation = generation;
        genome
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    pub fn enabled_connection_count(&self) -> usize {
        self.connections
            .iter()
            .filter(|connection| connection.enabled)
            .count()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn add_node(&mut self, id: usize) {
        if !self.node_mappings.contains_key(&id) {
            self.nodes.push(NodeGene {
                node_id: id,
                incoming_connections: Vec::new(),
                evaluation: EvaluationValue {
                    iteration: 0,
                    value: 0.0,
                },
            });
            self.node_mappings.insert(id, self.nodes.len() - 1);
        }
    }

    pub fn add_new_connection(
        &mut self,
        connection: Connection,
        weight_strategy: &NewConnectionWeight,
    ) {
        let weight = NewConnectionWeight::sample_weight(weight_strategy);
        self.add_connection(connection, weight, true);
    }

    // Falls die Enden der Connection noch nicht vorhanden sind, werden diese hinzugef??gt
    fn add_connection(&mut self, connection: Connection, weight: f64, enabled: bool) {
        self.add_node(connection.from);
        self.add_node(connection.to);

        self.connections.push(ConnectionGene {
            innovation: connection.innovation,
            weight,
            from: *self.node_mappings.get(&connection.from).unwrap(),
            to: *self.node_mappings.get(&connection.to).unwrap(),
            enabled,
        });
        let index = self.connections.len() - 1;
        self.connection_mappings
            .insert(connection.innovation, index);
        self.nodes[*self.node_mappings.get(&connection.to).unwrap()]
            .incoming_connections
            .push(index);
    }

    pub fn evaluate(&mut self, input: &[f64], config: &EvaluationConfig) -> Vec<f64> {
        for (i, value) in input.iter().enumerate() {
            self.nodes[i].evaluation = EvaluationValue {
                iteration: self.next_iteration,
                value: *value,
            }
        }
        let mut result = Vec::with_capacity(self.output_node_count);
        for i in input.len()..input.len() + self.output_node_count {
            result.push(self.evaluate_node(i, input, config));
        }
        self.next_iteration += 1;
        result
    }

    // node_id bezieht sich auf den Index im Genome
    fn evaluate_node(&mut self, node_id: usize, input: &[f64], config: &EvaluationConfig) -> f64 {
        if self.nodes[node_id].evaluation.iteration == self.next_iteration {
            self.nodes[node_id].evaluation.value
        } else {
            let mut value = 0.0;
            for i in 0..self.nodes[node_id].incoming_connections.len() {
                let connection = self.nodes[node_id].incoming_connections[i];
                if self.connections[connection].enabled {
                    let from = self.connections[connection].from;
                    let weight = self.connections[connection].weight;
                    value += weight * self.evaluate_node(from, input, config);
                }
            }
            value += config.bias;
            value = (config.activation.function())(value);
            self.nodes[node_id].evaluation.iteration = self.next_iteration;
            self.nodes[node_id].evaluation.value = value;
            value
        }
    }

    pub fn distance(&self, other: &Genome, config: &DistanceConfig) -> f64 {
        let mut disjoint = 0;
        let mut similar = 0;
        let mut weight_difference = 0.0;

        let mut my_connections = self.connection_mappings.keys().peekable();
        let mut other_connections = other.connection_mappings.keys().peekable();

        while my_connections.peek().is_some() && other_connections.peek().is_some() {
            if my_connections.peek().eq(&other_connections.peek()) {
                // Similar
                let my_gene = &self.connections[*self
                    .connection_mappings
                    .get(my_connections.next().unwrap())
                    .unwrap()];
                let other_gene = &other.connections[*other
                    .connection_mappings
                    .get(other_connections.next().unwrap())
                    .unwrap()];
                weight_difference += (my_gene.weight - other_gene.weight).abs();
                similar += 1;
            } else {
                // Disjoint
                if my_connections.peek().lt(&other_connections.peek()) {
                    my_connections.next();
                } else {
                    other_connections.next();
                }
                disjoint += 1;
            }
        }

        // Excess
        let excess = max(my_connections.count(), other_connections.count());

        // Casts zu floats
        let disjoint = disjoint as f64;
        let similar = similar as f64;
        let excess = excess as f64;
        let n = max(self.connections.len(), other.connections.len()) as f64;

        (disjoint * config.c1 + excess * config.c2) / n + weight_difference / similar * config.c3
    }

    pub fn mutate(
        &mut self,
        pool: &mut GenePool,
        config: &MutationConfig,
        new_id: u64,
        new_generation: u32,
    ) {
        self.id = new_id;
        self.generation = new_generation;
        self.mutate_connections(config);

        if rand::thread_rng().gen_bool(config.add_node_prob) {
            self.mutate_add_node(pool, config);
        }

        if rand::thread_rng().gen_bool(config.add_connection_prob) {
            self.mutate_add_connection(pool, config);
        }
    }

    fn mutate_connections(&mut self, config: &MutationConfig) {
        let rng = &mut rand::thread_rng();
        if rng.gen_bool(config.change_weight_prob) {
            self.connections.iter_mut().for_each(|connection| {
                if rng.gen_bool(config.toggle_connection_prob) {
                    connection.enabled = !connection.enabled;
                } else if connection.enabled {
                    if rng.gen_bool(config.shift_weight_prob) {
                        connection.weight += config.shift_weight_dist.to_dist().sample(rng);
                    } else {
                        connection.weight = config.random_weight_dist.to_dist().sample(rng);
                    }
                }
            });
        }
    }

    fn mutate_add_node(&mut self, pool: &mut GenePool, config: &MutationConfig) {
        // Falls das Genome keine Connections enth??lt, kann auch keine Node hinzugef??gt werden
        if self.connections.is_empty() {
            return;
        }

        // Zuf??llige Connection ausw??hlen
        let index = rand::thread_rng().gen_range(0..self.connections.len());
        let connection = &self.connections[index];

        let from = self.nodes[connection.from].node_id;
        let to = self.nodes[connection.to].node_id;
        let old_connection_weight = connection.weight;
        let old_connection_enabled = connection.enabled;

        // Neue Node erstellen und zum Genome hinzuf??gen
        let new_node = pool.create_hidden_node_between(from, to);
        self.add_node(new_node);

        // Connection vom alten from zur neuen Node erstellen
        let left_connection = pool.create_connection(from, new_node).unwrap();
        self.add_connection(
            left_connection,
            old_connection_weight,
            old_connection_enabled,
        );

        // Connection von der neuen Node zum alten to erstellen
        let right_connection = pool.create_connection(new_node, to).unwrap();
        self.add_new_connection(right_connection, &config.new_connection_weight);

        // Die alte Connection muss nicht (und darf nicht, falls sie sp??ter wieder enabled wird!)
        // aus den incoming_connections von to entfernt werden. Sie wird stattdessen disabled.
        self.connections[index].enabled = false;
    }

    fn mutate_add_connection(&mut self, pool: &mut GenePool, config: &MutationConfig) {
        if self.nodes.len() <= 1 {
            return;
        }

        for _ in 0..config.add_connection_retry_count {
            let a = self.nodes.choose(&mut rand::thread_rng()).unwrap();
            let b = self.nodes.choose(&mut rand::thread_rng()).unwrap();

            // Falls a links von b oder b links von a ist, funktioniert einer der F??lle
            if let Some(connection) = pool.create_connection(a.node_id, b.node_id) {
                // Passende Nodes gefunden
                if let Some(index) = self.connection_mappings.get(&connection.innovation) {
                    if !self.connections[*index].enabled {
                        self.connections[*index].enabled = true;
                    } else {
                        continue;
                    }
                } else {
                    self.add_new_connection(connection, &config.new_connection_weight);
                }
                break;
            } else if let Some(connection) = pool.create_connection(b.node_id, a.node_id) {
                // Passende Nodes gefunden
                if let Some(index) = self.connection_mappings.get(&connection.innovation) {
                    if !self.connections[*index].enabled {
                        self.connections[*index].enabled = true;
                    } else {
                        continue;
                    }
                } else {
                    self.add_new_connection(connection, &config.new_connection_weight);
                }
                break;
            }
        }
    }

    fn get_connection_from_innovation(&self, innovation: usize) -> &ConnectionGene {
        &self.connections[*self.connection_mappings.get(&innovation).unwrap()]
    }

    pub fn crossover(
        &self,
        other: &Genome,
        pool: &GenePool,
        config: &CrossoverConfig,
        offspring_id: u64,
        offspring_generation: u32,
    ) -> Genome {
        let mut offspring = Genome::new(
            offspring_id,
            offspring_generation,
            self.input_node_count,
            self.output_node_count,
        );

        let mut my_connections = self.connection_mappings.keys().peekable();
        let mut other_connections = other.connection_mappings.keys().peekable();

        while my_connections.peek().is_some() && other_connections.peek().is_some() {
            if my_connections.peek().eq(&other_connections.peek()) {
                // Similar
                let my_gene = self.get_connection_from_innovation(*my_connections.next().unwrap());
                let other_gene =
                    other.get_connection_from_innovation(*other_connections.next().unwrap());
                crossover_similar(my_gene, other_gene, &mut offspring, pool, config);
            } else {
                // Disjoint
                if my_connections.peek().lt(&other_connections.peek()) {
                    let my_gene =
                        self.get_connection_from_innovation(*my_connections.next().unwrap());
                    offspring.add_connection(
                        pool.connections[my_gene.innovation],
                        my_gene.weight,
                        my_gene.enabled,
                    );
                } else {
                    let other_gene =
                        other.get_connection_from_innovation(*other_connections.next().unwrap());
                    offspring.add_connection(
                        pool.connections[other_gene.innovation],
                        other_gene.weight,
                        other_gene.enabled,
                    );
                }
            }
        }

        // Excess
        if my_connections.peek().is_some() {
            for innovation in my_connections {
                let my_gene = self.get_connection_from_innovation(*innovation);
                offspring.add_connection(
                    pool.connections[my_gene.innovation],
                    my_gene.weight,
                    my_gene.enabled,
                );
            }
        } else if other_connections.peek().is_some() {
            for innovation in other_connections {
                let other_gene = other.get_connection_from_innovation(*innovation);
                offspring.add_connection(
                    pool.connections[other_gene.innovation],
                    other_gene.weight,
                    other_gene.enabled,
                );
            }
        }

        offspring
    }

    pub fn get_status_of_connection(&self, node_id: usize) -> (f64, bool) {
        let gene = &self.connections[*self.connection_mappings.get(&node_id).unwrap()];
        (gene.weight, gene.enabled)
    }

    pub fn from_printable(printable_genome: &PrintableGenome, pool: &GenePool) -> Self {
        let mut genome = Genome::new(
            printable_genome.id,
            printable_genome.generation,
            pool.input_count,
            pool.output_count,
        );

        for node in &printable_genome.nodes {
            // Skip in and out nodes
            if pool.nodes[*node as usize].node_type == NodeType::Hidden {
                genome.add_node(*node as usize);   
            }
        }

        for connection in &printable_genome.connections {
            genome.add_connection(
                pool.connections[connection.innovation as usize],
                connection.weight,
                connection.enabled,
            );
        }

        genome
    }
}

impl Into<PrintableGenome> for &Genome {
    fn into(self) -> PrintableGenome {
        let mut printable = PrintableGenome {
            id: self.id,
            generation: self.generation,
            connections: Vec::new(),
            nodes: self.node_mappings.keys().map(|n| *n as u64).collect(),
        };

        for connection in &self.connections {
            printable.connections.push(PrintableConnectionGene {
                innovation: connection.innovation as u64,
                weight: connection.weight,
                enabled: connection.enabled,
            });
        }

        printable
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct ConnectionGene {
    innovation: usize,
    from: usize, // Bezieht sich auf den Index im Genome
    to: usize,   // Bezieht sich auf den Index im Genome
    weight: f64,
    enabled: bool,
}

impl fmt::Debug for ConnectionGene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.enabled {
            write!(
                f,
                "{}:{}-({:.2})->{}",
                self.innovation, self.from, self.weight, self.to
            )
        } else {
            write!(
                f,
                "{}:{}-!({:.2})->{}",
                self.innovation, self.from, self.weight, self.to
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NodeGene {
    node_id: usize,
    #[serde(skip)]
    incoming_connections: Vec<usize>, // Bezieht sich auf den Index im Genome
    #[serde(skip)]
    evaluation: EvaluationValue,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct EvaluationValue {
    iteration: u64,
    value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DistanceConfig {
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
}

impl DistanceConfig {
    pub fn validate(&self) -> Result<(), String> {
        config_util::assert_not_negative(self.c1, "c1")
            .and(config_util::assert_not_negative(self.c2, "c2"))
            .and(config_util::assert_not_negative(self.c3, "c3"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationConfig {
    pub bias: f64,
    pub activation: Activation,
}

#[derive(Serialize, Deserialize)]
pub struct MutationConfig {
    pub change_weight_prob: f64, // Wahrscheinlichkeit, dass das Gewicht jedes einzelnen ConnectionGenes ver??ndert wird
    pub random_weight_dist: NormalDistribution, // Standardabweichung s der N(0, s)-Verteilung f??r den zuf??lligen Wert der Connection bei einem weight change ohne shift
    pub shift_weight_prob: f64, // Wahrscheinlichkeit, dass weight change das Gewicht shiftet und nicht zuf??llig neu setzt
    pub shift_weight_dist: NormalDistribution, // Standardabweichung s der N(0, s)-Verteilung f??r den shift eines weight shifts
    pub add_node_prob: f64, // Wahrscheinlichkeit, dass ein neuer Node hinzugef??gt wird
    pub add_connection_prob: f64, // Wahrscheinlichkeit, dass eine neue Connection zwischen bestehenden Nodes hinzugef??gt wird
    pub add_connection_retry_count: u32, // Anzahl der Versuche, zwei passende Nodes f??r eine neue Connnection auszulosen
    pub new_connection_weight: NewConnectionWeight, // Wie das Gewicht einer neuen Connection festgelegt werden soll.
    pub toggle_connection_prob: f64, // Wahrscheinlichkeit, dass eine zuf??llige Connection enabled bzw. disabled wird
}

impl MutationConfig {
    pub fn validate(&self) -> Result<(), String> {
        config_util::assert_probability(self.change_weight_prob, "change_weight_prob")
            .and(config_util::assert_probability(
                self.shift_weight_prob,
                "shift_weight_prob",
            ))
            .and(config_util::assert_probability(
                self.add_node_prob,
                "add_node_prob",
            ))
            .and(config_util::assert_probability(
                self.add_connection_prob,
                "add_connection_prob",
            ))
            .and(config_util::assert_probability(
                self.toggle_connection_prob,
                "toggle_connection_prob",
            ))
    }
}

#[derive(Serialize, Deserialize)]
pub enum NewConnectionWeight {
    Random(NormalDistribution),
    Fixed(f64),
}

impl NewConnectionWeight {
    fn sample_weight(strategy: &NewConnectionWeight) -> f64 {
        match strategy {
            NewConnectionWeight::Random(dist) => dist.to_dist().sample(&mut rand::thread_rng()),
            &NewConnectionWeight::Fixed(value) => value,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CrossoverConfig {
    pub disable_connection_prob: f64, // Wahrscheinlichkeit, dass eine Connection disabled wird, wenn die Connection in einem Elternteil disabled ist
    pub weight_strategy: CrossoverWeightStrategy, // Wie das Gewicht einer Connection die in beiden Eltern vorhanden ist bestimmt werden soll
}

impl CrossoverConfig {
    pub fn validate(&self) -> Result<(), String> {
        config_util::assert_probability(self.disable_connection_prob, "disable_connection_prob")
    }
}

#[derive(Serialize, Deserialize)]
pub enum CrossoverWeightStrategy {
    Random, // Gewicht von einem zuf??lligen Eltern
    Better, // Gewicht des besseren Elternteils
    Mean,   // Mittelwert der Elterngewichte
}

/*
/////////////// Sp??ter implementieren ////////////////////////////////
pub enum NETWORK_TYPE {
  FEED_FORWARD,         // Aktuelle Implementierung. Erlaubt keine R??ckw??rtskanten / Kreise
  TIMELINE,             // Der Wert einer R??ckw??rtskante ist der der letzten Auswertung des Netzwerks. Das Netzwerk muss zur??ckgesetzt werden k??nnen
  STABLILIZING          // (Originale Implementierung) R??ckw??rtskanten werden im ersten Durchlauf nicht beachtet. Das Netzwerk wird solange erneut
                        // ausgewertet, bis jede Kante berechnet werden konnte
}
*/

///////////////////////////////////////// Crossovers ////////////////////////////////////////////////////////7
fn crossover_similar(
    better_gene: &ConnectionGene,
    worse_gene: &ConnectionGene,
    offspring: &mut Genome,
    pool: &GenePool,
    config: &CrossoverConfig,
) {
    let weight = match config.weight_strategy {
        CrossoverWeightStrategy::Random => {
            if rand::thread_rng().gen_bool(0.5) {
                better_gene.weight
            } else {
                worse_gene.weight
            }
        }
        CrossoverWeightStrategy::Better => better_gene.weight,
        CrossoverWeightStrategy::Mean => (better_gene.weight + worse_gene.weight) / 2.0,
    };

    if better_gene.enabled == worse_gene.enabled {
        offspring.add_connection(
            pool.connections[better_gene.innovation],
            weight,
            better_gene.enabled,
        );
    } else {
        offspring.add_connection(
            pool.connections[better_gene.innovation],
            weight,
            rand::thread_rng().gen_bool(1.0 - config.disable_connection_prob),
        );
    }
}
