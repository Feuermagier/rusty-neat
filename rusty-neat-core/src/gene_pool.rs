use std::{rc::Rc, usize};

use hashbrown::HashMap;
use rusty_neat_interchange::gene_pool::{PrintableConnection, PrintableGenePool, PrintableNode, PrintableNodeType};

use crate::genome::{Genome, NewConnectionWeight};

const INPUT_NODE_DEPTH: f64 = 0.0;
const OUTPUT_NODE_DEPTH: f64 = 1.0;

#[derive(Debug)]
pub struct GenePool {
    pub nodes: Vec<Node>, // Liste aller Nodes über alle Genomes hinweg
    pub connections: Vec<Rc<Connection>>, // Liste aller Connections über alle Genomes hinweg (Index = innovation der Connection)
    pub connection_mappings: HashMap<(usize, usize), Rc<Connection>>, // (from, to) -> innovation
    pub input_count: usize,
    pub output_count: usize,
}

impl GenePool {
    pub fn new() -> GenePool {
        GenePool {
            nodes: Vec::new(),
            connection_mappings: HashMap::new(),
            connections: Vec::new(),
            input_count: 0,
            output_count: 0,
        }
    }

    pub fn from_printable(printable: &PrintableGenePool) -> Self {
        let mut pool = GenePool::new();
        pool.nodes.reserve(printable.nodes.len());
        pool.connections.reserve(printable.connections.len());

        for node in &printable.nodes {
            pool.nodes.insert(
                node.id as usize,
                Node {
                    id: node.id as usize,
                    node_type: NodeType::from(&node.node_type),
                    depth: node.depth,
                    vertical_placement: node.vertical_placement,
                },
            );
            if let PrintableNodeType::Input(_) = node.node_type {
                pool.input_count += 1;
            } else if let PrintableNodeType::Output(_) = node.node_type {
                pool.output_count += 1;
            }
        }

        for connection in &printable.connections {
            let connection = Rc::from(Connection {
                from: connection.from as usize,
                to: connection.to as usize,
                innovation: connection.innovation as usize,
            });

            pool.connections
                .insert(connection.innovation, Rc::clone(&connection));
            pool.connection_mappings
                .insert((connection.from, connection.to), Rc::clone(&connection));
        }

        pool
    }

    pub fn new_dense(input_nodes: usize, output_nodes: usize) -> GenePool {
        let mut pool = GenePool::new();
        pool.nodes.reserve(input_nodes + output_nodes);
        pool.connections.reserve(input_nodes * output_nodes);

        for i in 0..input_nodes {
            pool.create_input_node(i as f64 / input_nodes as f64);
        }

        for i in 0..output_nodes {
            pool.create_output_node(i as f64 / output_nodes as f64);
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
            self.connection_mappings
                .insert((connection.from, connection.to), Rc::clone(connection));
        }
    }

    pub fn create_input_node(&mut self, vertical_placement: f64) -> usize {
        let id = self.nodes.len();
        let node = Node {
            id,
            node_type: NodeType::Input(self.input_count),
            depth: INPUT_NODE_DEPTH,
            vertical_placement,
        };
        self.nodes.push(node);
        self.input_count += 1;
        id
    }

    pub fn create_output_node(&mut self, vertical_placement: f64) -> usize {
        let id = self.nodes.len();
        let node = Node {
            id,
            node_type: NodeType::Output(self.output_count),
            depth: OUTPUT_NODE_DEPTH,
            vertical_placement,
        };
        self.nodes.push(node);
        self.output_count += 1;
        id
    }

    pub fn create_hidden_node(&mut self, depth: f64, vertical_placement: f64) -> usize {
        let id = self.nodes.len();
        let node = Node {
            id,
            node_type: NodeType::Hidden,
            depth,
            vertical_placement,
        };
        self.nodes.push(node);
        id
    }

    pub fn create_hidden_node_between(&mut self, left_node: usize, right_node: usize) -> usize {
        let left_node = &self.nodes[left_node];
        let right_node = &self.nodes[right_node];
        let id = self.nodes.len();
        let node = Node {
            id,
            node_type: NodeType::Hidden,
            depth: (left_node.depth + right_node.depth) / 2.0,
            vertical_placement: (left_node.depth + right_node.depth) / 2.0,
        };
        self.nodes.push(node);
        id
    }

    pub fn create_connection(&mut self, from: usize, to: usize) -> Option<Rc<Connection>> {
        if let Some(connection) = self.connection_mappings.get(&(from, to)) {
            Some(Rc::clone(connection))
        } else {
            if self.nodes[from].depth >= self.nodes[to].depth {
                return None;
            }
            let connection = Rc::from(Connection {
                from,
                to,
                innovation: self.connections.len(),
            });
            self.connections.push(Rc::clone(&connection));
            self.connection_mappings
                .insert((from, to), Rc::clone(&connection));
            Some(connection)
        }
    }

    pub fn new_genome(&self, weight_strategy: &NewConnectionWeight, id: u64, generation: u32) -> Genome {
        let mut genome = Genome::new(id, generation);

        for node in &self.nodes {
            genome.add_node(node.id);
        }

        self.connections.iter().for_each(|connection| {
            genome.add_new_connection(Rc::clone(connection), weight_strategy)
        });

        genome
    }
}

impl Into<PrintableGenePool> for &GenePool {
    fn into(self) -> PrintableGenePool {
        PrintableGenePool {
            nodes: self.nodes.iter().map(|n|n.into()).collect(),
            connections: self.connections.iter().map(|c|c.as_ref().into()).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum NodeType {
    Input(usize),
    Hidden,
    Output(usize),
}

impl From<&PrintableNodeType> for NodeType {
    fn from(printable: &PrintableNodeType) -> Self {
        match printable {
            PrintableNodeType::Input(id) => NodeType::Input(*id),
            PrintableNodeType::Hidden => NodeType::Hidden,
            PrintableNodeType::Output(id) => NodeType::Output(*id),
        }
    }
}

impl Into<PrintableNodeType> for &NodeType {
    fn into(self) -> PrintableNodeType {
        match self {
            NodeType::Input(id) => PrintableNodeType::Input(*id),
            NodeType::Hidden => PrintableNodeType::Hidden,
            NodeType::Output(id) => PrintableNodeType::Output(*id),
        }
    }
}

// id ist immer gleich dem Index der Node im GenePool!
#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    pub depth: f64,
    pub vertical_placement: f64,
}

impl From<&PrintableNode> for Node {
    fn from(printable: &PrintableNode) -> Self {
        Self {
            id: printable.id as usize,
            node_type: (&printable.node_type).into(),
            depth: printable.depth,
            vertical_placement: printable.vertical_placement,
        }
    }
}

impl Into<PrintableNode> for &Node {
    fn into(self) -> PrintableNode {
        PrintableNode {
            id: self.id as u64,
            node_type: (&self.node_type).into(),
            depth: self.depth,
            vertical_placement: self.vertical_placement,
        }
    }
}

// innovation number entspricht dem Index im GenePool
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
    pub innovation: usize,
}

impl From<&PrintableConnection> for Connection {
    fn from(printable: &PrintableConnection) -> Self {
        Self {
            from: printable.from as usize,
            to: printable.to as usize,
            innovation: printable.innovation as usize,
        }
    }
}

impl Into<PrintableConnection> for &Connection {
    fn into(self) -> PrintableConnection {
        PrintableConnection {
            innovation: self.innovation as u64,
            from: self.from as u64,
            to: self.to as u64,
        }
    }
}