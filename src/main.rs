// use NodesVec from lib.rs
use iot_metrics_simulation::{MetricsFor, NodesVec, NodeStatus, initialize_network, TotalEnergyConsumption, TotalCommunicationOverhead, MetricsType, EnergyType, CommunicationType};

fn main() {
    let mut vec:NodesVec = initialize_network(100, 10, 10, 15);
    for i in 0..vec.len() {
        println!("{:?}", vec[i]);
    }
    vec.compromise_nodes(10);
    for i in 0..vec.len() {
        println!("{:?}", vec[i]);
    }
    let compromised_nodes = vec.compromised_nodes();
    println!("Compromised nodes: {:?}", compromised_nodes);
    println!("Number of compromised nodes: {}", compromised_nodes.len());
    // Println id of compromised nodes
    for node in compromised_nodes.iter() {
        println!("Compromised node id: {}", node.id);
    }

    let metrics: MetricsType = MetricsType::new(EnergyType::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0), CommunicationType::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));

    println!("Total energy consumption: {}", vec.total_energy_consumption(NodeStatus::Compromised, MetricsFor::Constrained, metrics));
    println!("Total communication overhead: {}", vec.total_communication_overhead(NodeStatus::Compromised, MetricsFor::Constrained, metrics));

    vec.reset();

    vec.leave_nodes(10);
    
    let leaving_nodes = vec.left_nodes();
    for node in leaving_nodes.iter() {
        println!("Leaving node id: {}", node.id);
    }

    println!("Total energy consumption: {}", vec.total_energy_consumption(NodeStatus::Leaving, MetricsFor::Constrained, metrics));
    println!("Total communication overhead: {}", vec.total_communication_overhead(NodeStatus::Leaving, MetricsFor::Constrained, metrics));

    vec.reset();

    vec.drain_nodes(10);

    let draining_nodes = vec.drained_nodes();
    for node in draining_nodes.iter() {
        println!("Draining node id: {}", node.id);
    }

    println!("Total energy consumption: {}", vec.total_energy_consumption(NodeStatus::Draining, MetricsFor::Constrained, metrics));
    println!("Total communication overhead: {}", vec.total_communication_overhead(NodeStatus::Draining, MetricsFor::Constrained, metrics));
}

