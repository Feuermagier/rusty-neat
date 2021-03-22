use std::collections::HashMap;

use crate::genome::Genome;

const OUTPUT_NODE_DEPTH: f64 = 100.0;

pub struct GenePool {
  pub nodes: Vec<Node>,
  pub connections: HashMap<(usize, usize), usize>,
  next_innovation: usize
}

impl GenePool {
  pub fn new(input_nodes: usize, output_nodes: usize) -> GenePool {
    let mut pool = GenePool {
      nodes: Vec::with_capacity(input_nodes + output_nodes),
      connections: HashMap::new(),
      next_innovation: 0
    };

    for i in 0..input_nodes {
      pool.create_node(0.0, NodeType::Input(i));
    }

    for i in 0..output_nodes {
      pool.create_node(OUTPUT_NODE_DEPTH, NodeType::Output(i));
    }

    for i in 0..input_nodes {
      for j in 0..output_nodes {
        pool.create_connection(pool.nodes[i].id, pool.nodes[input_nodes + j].id);
      }
    }

    pool
  }

  pub fn create_node(&mut self, depth: f64, node_type: NodeType) -> usize {
    let id = self.nodes.len();
    let node = Node {
      id,
      node_type,
      depth
    };
    self.nodes.push(node);
    id
  }

  pub fn create_connection(&mut self, start: usize, end: usize) -> usize {
    if let Some(innovation) = self.connections.get(&(start, end)) {
      *innovation
    } else {
      let innovation = self.next_innovation;
      self.connections.insert((start, end), innovation);
      self.next_innovation += 1;
      innovation
    }
  }

  pub fn new_genome(&self) -> Genome {
    let mut genome = Genome::new();

    for node in &self.nodes {
      genome.add_node(node.id);
    }

    for connection in &self.connections {
      genome.add_connection(connection.0.0, connection.0.1, *connection.1);
    }

    genome
  }
}

pub enum NodeType {
  Input(usize),
  Hidden,
  Output(usize)
}

// id ist immer gleich dem Index der Node im GenePool!
pub struct Node {
  pub id: usize,
  pub node_type: NodeType,
  pub depth: f64
}