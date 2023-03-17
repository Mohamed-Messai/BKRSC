use std::env;

use dotenv::dotenv;
// use NodesVec from lib.rs
use iot_metrics_simulation::{MetricsFor, NodesVec, NodeStatus, initialize_network, TotalEnergyConsumption, TotalCommunicationOverhead, MetricsType, EnergyType, CommunicationType, StateCostType, EnergyConsumptionType, ExchangeType, ExchangeCostType, CommunicationOverheadType, methods::bkrsc::get_metrics};

fn main() {
    dotenv().ok();
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

    let metrics = get_metrics(100, 10, 10);

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

