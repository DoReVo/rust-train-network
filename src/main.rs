use std::collections::HashMap;

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

impl Move {
    fn new() -> Self {
        Move {
            time: 0,
            train: "".to_string(),
            start_node: "".to_string(),
            pickup: vec![],
            end_node: "".to_string(),
            dropoff: vec![],
        }
    }
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    edge_map: &HashMap<String, HashMap<String, i32>>,
    start: &str,
    end: &str,
) -> Option<Vec<String>> {
    let mut dist: HashMap<String, i32> = HashMap::new();
    let mut prev: HashMap<String, String> = HashMap::new();
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start.to_string(), 0);
    heap.push(State {
        cost: 0,
        position: start.to_string(),
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            let mut path = Vec::new();
            let mut u = end;
            while let Some(p) = prev.get(u) {
                path.push(u.to_string());
                u = p;
            }
            path.push(start.to_string());
            path.reverse();
            return Some(path);
        }

        visited.insert(position.clone());

        if let Some(neighbors) = edge_map.get(&position) {
            for (neighbor, &value) in neighbors {
                let next = State {
                    cost: cost + value,
                    position: neighbor.to_string(),
                };

                if !visited.contains(neighbor)
                    && (dist.get(neighbor).is_none() || next.cost < dist[neighbor])
                {
                    heap.push(next.clone());
                    dist.insert(neighbor.to_string(), next.cost);
                    prev.insert(neighbor.to_string(), position.clone());
                }
            }
        }
    }
    None
}

fn main() {
    // List of train stations (nodes)
    let _nodes = vec![
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
            journey_time: 10,
        },
        Edge {
            name: "E2".to_string(),
            from_node: "B".to_string(),
            to_node: "C".to_string(),
            journey_time: 20,
        },
        Edge {
            name: "E3".to_string(),
            from_node: "C".to_string(),
            to_node: "A".to_string(),
            journey_time: 30,
        },
        Edge {
            name: "E4".to_string(),
            from_node: "A".to_string(),
            to_node: "D".to_string(),
            journey_time: 40,
        },
    ];

    // Trains available
    let mut trains = vec![Train {
        name: "Q1".to_string(),
        capacity: 10,
        current_node: "B".to_string(),
    }];

    // List of packages to move
    let mut packages_in_network = vec![
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
            start_node: "A".to_string(),
            destination_node: "D".to_string(),
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

    println!("Successfully created train network map {:?}\n", edge_map);

    let mut packages_in_train: HashMap<String, Vec<Package>> = HashMap::new();

    while !packages_in_network.is_empty() {
        for train in &mut trains {
            println!(
                "Processing train {}, current station {}",
                train.name, train.current_node
            );

            let mut load = 0;
            let mut dropoffs: Vec<Package> = Vec::new();
            let mut move_info = Move::new();

            move_info.train = train.name.clone();

            // Packages in current station
            let package_in_current_station: Vec<&Package> = packages_in_network
                .iter()
                .filter(|p| p.start_node == train.current_node)
                .collect();

            move_info.start_node = train.current_node.clone();

            // Track packages to pickup
            move_info.pickup = package_in_current_station
                .iter()
                .map(|&p| p.clone())
                .collect::<Vec<Package>>();

            if package_in_current_station.is_empty() {
                println!("No packages in station {}", train.current_node);
            } else {
                println!(
                    "Loading packages to pick up at station {}, Packages to pickup are - {:?}",
                    train.current_node,
                    package_in_current_station
                        .iter()
                        .map(|&p| p.name.clone())
                        .collect::<Vec<String>>()
                );

                package_in_current_station.iter().for_each(|&package| {
                    if load + package.weight <= train.capacity {
                        println!(
                            "Train {} is picking up package {}",
                            train.name, package.name
                        );
                        // Load package
                        packages_in_train
                            .entry(train.name.clone())
                            .or_insert(vec![])
                            .push(package.clone());

                        println!(
                            "Package loaded in train, current packages in train {} is {:?}",
                            train.name,
                            packages_in_train
                                .get(&train.name.clone())
                                .unwrap()
                                .iter()
                                .map(|p| p.name.clone())
                                .collect::<Vec<String>>()
                        );
                        // Update train carry limit
                        load += package.weight;
                    } else {
                        println!("Train {} is full", train.name);
                    }
                });

                // Remove the picked up packages from the network
                packages_in_network.retain(|p| p.start_node != train.current_node);

                println!(
                    "Packages in network after operation {:?}",
                    packages_in_network
                        .iter()
                        .map(|p| p.name.clone())
                        .collect::<Vec<String>>()
                );
            }

            let mut path: Vec<String> = if let Some(packages) = packages_in_train.get(&train.name) {
                shortest_path(
                    &edge_map,
                    &train.current_node,
                    &packages[0].destination_node,
                )
                .unwrap_or_else(Vec::new)
            } else {
                shortest_path(
                    &edge_map,
                    &train.current_node,
                    &packages_in_network[0].start_node,
                )
                .unwrap_or_else(Vec::new)
            };

            let next_node = if path.len() > 1 {
                path[1].clone()
            } else {
                train.current_node.clone()
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
                train.name, next_node, move_info.start_node
            );

            move_info.end_node = next_node.clone();
            move_info.time = time;
            move_info.train = train.name.clone();

            let train_content = packages_in_train.get_mut(&train.name);

            match train_content {
                Some(packages) => {
                    // Find package to drop at this station
                    let to_drop: Vec<&Package> = packages
                        .iter()
                        .filter(|p| p.destination_node == train.current_node)
                        .collect();

                    if to_drop.is_empty() {
                        println!("No packages to drop at station {}", train.current_node);
                    } else {
                        println!(
                            "Packages to drop at station {} is {:?}",
                            train.current_node,
                            to_drop
                                .iter()
                                .map(|p| p.name.clone())
                                .collect::<Vec<String>>()
                        );
                        // Track the dropped packages
                        dropoffs
                            .extend(to_drop.iter().map(|&p| p.clone()).collect::<Vec<Package>>());
                        // Actually drop the package
                        packages.retain(|p| p.destination_node != train.current_node);
                    }
                }
                None => {
                    println!(
                        "No packages to drop at station {}, train has not picked up anything.",
                        train.current_node
                    );
                }
            };

            move_info.dropoff = dropoffs;

            match packages_in_train.get(&train.name) {
                Some(packages) => {
                    println!(
                        "Remaining packages in train {} is {:?}",
                        train.name,
                        packages
                            .iter()
                            .map(|p| p.name.clone())
                            .collect::<Vec<String>>()
                    );
                }
                None => {}
            };

            println!(
                "Remaining packages in the global network {:?}",
                packages_in_network
                    .iter()
                    .map(|p| p.name.clone())
                    .collect::<Vec<String>>()
            );

            moves.push(move_info);

            print!("\n");
        }
    }

    println!("\n\nList of moves:");
    for m in &moves {
        println!(
            "W={}, T={}, N1={}, P1={:?}, N2={}, P2={:?}",
            m.time,
            m.train,
            m.start_node,
            m.pickup
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>(),
            m.end_node,
            m.dropoff
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>()
        );
    }
}
