use std::ops::{Deref, DerefMut};

use rand::{seq::SliceRandom, thread_rng, rngs::ThreadRng, Rng};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeType {
    Gateway,
    Constrained,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EnergyConsumptionType {
    pub constrained: f32,
    pub gateway: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CommunicationOverheadType {
    pub constrained: f32,
    pub gateway: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EnergyType {
  pub compromised: EnergyConsumptionType,
  pub leaving: EnergyConsumptionType,
  pub draining: EnergyConsumptionType,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CommunicationType {
  pub compromised: CommunicationOverheadType,
  pub leaving: CommunicationOverheadType,
  pub draining: CommunicationOverheadType,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MetricsType {
  pub energy: EnergyType,
  pub communication: CommunicationType,
}

impl EnergyType {
    pub fn new(compromised_constrained: f32, compromised_gateway: f32, leaving_constrained: f32, leaving_gateway: f32, draining_constrained: f32, draining_gateway: f32) -> Self {
        Self {
            compromised: EnergyConsumptionType { constrained: compromised_constrained, gateway: compromised_gateway },
            leaving: EnergyConsumptionType { constrained: leaving_constrained, gateway: leaving_gateway },
            draining: EnergyConsumptionType { constrained: draining_constrained, gateway: draining_gateway },
        }
    }
}

impl CommunicationType {
    pub fn new(compromised_constrained: f32, compromised_gateway: f32, leaving_constrained: f32, leaving_gateway: f32, draining_constrained: f32, draining_gateway: f32) -> Self {
        Self {
            compromised: CommunicationOverheadType { constrained: compromised_constrained, gateway: compromised_gateway },
            leaving: CommunicationOverheadType { constrained: leaving_constrained, gateway: leaving_gateway },
            draining: CommunicationOverheadType { constrained: draining_constrained, gateway: draining_gateway },
        }
    }
}

impl MetricsType {
    pub fn new(energy: EnergyType, communication: CommunicationType) -> Self {
        Self {
            energy,
            communication,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeStatus {
    Compromised,
    Leaving,
    Draining,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MetricsFor {
    Constrained,
    Gateway,
    All
}

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub kind: NodeType,
    pub neighbors: Vec<usize>,
    pub max_possible_neighbors: usize,
    pub is_compromised: bool,
    pub is_leaving: bool,
    pub is_draining: bool,
}

// Create a trait for energy consumption in case of compromised node and another trait for energy consumption in case of leaving node, and another one for energy consumption in case of draining node
trait EnergyConsumption<T, M> {
    fn energy_consumption(&self, status: NodeStatus, metrics: M) -> f32;
}

trait CommunicationOverhead<T, M> {
    fn communication_overhead(&self, status: NodeStatus, metrics: M) -> f32;
}

pub trait TotalEnergyConsumption<M> {
    fn total_energy_consumption(&self, status: NodeStatus, metrics_for: MetricsFor, metrics: M) -> f32;
}

pub trait TotalCommunicationOverhead<M> {
    fn total_communication_overhead(&self, status: NodeStatus, metrics_for: MetricsFor, metrics: M) -> f32;
}

// Implement TotalEnergyConsumption trait and TotalCommunicationOverhead trait for NodesVec in one block
impl TotalEnergyConsumption<MetricsType> for NodesVec {
    fn total_energy_consumption(&self, status: NodeStatus, metrics_for: MetricsFor, metrics: MetricsType) -> f32 {
        let mut total_energy_consumption = 0.0;
        let filtered_nodes: _ = match metrics_for {
            MetricsFor::Constrained => {
                let constrained_nodes: Vec<&Node> = self.iter().filter(|node| node.kind == NodeType::Constrained).clone().collect();
                constrained_nodes
            },
            MetricsFor::Gateway => {
                self.iter().filter(|node| node.kind == NodeType::Gateway).clone().collect()
            },
            MetricsFor::All => {
                self.iter().collect()
            }
        };
        for node in filtered_nodes.iter() {
            if metrics_for == MetricsFor::Constrained && node.kind == NodeType::Constrained {
                total_energy_consumption += node.energy_consumption(status, metrics);
            } else if metrics_for == MetricsFor::Gateway && node.kind == NodeType::Gateway {
                total_energy_consumption += node.energy_consumption(status, metrics);
            } else if metrics_for == MetricsFor::All {
                total_energy_consumption += node.energy_consumption(status, metrics);
            }
            total_energy_consumption += node.energy_consumption(status, metrics);
        }
        total_energy_consumption
    }
}

impl TotalCommunicationOverhead<MetricsType> for NodesVec {
    fn total_communication_overhead(&self, status: NodeStatus, metrics_for: MetricsFor, metrics: MetricsType) -> f32 {
        let mut total_communication_overhead = 0.0;
        let filtered_nodes: _ = match metrics_for {
            MetricsFor::Constrained => {
                let constrained_nodes: Vec<&Node> = self.iter().filter(|node| node.kind == NodeType::Constrained).clone().collect();
                constrained_nodes
            },
            MetricsFor::Gateway => {
                self.iter().filter(|node| node.kind == NodeType::Gateway).clone().collect()
            },
            MetricsFor::All => {
                self.iter().collect()
            }
        };
        for node in filtered_nodes.iter() {
            if metrics_for == MetricsFor::Constrained && node.kind == NodeType::Constrained {
                total_communication_overhead += node.communication_overhead(status, metrics);
            } else if metrics_for == MetricsFor::Gateway && node.kind == NodeType::Gateway {
                total_communication_overhead += node.communication_overhead(status, metrics);
            } else if metrics_for == MetricsFor::All {
                total_communication_overhead += node.communication_overhead(status, metrics);
            }
            total_communication_overhead += node.communication_overhead(status, metrics);
        }
        total_communication_overhead
    }
}

impl EnergyConsumption<Node, MetricsType> for Node {
    fn energy_consumption(&self, status: NodeStatus, metrics: MetricsType) -> f32 {
        match status {
            NodeStatus::Compromised => {
                match self.kind {
                    NodeType::Gateway => 0.0,
                    NodeType::Constrained => 0.0,
                }
            }
            NodeStatus::Leaving => {
                match self.kind {
                    NodeType::Gateway => 0.0,
                    NodeType::Constrained => 0.0,
                }
            }
            NodeStatus::Draining => {
                match self.kind {
                    NodeType::Gateway => 0.0,
                    NodeType::Constrained => 0.0,
                }
            }
        }
    }
}

impl CommunicationOverhead<Node, MetricsType> for Node {
    fn communication_overhead(&self, status: NodeStatus, metrics: MetricsType) -> f32 {
        match status {
            NodeStatus::Compromised => {
                match self.kind {
                    NodeType::Gateway => 0.0,
                    NodeType::Constrained => 0.0,
                }
            }
            NodeStatus::Leaving => {
                match self.kind {
                    NodeType::Gateway => 0.0,
                    NodeType::Constrained => 0.0,
                }
            }
            NodeStatus::Draining => {
                match self.kind {
                    NodeType::Gateway => 0.0,
                    NodeType::Constrained => 0.0,
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct NodesVec(Vec<Node>);

impl NodesVec {


    fn new() -> Self {
        Self(Vec::new())
    }

    pub fn compromise_nodes(&mut self, number_of_nodes_to_compromise: usize) -> &mut Self {
        let mut rng: ThreadRng = thread_rng();
        // Select number_of_nodes_to_compromise random nodes and compromise them
        let mut compromised_nodes: Vec<usize> = (0..self.len()).collect();
        compromised_nodes.shuffle(&mut rng);
        compromised_nodes.truncate(number_of_nodes_to_compromise);
        for node_id in compromised_nodes {
            self[node_id].is_compromised = true;
        }
        self
    }

    pub fn compromised_nodes(&self) -> NodesVec {
        let compromised_vec =  self.iter().filter(|node| node.is_compromised).collect::<Vec<&Node>>();
        let mut compromised_nodes: NodesVec = NodesVec::new();
        for node in compromised_vec {
            compromised_nodes.push(Node::new(node.id, node.kind, node.neighbors.clone(), node.max_possible_neighbors));
        }
        compromised_nodes
    }

    pub fn leave_nodes(&mut self, number_of_nodes_to_leave: usize) -> &mut Self {
        let mut rng: ThreadRng = thread_rng();
        // Select number_of_nodes_to_leave random nodes and leave them
        let mut leaving_nodes: Vec<usize> = (0..self.len()).collect();
        leaving_nodes.shuffle(&mut rng);
        leaving_nodes.truncate(number_of_nodes_to_leave);
        for node_id in leaving_nodes {
            self[node_id].is_leaving = true;
        }
        self
    }

    pub fn drain_nodes(&mut self, number_of_nodes_to_drain: usize) -> &mut Self {
        let mut rng: ThreadRng = thread_rng();
        // Select number_of_nodes_to_drain random nodes and drain them
        let mut drained_nodes: Vec<usize> = (0..self.len()).collect();
        drained_nodes.shuffle(&mut rng);
        drained_nodes.truncate(number_of_nodes_to_drain);
        for node_id in drained_nodes {
            self[node_id].is_draining = true;
        }
        self
    }

    pub fn drained_nodes(&self) -> NodesVec {
        let drained_vec =  self.iter().filter(|node| node.is_draining).collect::<Vec<&Node>>();
        let mut drained_nodes: NodesVec = NodesVec::new();
        for node in drained_vec {
            drained_nodes.push(Node::new(node.id, node.kind, node.neighbors.clone(), node.max_possible_neighbors));
        }
        drained_nodes
    }

    pub fn left_nodes(&self) -> NodesVec {
        let leaving_vec =  self.iter().filter(|node| node.is_leaving).collect::<Vec<&Node>>();
        let mut leaving_nodes: NodesVec = NodesVec::new();
        for node in leaving_vec {
            leaving_nodes.push(Node::new(node.id, node.kind, node.neighbors.clone(), node.max_possible_neighbors));
        }
        leaving_nodes
    }

    pub fn reset(&mut self) -> &mut Self {
        for node in self.iter_mut() {
            node.is_compromised = false;
            node.is_leaving = false;
            node.is_draining = false;
        }
        self
    }

}

impl Deref for NodesVec {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodesVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Node {
    fn new(id: usize, kind: NodeType, neighbors: Vec<usize>, max_possible_neighbors: usize) -> Self {
        Self {
            id,
            kind,
            neighbors,
            max_possible_neighbors,
            is_compromised: false,
            is_leaving: false,
            is_draining: false,
        }
    }
}

pub fn initialize_network(number_of_nodes: i32, number_of_gateways: i32, number_of_min_possible_neighbors: i32, number_of_max_possible_neighbors: i32) -> NodesVec {
    let mut nodes: NodesVec = NodesVec::new();

    let mut rng: ThreadRng = thread_rng();

    // Push gateway nodes
    for _ in 0..number_of_gateways {
        let number_of_neighbors: i32 = rng.gen_range(number_of_min_possible_neighbors..=number_of_max_possible_neighbors);
        let node = Node::new(nodes.len(), NodeType::Gateway, vec![], number_of_neighbors as usize);
        nodes.push(node);
    }

    // Push constrained nodes
    for _ in 0..(number_of_nodes - number_of_gateways) {
        let number_of_neighbors: i32 = rng.gen_range(number_of_min_possible_neighbors..=number_of_max_possible_neighbors);
        let node = Node::new(nodes.len(), NodeType::Constrained, vec![], number_of_neighbors as usize);
        nodes.push(node);
    }

    // Sort nodes randomly
    nodes.shuffle(&mut rng);

    // Update id of each node to match its index in the nodes array
    for i in 0..nodes.len() {
        nodes[i].id = i;
    }

    // Start adding neighbors to each node
    for i in 0..nodes.len() {
        let current_node_id: usize = nodes[i].id;
        let number_of_current_neighbors: usize = nodes[i].neighbors.len();
        let number_of_current_max_possible_neighbors: usize = nodes[i].max_possible_neighbors;
        let number_of_current_remaining_possible_neighbors: usize = number_of_current_max_possible_neighbors - number_of_current_neighbors;
        // For each remaining possible neighbor, add it to the current node if it is not already a neighbor
        for _j in 0..number_of_current_remaining_possible_neighbors {
            // Get the list of nodes that are not already neighbors of the current node and that are not the current node itself and that do not have the maximum number of neighbors
            let possible_neighbors: Vec<usize> = (0..nodes.len())
                .filter(|&k| k != i)
                .filter(|&k| !nodes[i].neighbors.contains(&k))
                .filter(|&k| nodes[k].neighbors.len() < nodes[k].max_possible_neighbors)
                .collect();

            // If there are no possible neighbors, break the loop
            if possible_neighbors.len() == 0 {
                break;
            }

            // Pick a random node from the list of possible neighbors
            let neighbor_index: usize = rng.gen_range(0..possible_neighbors.len());
            let neighbor: usize = possible_neighbors[neighbor_index];
            let neighbor_id: usize = nodes[neighbor].id;
            // Add the neighbor id to the current node's list of neighbors
            nodes[i].neighbors.push(neighbor_id);
            // Add the current node id to the neighbor's list of neighbors
            nodes[neighbor].neighbors.push(current_node_id);
        }
    }



    nodes
}