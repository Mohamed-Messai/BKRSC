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
    let vec: NodesVec = initialize_network(
        number_of_nodes,
        number_of_gateways,
        number_of_min_possible_neighbors,
        number_of_max_possible_neighbors,
    );
    simulate(
        vec,
        number_of_nodes,
        number_of_gateway_members,
        (number_of_min_possible_neighbors + number_of_max_possible_neighbors) / 2 as i32,
    );
}

fn simulate(
    mut vec: NodesVec,
    number_of_nodes: i32,
    number_of_gateway_members: i32,
    number_of_neighbors: i32,
) {
    let iterations = 1000;
    let min_affected_nodes = 1;
    let max_affected_nodes = 10;

    let mut bkrsc_compromised_results: Vec<Vec<(f32, f32)>> = vec![];
    let mut others_compromised_results: Vec<Vec<(f32, f32)>> = vec![];
    
    let mut bkrsc_leaving_results: Vec<Vec<(f32, f32)>> = vec![];
    let mut others_leaving_results: Vec<Vec<(f32, f32)>> = vec![];

    let mut bkrsc_draining_results: Vec<Vec<(f32, f32)>> = vec![];
    let mut others_draining_results: Vec<Vec<(f32, f32)>> = vec![];

    let mut bkrsc_compromised_average: Vec<(f32, f32)> = vec![];
    let mut others_compromised_average: Vec<(f32, f32)> = vec![];
    let mut bkrsc_leaving_average: Vec<(f32, f32)> = vec![];
    let mut others_leaving_average: Vec<(f32, f32)> = vec![];
    let mut bkrsc_draining_average: Vec<(f32, f32)> = vec![];
    let mut others_draining_average: Vec<(f32, f32)> = vec![];

    let bkrsc_metrics = bkrsc_get_metrics(
        number_of_nodes as u32,
        number_of_gateway_members as u32,
        number_of_neighbors as u32,
    );
    let others_metrics = others_get_metrics(
        number_of_nodes as u32,
        number_of_gateway_members as u32,
        number_of_neighbors as u32,
    );

    for iteration in 0..iterations {

        bkrsc_compromised_results.push(vec![]);
        others_compromised_results.push(vec![]);
        bkrsc_leaving_results.push(vec![]);
        others_leaving_results.push(vec![]);
        bkrsc_draining_results.push(vec![]);
        others_draining_results.push(vec![]);

        for i in min_affected_nodes..=max_affected_nodes {
            vec.compromise_nodes(i);
            
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

            println!(
                "bkrsc compromised: {:?}",
                bkrsc_compromised_average
            );

            bkrsc_compromised_results[iteration as usize].push((
                vec.total_energy_consumption(
                    NodeStatus::Compromised,
                    MetricsFor::Constrained,
                    bkrsc_metrics,
                ),
                vec.total_communication_overhead(
                    NodeStatus::Compromised,
                    MetricsFor::Constrained,
                    bkrsc_metrics,
                ),
            ));

            others_compromised_results[iteration as usize].push((
                vec.total_energy_consumption(
                    NodeStatus::Compromised,
                    MetricsFor::Constrained,
                    others_metrics,
                ),
                vec.total_communication_overhead(
                    NodeStatus::Compromised,
                    MetricsFor::Constrained,
                    others_metrics,
                ),
            ));

            vec.reset();

            vec.leave_nodes(i);

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

            bkrsc_leaving_results[iteration as usize].push((
                vec.total_energy_consumption(
                    NodeStatus::Leaving,
                    MetricsFor::Constrained,
                    bkrsc_metrics,
                ),
                vec.total_communication_overhead(
                    NodeStatus::Leaving,
                    MetricsFor::Constrained,
                    bkrsc_metrics,
                ),
            ));

            others_leaving_results[iteration as usize].push((
                vec.total_energy_consumption(
                    NodeStatus::Leaving,
                    MetricsFor::Constrained,
                    others_metrics,
                ),
                vec.total_communication_overhead(
                    NodeStatus::Leaving,
                    MetricsFor::Constrained,
                    others_metrics,
                ),
            ));

            vec.reset();

            vec.drain_nodes(i);

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

            bkrsc_draining_results[iteration as usize].push((
                vec.total_energy_consumption(
                    NodeStatus::Draining,
                    MetricsFor::Constrained,
                    bkrsc_metrics,
                ),
                vec.total_communication_overhead(
                    NodeStatus::Draining,
                    MetricsFor::Constrained,
                    bkrsc_metrics,
                ),
            ));

            others_draining_results[iteration as usize].push((
                vec.total_energy_consumption(
                    NodeStatus::Draining,
                    MetricsFor::Constrained,
                    others_metrics,
                ),
                vec.total_communication_overhead(
                    NodeStatus::Draining,
                    MetricsFor::Constrained,
                    others_metrics,
                ),
            ));

            vec.reset();
        }
    }

    // Calculate the average of the results of communication overhead and energy consumption
    // Sum and average the same index of each iteration and divide by the number of iterations
    // Push the result to the corresponding vector as tuple (energy consumption, communication overhead)
    // The result variables are bkrsc_compromised_average, bkrsc_leaving_average, bkrsc_draining_average,
    // others_compromised_average, others_leaving_average, others_draining_average
    for i in min_affected_nodes - 1..max_affected_nodes {
        let mut bkrsc_compromised_energy = 0.0;
        let mut bkrsc_compromised_communication = 0.0;
        let mut others_compromised_energy = 0.0;
        let mut others_compromised_communication = 0.0;
        let mut bkrsc_leaving_energy = 0.0;
        let mut bkrsc_leaving_communication = 0.0;
        let mut others_leaving_energy = 0.0;
        let mut others_leaving_communication = 0.0;
        let mut bkrsc_draining_energy = 0.0;
        let mut bkrsc_draining_communication = 0.0;
        let mut others_draining_energy = 0.0;
        let mut others_draining_communication = 0.0;

        for j in 0..iterations {
            bkrsc_compromised_energy += bkrsc_compromised_results[j as usize][i as usize].0;
            bkrsc_compromised_communication +=
                bkrsc_compromised_results[j as usize][i as usize].1;
            others_compromised_energy += others_compromised_results[j as usize][i as usize].0;
            others_compromised_communication +=
                others_compromised_results[j as usize][i as usize].1;
            bkrsc_leaving_energy += bkrsc_leaving_results[j as usize][i as usize].0;
            bkrsc_leaving_communication += bkrsc_leaving_results[j as usize][i as usize].1;
            others_leaving_energy += others_leaving_results[j as usize][i as usize].0;
            others_leaving_communication += others_leaving_results[j as usize][i as usize].1;
            bkrsc_draining_energy += bkrsc_draining_results[j as usize][i as usize].0;
            bkrsc_draining_communication += bkrsc_draining_results[j as usize][i as usize].1;
            others_draining_energy += others_draining_results[j as usize][i as usize].0;
            others_draining_communication += others_draining_results[j as usize][i as usize].1;
        }

        bkrsc_compromised_average.push((
            bkrsc_compromised_energy / iterations as f32,
            bkrsc_compromised_communication / iterations as f32,
        ));
        others_compromised_average.push((
            others_compromised_energy / iterations as f32,
            others_compromised_communication / iterations as f32,
        ));
        bkrsc_leaving_average.push((
            bkrsc_leaving_energy / iterations as f32,
            bkrsc_leaving_communication / iterations as f32,
        ));
        others_leaving_average.push((
            others_leaving_energy / iterations as f32,
            others_leaving_communication / iterations as f32,
        ));
        bkrsc_draining_average.push((
            bkrsc_draining_energy / iterations as f32,
            bkrsc_draining_communication / iterations as f32,
        ));
        others_draining_average.push((
            others_draining_energy / iterations as f32,
            others_draining_communication / iterations as f32,
        ));
    }

    println!("BKRSC-COMPROMISED-ENERGY");
    for i in 0..bkrsc_compromised_average.len() {
        println!("({}, {})", i + 1, bkrsc_compromised_average[i].0);
    }
    println!("OTHERS-COMPROMISED-ENERGY");
    for i in 0..others_compromised_average.len() {
        println!("({}, {})", i + 1, others_compromised_average[i].0);
    }
    println!("BKRSC-COMPROMISED-COMMUNICATION");
    for i in 0..bkrsc_compromised_average.len() {
        println!("({}, {})", i + 1, bkrsc_compromised_average[i].1);
    }
    println!("OTHERS-COMPROMISED-COMMUNICATION");
    for i in 0..others_compromised_average.len() {
        println!("({}, {})", i + 1, others_compromised_average[i].1);
    }

    println!("BKRSC-LEAVING-ENERGY");
    for i in 0..bkrsc_leaving_average.len() {
        println!("({}, {})", i + 1, bkrsc_leaving_average[i].0);
    }
    println!("OTHERS-LEAVING-ENERGY");
    for i in 0..others_leaving_average.len() {
        println!("({}, {})", i + 1, others_leaving_average[i].0);
    }
    println!("BKRSC-LEAVING-COMMUNICATION");
    for i in 0..bkrsc_leaving_average.len() {
        println!("({}, {})", i + 1, bkrsc_leaving_average[i].1);
    }
    println!("OTHERS-LEAVING-COMMUNICATION");
    for i in 0..others_leaving_average.len() {
        println!("({}, {})", i + 1, others_leaving_average[i].1);
    }

    println!("BKRSC-DRAINING-ENERGY");
    for i in 0..bkrsc_draining_average.len() {
        println!("({}, {})", i + 1, bkrsc_draining_average[i].0);
    }
    println!("OTHERS-DRAINING-ENERGY");
    for i in 0..others_draining_average.len() {
        println!("({}, {})", i + 1, others_draining_average[i].0);
    }
    println!("BKRSC-DRAINING-COMMUNICATION");
    for i in 0..bkrsc_draining_average.len() {
        println!("({}, {})", i + 1, bkrsc_draining_average[i].1);
    }
    println!("OTHERS-DRAINING-COMMUNICATION");
    for i in 0..others_draining_average.len() {
        println!("({}, {})", i + 1, others_draining_average[i].1);
    }


    

}
