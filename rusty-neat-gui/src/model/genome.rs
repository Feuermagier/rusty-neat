use std::rc::Rc;

use druid::{Data, Lens, Point, Size};
use im::{HashMap, Vector};
use rusty_neat_interchange::{
    gene_pool::{PrintableGenePool, PrintableNodeType},
    genome::PrintableGenome,
};

#[derive(Clone, Lens, Data)]
pub struct Genome {
    pub id: u64,
    pub generation: u32,
    pub nodes: Vector<Rc<Node>>,
    pub connections: Vector<Rc<Connection>>,
}

impl From<(&PrintableGenome, &PrintableGenePool)> for Genome {
    fn from((printable, pool): (&PrintableGenome, &PrintableGenePool)) -> Self {
        let mut genome = Self {
            id: printable.id,
            generation: printable.generation,
            nodes: Vector::new(),
            connections: Vector::new(),
        };

        let mut node_map = HashMap::new();

        for node in &printable.nodes {
            let printable_node = &pool.nodes[*node as usize];
            node_map.insert(
                node,
                Rc::from(Node {
                    id: printable_node.id,
                    position: Point::new(printable_node.depth, printable_node.vertical_placement),
                    activation: "TODO".to_owned(),
                    bias: 0.0,
                    node_type: Rc::from(printable_node.node_type.clone()),
                }),
            );

            genome
                .nodes
                .push_back(Rc::clone(node_map.get(&node).unwrap()));
        }

        for connection in &printable.connections {
            genome.connections.push_back(Rc::from(Connection {
                start: Rc::clone(
                    node_map
                        .get(&pool.connections[connection.innovation as usize].from)
                        .unwrap(),
                ),
                end: Rc::clone(
                    node_map
                        .get(&pool.connections[connection.innovation as usize].to)
                        .unwrap(),
                ),
                innovation: connection.innovation,
                enabled: connection.enabled,
                weight: connection.weight,
            }));
        }

        genome
    }
}

impl PartialEq for Genome {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Data)]
pub struct Node {
    pub id: u64,
    pub position: Point,
    pub activation: String,
    pub bias: f64,
    pub node_type: Rc<PrintableNodeType>,
}

impl Node {
    pub fn actual_position(&self, bounding_rect: Size, offset: f64) -> Point {
        Point::new(
            offset + self.position.x * (bounding_rect.width - 2.0 * offset),
            offset + self.position.y * (bounding_rect.height - 2.0 * offset),
        )
    }
}

#[derive(Clone, Data)]
pub struct Connection {
    pub start: Rc<Node>,
    pub end: Rc<Node>,
    pub innovation: u64,
    pub enabled: bool,
    pub weight: f64,
}

pub enum NodeType {
    Input(u32),
    Hidden,
    Output(u32),
}

impl From<PrintableNodeType> for NodeType {
    fn from(printable: PrintableNodeType) -> Self {
        match printable {
            PrintableNodeType::Input(id) => NodeType::Input(id as u32),
            PrintableNodeType::Hidden => NodeType::Hidden,
            PrintableNodeType::Output(id) => NodeType::Output(id as u32),
        }
    }
}
