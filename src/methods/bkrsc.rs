use std::env;

use lazy_static::lazy_static;

use crate::{
    CommunicationOverheadType, CommunicationType, EnergyConsumptionType, EnergyType,
    ExchangeCostType, ExchangeType, MetricsType, StateCostType,
};

lazy_static! {
    static ref EPSB: f32 = env::var("EPSB")
        .unwrap_or(0.0001.to_string())
        .parse::<f32>()
        .unwrap();
    static ref EPRB: f32 = env::var("EPRB")
        .unwrap_or(0.0001.to_string())
        .parse::<f32>()
        .unwrap();
    static ref SENT_MESSAGE_SIZE: u32 = env::var("SENT_MESSAGE_SIZE")
        .unwrap_or(16.to_string())
        .parse::<u32>()
        .unwrap();
    static ref RECEIVED_MESSAGE_SIZE: u32 = env::var("RECEIVED_MESSAGE_SIZE")
        .unwrap_or(16.to_string())
        .parse::<u32>()
        .unwrap();
}

pub fn get_metrics(
    number_of_nodes: u32,
    number_of_gateway_members: u32,
    number_of_neighbors: u32,
) -> MetricsType {
    let metrics = MetricsType {
        energy: EnergyType {
            compromised: EnergyConsumptionType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: number_of_nodes,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: number_of_neighbors + 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
            },
            draining: EnergyConsumptionType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: number_of_gateway_members,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
            },
            leaving: EnergyConsumptionType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
            },
        },
        communication: CommunicationType {
            compromised: CommunicationOverheadType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
            },
            draining: CommunicationOverheadType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
            },
            leaving: CommunicationOverheadType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0.1,
                        received: 0.1,
                    },
                },
            },
        },
    };
    metrics
}
