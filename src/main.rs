use std::{
    collections::{HashMap, VecDeque},
    vec,
};

#[derive(Debug, Clone)]
struct Edge {
    name: String,
    from_node: String,
    to_node: String,
    journey_time: i32,
}

#[derive(Debug, Clone)]
struct Train {
    name: String,
    capacity: i32,
    current_node: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Package {
    name: String,
    weight: i32,
    start_node: String,
    destination_node: String,
}

#[derive(Debug, Clone)]
struct Move {
    time: i32,
    train: String,
    start_node: String,
    pickup: Vec<Package>,
    end_node: String,
    dropoff: Vec<Package>,
}

fn main() {
    // List of train stations (nodes)
    let nodes = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
    ];

    // List of routes between train station (edges)
    let edges = vec![
        Edge {
            name: "E1".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            journey_time: 30,
        },
        Edge {
            name: "E2".to_string(),
            from_node: "B".to_string(),
            to_node: "C".to_string(),
            journey_time: 10,
        },
        Edge {
            name: "E3".to_string(),
            from_node: "C".to_string(),
            to_node: "A".to_string(),
            journey_time: 20,
        },
        Edge {
            name: "E4".to_string(),
            from_node: "D".to_string(),
            to_node: "A".to_string(),
            journey_time: 15,
        },
    ];

    // Trains available
    let mut trains = vec![
        Train {
            name: "Q1".to_string(),
            capacity: 6,
            current_node: "B".to_string(),
        },
        Train {
            name: "Q2".to_string(),
            capacity: 10,
            current_node: "C".to_string(),
        },
    ];

    // List of packages to move
    let mut packages = vec![
        Package {
            name: "K1".to_string(),
            weight: 5,
            start_node: "A".to_string(),
            destination_node: "C".to_string(),
        },
        Package {
            name: "K2".to_string(),
            weight: 5,
            start_node: "C".to_string(),
            destination_node: "A".to_string(),
        },
        Package {
            name: "K3".to_string(),
            weight: 5,
            start_node: "D".to_string(),
            destination_node: "A".to_string(),
        },
        Package {
            name: "K4".to_string(),
            weight: 1,
            start_node: "D".to_string(),
            destination_node: "A".to_string(),
        },
    ];

    let mut moves: Vec<Move> = Vec::new();
    let mut time = 0;

    println!("Creating the train network map");
    let mut edge_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

    // List of nodes, and where you can go from there.
    for edge in &edges {
        edge_map
            .entry(edge.from_node.clone())
            .or_insert(HashMap::new())
            .insert(edge.to_node.clone(), edge.journey_time);

        edge_map
            .entry(edge.to_node.clone())
            .or_insert(HashMap::new())
            .insert(edge.from_node.clone(), edge.journey_time);
    }

    println!("Successfully created train network map {:?}", edge_map);

    while !packages.is_empty() {
        for train in &mut trains {
            println!("Processing train - {:?}", train);

            let mut load = 0;
            // List of items in train
            let mut package_in_train: VecDeque<Package> = VecDeque::new();
            let mut dropoffs: Vec<Package> = Vec::new();

            // Find list of packages in the current station
            let mut to_pickup: VecDeque<Package> = packages
                .iter()
                .cloned()
                .filter(|p| p.start_node == train.current_node)
                .collect();

            println!(
                "Loading packages to pick up at station {}",
                train.current_node
            );

            if to_pickup.is_empty() {
                println!("No package in station {}", train.current_node);
            } else {
                while let Some(package_to_pickup) = to_pickup.pop_front() {
                    if load + package_to_pickup.weight <= train.capacity {
                        println!(
                            "Train - {}, is picking up package - {:?}",
                            train.name, package_to_pickup
                        );
                        // Load package in train
                        package_in_train.push_back(package_to_pickup.clone());
                        // Update train carry limit
                        load += package_to_pickup.weight;
                        // Remove current package from the list of packages
                        // in the network.
                        packages.retain(|p| p.name != package_to_pickup.name);
                    } else {
                        println!("Train {} is full", train.name);
                        break;
                    }
                }
            }

            let start_node = train.current_node.clone();

            // If in train have packages, go to that package destination
            let next_node = if let Some(destination) = package_in_train.front() {
                destination.destination_node.clone()
            } else {
                // if no packages in train, find the next
                // package to pickup from the global package variable
                packages
                    .iter()
                    .map(|p| p.start_node.clone())
                    .next()
                    .unwrap_or_else(|| train.current_node.clone())
            };

            println!("Train {} next node is set to {}", train.name, next_node);

            if edge_map.get(&train.current_node).is_none()
                || edge_map
                    .get(&train.current_node)
                    .unwrap()
                    .get(&next_node)
                    .is_none()
            {
                println!(
                    "No valid path from {} to {}. Skipping train {}",
                    train.current_node, next_node, train.name
                );
                continue;
            }

            let travel_time = edge_map[&train.current_node][&next_node];
            time += travel_time;

            train.current_node = next_node.clone();
            println!(
                "Train {} moved to {} from {}",
                train.name, next_node, start_node
            );

            // Find packages to drop at this destination
            let to_drop: VecDeque<Package> = package_in_train
                .iter()
                .cloned()
                .filter(|p| p.destination_node == train.current_node)
                .collect();

            // Drop packages
            if to_drop.is_empty() {
                println!("No packages to drop at station {}", train.current_node);
            } else {
                let to_dropz: Vec<&Package> = package_in_train
                    .iter()
                    .filter(|p| p.destination_node == train.current_node)
                    .collect();

                println!(
                    "Packages to drop at station {} is {:?}",
                    train.current_node,
                    to_dropz
                        .iter()
                        .map(|p| p.name.clone())
                        .collect::<Vec<String>>()
                );

                // Track the dropped packages
                dropoffs.extend(
                    package_in_train
                        .iter()
                        .filter(|p| p.destination_node == train.current_node)
                        .cloned()
                        .collect::<Vec<Package>>(),
                );

                // Drop packages
                package_in_train.retain(|p| p.destination_node != train.current_node);
            }

            println!("Packages left to be drop {:?}", packages);

            let m = Move {
                time,
                train: train.name.clone(),
                start_node: start_node,
                pickup: to_pickup.into(),
                end_node: next_node,
                dropoff: dropoffs,
            };

            moves.push(m);
        }
    }

    println!("\n\nList of moves:");
    for m in &moves {
        println!(
            "W={}, T={}, N1={}, P1={:?}, N2={}, P2={:?}",
            m.time, m.train, m.start_node, m.pickup, m.end_node, m.dropoff
        );
    }
}
