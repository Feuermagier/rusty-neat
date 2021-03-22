use hashbrown::HashMap;

use crate::gene_pool::{GenePool, NodeType};

pub struct Genome {
  connections: Vec<ConnectionGene>,
  nodes: Vec<NodeGene>,
  node_mappings: HashMap<usize, usize> // node_id -> gene_id
}

impl Genome {
  pub fn new() -> Genome {
    let genome =  Genome { 
      connections: Vec::new(), 
      nodes: Vec::new(),
      node_mappings: HashMap::new()
    };

    genome
  }

  pub fn add_node(&mut self, id: usize) {
    self.nodes.push(NodeGene {
      node_id: id,
      incoming_connections: Vec::new(),
      evaluation: EvaluationValue {iteration: 0, value: 0.0}
    });
    self.node_mappings.insert(id, self.nodes.len() - 1);
  }

  pub fn add_connection(&mut self, from: usize, to: usize, innovation: usize) {
    self.connections.push(ConnectionGene {
      innovation,
      weight: 1.0,
      from: *self.node_mappings.get(&from).unwrap(),
      enabled: true
    });
    self.nodes[*self.node_mappings.get(&to).unwrap()].incoming_connections.push(self.connections.len() - 1);
  }

  pub fn evaluate(&mut self, input: &Vec<f64>, iteration: u32, pool: &GenePool) -> Vec<f64> {
    for node in &mut self.nodes {
      if let NodeType::Input(i) = pool.nodes[node.node_id].node_type {
        node.evaluation = EvaluationValue {iteration, value: input[i]};
      }
    }
    let mut result = Vec::<f64>::new();
    for i in 0..self.nodes.len() {
      if let NodeType::Output(out_node_id) = pool.nodes[self.nodes[i].node_id].node_type {
        result.insert(out_node_id, self.evaluate_node(i, input, iteration));
      }
    }
    result
  }

  fn evaluate_node(&mut self, node_id: usize, input: &Vec<f64>, iteration: u32) -> f64 {
    if self.nodes[node_id].evaluation.iteration == iteration {
      self.nodes[node_id].evaluation.value
    } else {
      let mut value = 0.0;
      for i in 0..self.nodes[node_id].incoming_connections.len() {
        let connection = self.nodes[node_id].incoming_connections[i];
        if self.connections[connection].enabled {
          let from = self.connections[connection].from;
          let weight = self.connections[connection].weight;
          value += weight * self.evaluate_node(from, input, iteration);
        }
      }
      self.nodes[node_id].evaluation.iteration = iteration;
      self.nodes[node_id].evaluation.value = value;
      value
    }
  }
}

struct ConnectionGene {
  innovation: usize,
  from: usize, // Bezieht sich auf den Index im Genome
  weight: f64,
  enabled: bool
}

struct NodeGene {
  node_id: usize,
  incoming_connections: Vec<usize>,   // Bezieht sich auf den Index im Genome
  evaluation: EvaluationValue
}

struct EvaluationValue {
  iteration: u32,
  value: f64
}