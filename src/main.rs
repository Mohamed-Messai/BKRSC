use std::env;

use dotenv::dotenv;
// use NodesVec from lib.rs
use iot_metrics_simulation::{
    initialize_network,
    methods::{bkrsc::get_metrics as bkrsc_get_metrics, others::get_metrics as others_get_metrics},
    CommunicationOverheadType, CommunicationType, EnergyConsumptionType, EnergyType,
    ExchangeCostType, ExchangeType, MetricsFor, MetricsType, NodeStatus, NodesVec, StateCostType,
    TotalCommunicationOverhead, TotalEnergyConsumption,
};

fn main() {
    dotenv().ok();
    let number_of_nodes = env::var("NUMBER_OF_NODES")
        .expect("NUMBER_OF_NODES must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_NODES must be a number");
    let number_of_gateways = env::var("NUMBER_OF_GATEWAYS")
        .expect("NUMBER_OF_GATEWAYS must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_GATEWAYS must be a number");
    let number_of_min_possible_neighbors = env::var("NUMBER_OF_MIN_POSSIBLE_NEIGHBORS")
        .expect("NUMBER_OF_MIN_POSSIBLE_NEIGHBORS must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_MIN_POSSIBLE_NEIGHBORS must be a number");
    let number_of_max_possible_neighbors = env::var("NUMBER_OF_MAX_POSSIBLE_NEIGHBORS")
        .expect("NUMBER_OF_MAX_POSSIBLE_NEIGHBORS must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_MAX_POSSIBLE_NEIGHBORS must be a number");
    let number_of_gateway_members = env::var("NUMBER_OF_GATEWAY_MEMBERS")
        .expect("NUMBER_OF_GATEWAY_MEMBERS must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_GATEWAY_MEMBERS must be a number");
    let vec: NodesVec = initialize_network(number_of_nodes, number_of_gateways, number_of_min_possible_neighbors, number_of_max_possible_neighbors);
    simulate(vec, number_of_nodes, number_of_gateway_members, (number_of_min_possible_neighbors + number_of_max_possible_neighbors)/2 as i32);
}

fn simulate(mut vec: NodesVec, number_of_nodes: i32, number_of_gateway_members: i32, number_of_neighbors: i32) {
    for i in 1..=10 {
        vec.compromise_nodes(i);
        let compromised_nodes = vec.compromised_nodes();

        let bkrsc_metrics = bkrsc_get_metrics(number_of_nodes as u32, number_of_gateway_members as u32, number_of_neighbors as u32);

        let others_metrics = others_get_metrics(number_of_nodes as u32, number_of_gateway_members as u32, number_of_neighbors as u32);

        println!(
            "COMPROMISED-BKRSC: Total energy consumption: {}",
            vec.total_energy_consumption(
                NodeStatus::Compromised,
                MetricsFor::Constrained,
                bkrsc_metrics
            )
        );
        println!(
            "COMPROMISED-BKRSC: Total communication overhead: {}",
            vec.total_communication_overhead(
                NodeStatus::Compromised,
                MetricsFor::Constrained,
                bkrsc_metrics
            )
        );

        println!(
            "COMPROMISED-OTHERS: Total energy consumption: {}",
            vec.total_energy_consumption(
                NodeStatus::Compromised,
                MetricsFor::Constrained,
                others_metrics
            )
        );
        println!(
            "COMPROMISED-OTHERS: Total communication overhead: {}",
            vec.total_communication_overhead(
                NodeStatus::Compromised,
                MetricsFor::Constrained,
                others_metrics
            )
        );

        vec.reset();

        vec.leave_nodes(i);

        let leaving_nodes = vec.left_nodes();

        println!(
            "LEAVING-BKRSC: Total energy consumption: {}",
            vec.total_energy_consumption(
                NodeStatus::Leaving,
                MetricsFor::Constrained,
                bkrsc_metrics
            )
        );
        println!(
            "LEAVING-BKRSC: Total communication overhead: {}",
            vec.total_communication_overhead(
                NodeStatus::Leaving,
                MetricsFor::Constrained,
                bkrsc_metrics
            )
        );

        println!(
            "LEAVING-OTHERS: Total energy consumption: {}",
            vec.total_energy_consumption(
                NodeStatus::Leaving,
                MetricsFor::Constrained,
                others_metrics
            )
        );
        println!(
            "LEAVING-OTHERS: Total communication overhead: {}",
            vec.total_communication_overhead(
                NodeStatus::Leaving,
                MetricsFor::Constrained,
                others_metrics
            )
        );

        vec.reset();

        vec.drain_nodes(i);

        let draining_nodes = vec.drained_nodes();

        println!(
            "DRAINED-BKRSC: Total energy consumption: {}",
            vec.total_energy_consumption(
                NodeStatus::Draining,
                MetricsFor::Constrained,
                bkrsc_metrics
            )
        );
        println!(
            "DRAINED-BKRSC: Total communication overhead: {}",
            vec.total_communication_overhead(
                NodeStatus::Draining,
                MetricsFor::Constrained,
                bkrsc_metrics
            )
        );

        println!(
            "DRAINED-OTHERS: Total energy consumption: {}",
            vec.total_energy_consumption(
                NodeStatus::Draining,
                MetricsFor::Constrained,
                others_metrics
            )
        );
        println!(
            "DRAINED-OTHERS: Total communication overhead: {}",
            vec.total_communication_overhead(
                NodeStatus::Draining,
                MetricsFor::Constrained,
                others_metrics
            )
        );

        vec.reset();
    }
}
