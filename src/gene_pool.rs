use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::{activation::Activation, genome::Genome};

const INPUT_NODE_DEPTH: f64 = 0.0;
const OUTPUT_NODE_DEPTH: f64 = 100.0;

#[derive(Serialize, Deserialize, Debug)]
pub struct GenePool {
  pub nodes: Vec<Node>, // Liste aller Nodes
  pub connections: Vec<Connection>, // Liste aller Nodes
  #[serde(skip)]
  pub connection_mappings: HashMap<Connection, usize>, // connection -> innovation
  pub input_count: usize,
  pub output_count: usize,
  activation: Activation,
  bias: f64
}

impl GenePool {
  pub fn new(activation: Activation, bias: f64) -> GenePool {
    GenePool {
      nodes: Vec::new(),
      connection_mappings: HashMap::new(),
      connections: Vec::new(),
      input_count: 0,
      output_count: 0,
      activation,
      bias
    }
  }

  pub fn new_dense(input_nodes: usize, output_nodes: usize, activation: Activation, bias: f64) -> GenePool {
    let mut pool = GenePool::new(activation, bias);
    pool.nodes.reserve(input_nodes + output_nodes);
    pool.connections.reserve(input_nodes * output_nodes);

    for _ in 0..input_nodes {
      pool.create_input_node();
    }

    for _ in 0..output_nodes {
      pool.create_output_node();
    }

    for i in 0..input_nodes {
      for j in 0..output_nodes {
        pool.create_connection(pool.nodes[i].id, pool.nodes[input_nodes + j].id);
      }
    }

    pool
  }

  pub fn regenerate_fields(&mut self) {
    self.connection_mappings.clear();
    for i in 0..self.connections.len() {
      let connection = &self.connections[i];
      self.connection_mappings.insert(*connection, i);
    }
  }

  pub fn create_input_node(&mut self) -> usize {
    let id = self.nodes.len();
    let node = Node {
      id,
      node_type: NodeType::Input(self.input_count),
      depth: INPUT_NODE_DEPTH
    };
    self.nodes.push(node);
    self.input_count += 1;
    id
  }

  pub fn create_output_node(&mut self) -> usize {
    let id = self.nodes.len();
    let node = Node {
      id,
      node_type: NodeType::Output(self.output_count),
      depth: OUTPUT_NODE_DEPTH
    };
    self.nodes.push(node);
    self.output_count += 1;
    id
  }

  pub fn create_hidden_node(&mut self, depth: f64) -> usize {
    let id = self.nodes.len();
    let node = Node {
      id,
      node_type: NodeType::Hidden,
      depth
    };
    self.nodes.push(node);
    id
  }

  pub fn create_connection(&mut self, start: usize, end: usize) -> usize {
    let connection = Connection {start, end};
    if let Some(innovation) = self.connection_mappings.get(&connection) {
      *innovation
    } else {
      let innovation = self.connections.len();
      self.connections.push(connection);
      self.connection_mappings.insert(connection, innovation);
      innovation
    }
  }

  pub fn new_genome(&self) -> Genome {
    let mut genome = Genome::new(self.activation, self.bias);

    for node in &self.nodes {
      genome.add_node(node.id);
    }

    for i in 0..self.connections.len() {
      let connection = &self.connections[i];
      genome.add_connection(connection.start, connection.end, i);
    }

    genome
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NodeType {
  Input(usize),
  Hidden,
  Output(usize)
}

// id ist immer gleich dem Index der Node im GenePool!
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
  pub id: usize,
  pub node_type: NodeType,
  pub depth: f64
}

// innovation number entspricht dem Index im GenePool
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Connection {
  pub start: usize,
  pub end: usize
}