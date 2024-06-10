use crate::Error;

use crate::activity::*;

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::path::Path;
use std::time::Duration;

use serde_json::Value;

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::Direction;
use petgraph::algo;

pub struct Dependency {
    pub graph: Graph<Activity, f32>,

    pub activity_count: usize,

    pub raw: Graph<Activity, f32>,

    indices_map: Vec<(usize, NodeIndex)>,

    pub net_parents_map: HashMap<NodeIndex, Vec<usize>>,

    largest: f32,
}

impl Dependency {
    /// Generate the dependency from a wprofx json file
    pub fn new(path: &Path) -> Result<Dependency, Error> {
        let file = File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let wprof: Value = serde_json::from_reader(reader)?;
        let mut largest = 0.0;

        match wprof.as_array() {
            Some(items) => {
                let mut activities = Vec::new();
                let mut parsed_edges = None;
                for item in items {
                    // Check if it is either a web object or dependency object, otherwise it is statistics
                    if let Some(id) = item.get("id") {
                        let id = id.as_str().expect("\"id\" is not a string");
                        if id != "Deps" {
                            // web object case
                            if let Some(partial_activities) = parse_web_object(item) {
                                activities.extend(partial_activities);
                            }
                        } else {
                            // dependency case
                            parsed_edges = parse_dependency(item);
                        }
                    }
                }

                let mut graph = Graph::new();
                let mut raw = Graph::new();
                // Populate the graph nodes
                let mut activity_id = 0;
                for activity in activities {
                    if activity.end_time > largest {
                        largest = activity.end_time;
                    }

                    raw.add_node(activity.clone());
                    graph.add_node(activity);
                    activity_id += 1;
                }

                // Populate the graph edges, check whether a dependency does not exist
                if let Some(v) = parsed_edges {
                    // At first, labels should be replaced with activity_ids
                    let mut edges = VecDeque::new();
                    for edge in v {
                        let (time, tail_label, head_label) = edge;
                        let tail_index = node_index_by_label(&graph, &tail_label).unwrap();
                        let head_index = node_index_by_label(&graph, &head_label).unwrap();

                        raw.add_edge(tail_index, head_index, time);      
                        if !graph.contains_edge(tail_index, head_index) {
                            graph.add_edge(tail_index, head_index, time);
                            edges.push_back((time, tail_index, head_index));
                        }                 
                    }

                    while edges.len() != 0 {
                        let (time, tail_index, head_index) = edges.pop_front().unwrap();
                        let tail = graph.node_weight(tail_index).unwrap();
                        let head = graph.node_weight(head_index).unwrap();

                        if time == 0.0 {
                            continue;
                        } else if time == -1.0 {
                            continue;
                        } else if time == head.start_time {
                            // tail: |--------------|
                            // head:        |---------------|
                            //
                            // or
                            //
                            // tail: |-----------------------|
                            // head:        |---------|

                            let tail = graph.node_weight_mut(tail_index).unwrap();
                            let latter = tail.halve_and_duplicate(time);
                            let latter_index = graph.add_node(latter);
                            activity_id += 1;

                            
                            let mut outgoings = graph
                                .neighbors_directed(tail_index, Direction::Outgoing)
                                .detach();
                            let mut outgoing_list = Vec::new();
                            while let Some(ex) = outgoings.next_edge(&graph) {
                                outgoing_list.push(ex);
                            }
                            outgoing_list.sort_by(|a, b| graph.edge_weight(*b).partial_cmp(&graph.edge_weight(*a)).unwrap());
                            while let Some(ex) = outgoing_list.pop() {
                                let (_, o_head_index) = graph.edge_endpoints(ex).unwrap();
                                let o_time = graph.edge_weight(ex).unwrap().clone();
                                if o_time == -1.0 || o_time == time {
                                    graph.update_edge(tail_index, o_head_index, -1.0);
                                } else {
                                    graph.remove_edge(ex);
                                    graph.update_edge(latter_index, o_head_index, o_time);
                                    edges = edges
                                        .into_iter()
                                        .filter(|v| *v != (o_time, tail_index, o_head_index))
                                        .collect();
                                    edges.push_back((o_time, latter_index, o_head_index));
                                }
                            }

                            let mut incomings = graph
                                .neighbors_directed(tail_index, Direction::Incoming)
                                .detach();
                            let mut incoming_list = Vec::new();
                            while let Some(ex) = incomings.next_edge(&graph) {
                                incoming_list.push(ex);
                            }
                            incoming_list.sort_by(|a, b| graph.edge_weight(*b).partial_cmp(&graph.edge_weight(*a)).unwrap());
                            while let Some(e) = incoming_list.pop() {
                                let (i_tail_index, _) = graph.edge_endpoints(e).unwrap();
                                let i_time = graph.edge_weight(e).unwrap().clone();
                                if i_time > time {
                                    graph.remove_edge(e);
                                    graph.update_edge(i_tail_index, latter_index, i_time);
                                    edges = edges
                                        .into_iter()
                                        .filter(|v| *v != (i_time, i_tail_index, tail_index))
                                        .collect();
                                    edges.push_back((i_time, i_tail_index, latter_index));
                                }
                            }
                            graph.update_edge(tail_index, latter_index, 0.0);
                        } else if time == tail.end_time {
                            // tail: |--------|
                            // head: |----------------|
                            let head = graph.node_weight_mut(head_index).unwrap();
                            let latter = head.halve_and_duplicate(time);
                            let latter_index = graph.add_node(latter);
                            activity_id += 1;

                            graph.remove_edge(graph.find_edge(tail_index, head_index).unwrap());
                            graph.update_edge(tail_index, latter_index, -1.0);

                        } else if time == tail.start_time {
                            // Maybe wrong dependencies
                            graph.remove_edge(graph.find_edge(tail_index, head_index).unwrap());
                        } else {
                            panic!("Case [5]");
                        }
                    }

                    let mut indices_map = Vec::new();
                    let mut i = 0;
                    for node in graph.node_indices() {
                        indices_map.push((i, node));
                        i += 1;
                    }

                    assert!(!petgraph::algo::is_cyclic_directed(&graph));

                    let dependency = Dependency {
                        graph,
                        activity_count: activity_id,
                        raw,
                        indices_map,
                        net_parents_map: HashMap::new(),                        
                        largest,
                    };

                    Ok(dependency)
                } else {
                    Err(Error::InvalidWProfX)
                }
            }
            None => Err(Error::InvalidWProfX),
        }
    }

    pub fn simplified_graph(&mut self) -> (StableGraph<Activity, f32>, HashMap::<NodeIndex, usize>) {
        let mut simple_graph = StableGraph::new();
        let mut ids = HashMap::new();
        let mut trans = HashMap::new();
        let mut adj_map = HashMap::new();

        // print!("  simplified_graph(): ");
        let net_ids = self.net_activities();
    
        for net_id in &net_ids {
            let onx = self.node_index(*net_id).unwrap();
            let net = self.activity(*net_id).unwrap();
            let nx = simple_graph.add_node(net.clone());
            let adj_onxs = Dependency::find_adjacent_networkings(&self.graph, onx);
            for adj in &adj_onxs {                
                if let Some(v) = self.net_parents_map.get_mut(adj) {
                    v.push(*net_id);
                } else {
                    self.net_parents_map.insert(*adj, vec![*net_id]);
                }
            }
            adj_map.insert(nx, adj_onxs);
            trans.insert(onx, nx);
            ids.insert(nx, *net_id);
        }
    
        let nxs: Vec<_> = simple_graph.node_indices().collect();
        for nx in nxs {
            let adj_onxs = adj_map.get(&nx).unwrap();
            for adj_onx in adj_onxs {
                let adj_nx = trans.get(adj_onx).unwrap();
                simple_graph.add_edge(nx, *adj_nx, 0.0 as f32);
            }
        }

        (simple_graph, ids)
    }

    /// Generate a discovery graph from a simple graph without algo::all_simple_paths()
    pub fn deadlines(&self, rtt: usize)
        -> (HashMap<usize, Duration>, Duration) {
        // print!("  deadlines(): ");

        // Deadlines to return
        let mut deadlines = HashMap::new();

        // Mapping for NodeIndex and intermediate calcultions
        let mut im = HashMap::new();

        // Find root nodes
        let root_indices = self.root_indices();

        // println!("root indices: {:?}", root_indices);

        for root_index in root_indices {
            im.insert(root_index, Duration::new(0, 0));
            let mut stack: Vec<NodeIndex> = vec![root_index];

            while stack.len() > 0 {
                let current_index = stack.pop().unwrap();
                let current = im.get(&current_index).unwrap().clone();
                let mut nexts: Vec<NodeIndex> = self.graph
                    .neighbors_directed(current_index, Direction::Outgoing)
                    .filter(|next_index| {
                        let next = self.graph.node_weight(*next_index).unwrap();
                        let new_deadline = if let ActivityType::Networking(_) = next.activity_type {
                            // If all resources are cached, the time to fetch the resource is zero
                            current + Duration::from_millis(rtt as u64)
                        } else {
                            current + next.duration
                        };

                        match im.get(next_index) {
                            Some(old_deadline) => {
                                if new_deadline > *old_deadline {
                                    im.insert(*next_index, new_deadline);
                                    true
                                } else {
                                    // Discard it and prevent further calculation through this path
                                    // The path with the greatest deadline is still in the stack.
                                    false
                                }
                            },
                            None => {
                                im.insert(*next_index, new_deadline);
                                true
                            }
                        }
                        // match next.activity_type {
                        //     ActivityType::Networking(_) => {
                        //         match im.get(next_index) {
                        //             Some(v) => {
                        //                 if current > *v {
                        //                     im.insert(*next_index, current);
                        //                     true
                        //                 } else {
                        //                     false
                        //                 }
                        //             },
                        //             None => {
                        //                 im.insert(*next_index, current);
                        //                 true
                        //             }
                        //         }
                        //     },
                        //     _ => {
                        //         match im.get(next_index) {
                        //             Some(v) => {
                        //                 if current + next.duration > *v {
                        //                     im.insert(*next_index, current + next.duration);
                        //                     true
                        //                 } else {
                        //                     false
                        //                 }
                        //             },
                        //             None => {
                        //                 im.insert(*next_index, current + next.duration);
                        //                 true
                        //             }   
                        //         }
                        //     }
                        // }
                    })
                    .collect();
                stack.append(&mut nexts);
            }
        }

        for object_id in self.net_activities() {
            let fnx = self.node_index(object_id).unwrap();
            let deadline = im.get(&fnx).unwrap();
            deadlines.insert(object_id, *deadline);
        }

        let (last_index, last_deadline) = im.iter()
        .max_by_key(|(_, &value)| value)
        .map(|(&key, &value)| (key, value)).unwrap();

        let activity = self.graph.node_weight(last_index).unwrap();
        let plt = match activity.activity_type {
            ActivityType::Networking(_) => last_deadline + Duration::from_millis(rtt as u64),
            _ => last_deadline + activity.duration,
        };

        // println!("done.");

        // for (k, v) in &deadlines {
        //     let fnx = self.node_index(*k).unwrap();
        //     let n = self.graph.node_weight(fnx).unwrap();
        //     println!("{}: {:?}", n.url, v);
        // }
        (deadlines, plt)
    }

    /// Generate a discovery graph from a simple graph without algo::all_simple_paths()
    pub fn delayed_load(&self, rtt: usize, rrs: &Vec<(usize, Duration)>, level: isize) -> Duration {
        // print!("  deadlines(): ");

        let mut delayed_im = HashMap::new();
        // for (id, deadline) in rrs {
        //     let index = self.node_index(*id).unwrap();
        //     if delay < 0 {
        //         if *deadline <= Duration::from_millis(-delay as u64) {
        //             delayed_im.insert(index, Duration::from_millis(0));
        //         } else {
        //             delayed_im.insert(index, *deadline - Duration::from_millis(-delay as u64));
        //         }
        //     } else {
        //         delayed_im.insert(index, *deadline + Duration::from_millis(delay as u64));
        //     }
        // }
        
        // for (id, _deadline) in rrs {
        //     let index = self.node_index(*id).unwrap();
        //     delayed_im.insert(index, Duration::from_millis(delay as u64));
        // }

        // let fixed_deadline = step_size(&rrs, level);
        let fixed_deadline = step_size2(&rrs, level);
        
        for (id, deadline) in rrs {
            if level == -2 {
                let index = self.node_index(*id).unwrap();
                delayed_im.insert(index, *deadline);
            } else {
                let index = self.node_index(*id).unwrap();
                delayed_im.insert(index, fixed_deadline);
            }
        }
        // println!("deadline: {:?}", fixed_deadline);

        // for (k, v) in &delayed_im {
        //     println!("{:?} {:?}", k, v);
        // }

        // Deadlines to return
        let mut deadlines = HashMap::new();

        // Mapping for NodeIndex and intermediate calcultions
        let mut im = HashMap::new();

        // Find root nodes
        let root_indices = self.root_indices();

        // println!("root indices: {:?}", root_indices);

        for root_index in root_indices {
            im.insert(root_index, Duration::new(0, 0));
            let mut stack: Vec<NodeIndex> = vec![root_index];

            while stack.len() > 0 {
                let current_index = stack.pop().unwrap();
                
                if let Some(v) = delayed_im.get(&current_index) {
                    if v > im.get(&current_index).unwrap() {
                        im.insert(current_index, v.clone());
                    }
                };
                let current = im.get(&current_index).unwrap().clone();

                let mut nexts: Vec<NodeIndex> = self.graph
                    .neighbors_directed(current_index, Direction::Outgoing)
                    .filter(|next_index| {
                        let next = self.graph.node_weight(*next_index).unwrap();
                        let new_deadline = if let ActivityType::Networking(_) = next.activity_type {
                            // If all resources are cached, the time to fetch the resource is zero
                            current + Duration::from_millis(rtt as u64)
                        } else {
                            current + next.duration
                        };

                        match im.get(next_index) {
                            Some(old_deadline) => {
                                if new_deadline > *old_deadline {
                                    im.insert(*next_index, new_deadline);
                                    true
                                } else {
                                    // Discard it and prevent further calculation through this path
                                    // The path with the greatest deadline is still in the stack.
                                    false
                                }
                            },
                            None => {
                                im.insert(*next_index, new_deadline);
                                true
                            }
                        }
                    })
                    .collect();
                stack.append(&mut nexts);
            }
        }

        for object_id in self.net_activities() {
            let fnx = self.node_index(object_id).unwrap();
            let deadline = im.get(&fnx).unwrap();
            deadlines.insert(object_id, *deadline);
        }

        let (last_index, last_deadline) = im.iter()
        .max_by_key(|(_, &value)| value)
        .map(|(&key, &value)| (key, value)).unwrap();

        let activity = self.graph.node_weight(last_index).unwrap();
        let plt = match activity.activity_type {
            ActivityType::Networking(_) => last_deadline + Duration::from_millis(rtt as u64),
            _ => last_deadline + activity.duration,
        };

        plt
    }

    pub fn root_indices(&self) -> Vec<NodeIndex> {
        let mut root_indices = Vec::new();
        for nx in self.graph.node_indices() {
            if self.graph
                .neighbors_directed(nx, Direction::Incoming)
                .peekable()
                .peek()
                .is_none()
            {
                root_indices.push(nx);
            }
        }
        
        // for root_index in &root_indices {
        //     let n = self.graph.node_weight(*root_index).unwrap();
        //     println!("{}: {}", n.label, n.url);
        // }
        root_indices
    }

    pub fn object_graph(&self) -> Result<(StableGraph<Activity, bool>, HashMap<NodeIndex, usize>), Error> {
        let mut reachables = HashMap::new();
        let mut incomings = HashMap::new();
        let mut outgoings = HashMap::new();
        for nx in self.graph.node_indices() {
            reachables.insert(nx, HashSet::<NodeIndex>::new());

            let incoming: HashSet<NodeIndex> = self.graph.neighbors_directed(nx, Direction::Incoming).collect();
            incomings.insert(nx, incoming);

            let outgoing: HashSet<NodeIndex> = self.graph.neighbors_directed(nx, Direction::Outgoing).collect();
            outgoings.insert(nx, outgoing);
        }

        loop {
            let leaves: Vec<_> = outgoings
                .iter()
                .filter(|(_, outgoing)| {
                    outgoing.is_empty()
                })
                .collect();

            print!("\r  object_graph(): {:.2}%", leaves.len() as f64 / incomings.len() as f64 * 100.0);

            if leaves.len() == incomings.len() {
                break;
            }

            let leaves: Vec<_> = leaves.into_iter().map(|(k, _)| *k).collect();
            let dleaves = leaves.clone();

            for leaf in dleaves.iter() {
                //println!("leaf={:?}", leaf);
                if let Some(incoming) = incomings.get(leaf) {
                    //println!("incoming.len()={}", incoming.len());
                    for parent in incoming.iter() {
                        //println!("parent={:?}", parent);
                        if let ActivityType::Networking(_) = self.graph.node_weight(*parent).unwrap().activity_type {                            
                            // Skip
                        } else {
                            let leaf_reachable = reachables.get(leaf).unwrap().clone();
                            let reachable = reachables.get_mut(parent).unwrap();

                            reachable.insert(*leaf);
                            let reachable: HashSet<_> = reachable.union(&leaf_reachable).map(|k| *k).collect();
                            reachables.insert(*parent, reachable.clone());
                        }

                        let parent_outgoing = outgoings.get_mut(parent).unwrap();
                        parent_outgoing.remove(leaf);
                    }
                }
            }
        }

        let mut sg = StableGraph::new();
        let mut ids = HashMap::new();
        let mut reverse_ids = HashMap::new();

        for object_id in self.net_activities() {
            let object = self.activity(object_id).unwrap().clone();
            let snx = sg.add_node(object);
            ids.insert(snx, object_id);
            reverse_ids.insert(object_id, snx);
        }

        for nx in self.graph.node_indices() {
            if let ActivityType::Networking(_) = self.graph.node_weight(nx).unwrap().activity_type {
                let mut reachable = HashSet::new();
                for dst_nx in self.graph.neighbors_directed(nx, Direction::Outgoing) {
                    let partial_reachable = reachables.get(&dst_nx).unwrap();
                    reachable = reachable.union(partial_reachable).map(|k| *k).collect();
                }

                for reachable_nx in reachable.iter() {
                    if let ActivityType::Networking(_) = self.graph.node_weight(*reachable_nx).unwrap().activity_type {
                        let dst_id = self.activity_index(*reachable_nx).unwrap();
                        let dst_nx = reverse_ids.get(&dst_id).unwrap();

                        let src_id = self.activity_index(nx).unwrap();
                        let src_nx = reverse_ids.get(&src_id).unwrap();

                        sg.add_edge(*src_nx, *dst_nx, true);
                    }
                }
            }
        }

        // Find the true root
        let true_root_id = self.activity_index(NodeIndex::new(0)).unwrap();
        let true_root_nx = reverse_ids.get(&true_root_id).unwrap();

        // Connect the root of islands with the true root
        let root_indices = self.root_indices();
        for root_index in &root_indices {
            // Skip the true root
            if *root_index == NodeIndex::new(0) {
                continue;
            }

            let island_root_id = self.activity_index(*root_index).unwrap();
            if let Some(island_root_nx) = reverse_ids.get(&island_root_id) {
                sg.add_edge(*true_root_nx, *island_root_nx, true);
            } else {
                return Err(Error::InvalidWProfX);
            }

        }

        print!("\r                         ");
        println!("\r  object_graph(): done.");
        Ok((sg, ids))
    }

    pub fn object_graph2(&self) -> (StableGraph<Activity, bool>, HashMap<NodeIndex, usize>) {
        let mut reachables = HashMap::new();
        let mut incomings = HashMap::new();
        let mut outgoings = HashMap::new();
        for nx in self.graph.node_indices() {
            reachables.insert(nx, HashSet::<NodeIndex>::new());

            let incoming: HashSet<NodeIndex> = self.graph.neighbors_directed(nx, Direction::Incoming).collect();
            incomings.insert(nx, incoming);

            let outgoing: HashSet<NodeIndex> = self.graph.neighbors_directed(nx, Direction::Outgoing).collect();
            outgoings.insert(nx, outgoing);
        }

        loop {
            let leaves: Vec<_> = outgoings
                .iter()
                .filter(|(_, outgoing)| {
                    outgoing.is_empty()
                })
                .collect();

            print!("\r  object_graph(): {:.2}%", leaves.len() as f64 / incomings.len() as f64 * 100.0);

            if leaves.len() == incomings.len() {
                break;
            }

            let leaves: Vec<_> = leaves.into_iter().map(|(k, _)| *k).collect();

            for leaf in leaves.iter() {
                //println!("leaf={:?}", leaf);
                if let Some(incoming) = incomings.get(leaf) {
                    //println!("incoming.len()={}", incoming.len());
                    for parent in incoming.iter() {
                        //println!("parent={:?}", parent);
                        if let ActivityType::Networking(_) = self.graph.node_weight(*parent).unwrap().activity_type {                            
                            // Skip
                        } else {
                            // let leaf_reachable = reachables.get(leaf).unwrap().clone();
                            // let reachable = reachables.get_mut(parent).unwrap();

                            // reachable.insert(*leaf);
                            // let reachable: HashSet<_> = reachable.union(&leaf_reachable).map(|k| *k).collect();
                            // reachables.insert(*parent, reachable.clone());

                            let mut updated_reachable = reachables.get(leaf).unwrap().clone();
                            updated_reachable.insert(*leaf);

                            let parent_reachable = reachables.get(parent).unwrap();
                            let updated_reachable: HashSet<_> = updated_reachable.union(parent_reachable).map(|k| *k).collect();
                            
                            reachables.insert(*parent, updated_reachable);
                        }

                        let parent_outgoing = outgoings.get_mut(parent).unwrap();
                        parent_outgoing.remove(leaf);
                    }
                }
            }
        }

        let mut sg = StableGraph::new();
        let mut ids = HashMap::new();
        let mut reverse_ids = HashMap::new();

        for object_id in self.net_activities() {
            let object = self.activity(object_id).unwrap().clone();
            let snx = sg.add_node(object);
            ids.insert(snx, object_id);
            reverse_ids.insert(object_id, snx);
        }

        for nx in self.graph.node_indices() {
            if let ActivityType::Networking(_) = self.graph.node_weight(nx).unwrap().activity_type {
                let mut reachable = HashSet::new();
                for dst_nx in self.graph.neighbors_directed(nx, Direction::Outgoing) {
                    let partial_reachable = reachables.get(&dst_nx).unwrap();
                    reachable = reachable.union(partial_reachable).map(|k| *k).collect();
                }

                for reachable_nx in reachable.iter() {
                    if let ActivityType::Networking(_) = self.graph.node_weight(*reachable_nx).unwrap().activity_type {
                        let dst_id = self.activity_index(*reachable_nx).unwrap();
                        let dst_nx = reverse_ids.get(&dst_id).unwrap();

                        let src_id = self.activity_index(nx).unwrap();
                        let src_nx = reverse_ids.get(&src_id).unwrap();

                        sg.add_edge(*src_nx, *dst_nx, true);
                    }
                }
            }
        }

        print!("\r                         ");
        println!("\r  object_graph(): done.");
        (sg, ids)
    }

    pub fn object_graph3(&self) -> (StableGraph<Activity, bool>, HashMap<NodeIndex, usize>) {
        let mut ancestors: HashMap<NodeIndex, HashSet<NodeIndex>> = HashMap::new();

        let mut root_indices = Vec::new();
        for nx in self.graph.node_indices() {
            if self.graph
                .neighbors_directed(nx, Direction::Incoming)
                .peekable()
                .peek()
                .is_none()
            {
                let node = self.graph.node_weight(nx).unwrap();
                if let ActivityType::Networking(_) = node.activity_type {
                    root_indices.push(nx);
                }
            }
        }

        for root_index in root_indices {
            let mut root_ancestor = HashSet::new();
            root_ancestor.insert(root_index);
            ancestors.insert(root_index, root_ancestor);
            let mut stack: Vec<NodeIndex> = vec![root_index];

            while stack.len() > 0 {
                let current_index = stack.pop().unwrap();
                let current_ancestor = ancestors.get(&current_index).unwrap().clone();
                let mut nexts: Vec<NodeIndex> = self.graph
                    .neighbors_directed(current_index, Direction::Outgoing)
                    .filter(|next_index| {
                        let next = self.graph.node_weight(*next_index).unwrap();
                        match next.activity_type {
                            ActivityType::Networking(_) => {
                                match ancestors.get_mut(next_index) {
                                    Some(v) => {
                                        if v.is_superset(&current_ancestor) && !current_ancestor.is_superset(v) {
                                            false
                                        } else {
                                            let updated: HashSet<_> = v.union(&current_ancestor).map(|k| *k).collect();
                                            ancestors.insert(*next_index, updated);
                                            true
                                        }
                                    },
                                    None => {
                                        let mut next_ancestor = current_ancestor.clone();
                                        next_ancestor.insert(*next_index);
                                        ancestors.insert(*next_index, next_ancestor);
                                        true
                                    }
                                }
                            },
                            _ => {
                                match ancestors.get_mut(next_index) {
                                    Some(v) => {
                                        if v.is_superset(&current_ancestor) && !current_ancestor.is_superset(v) {
                                            false
                                        } else {
                                            let updated: HashSet<_> = v.union(&current_ancestor).map(|k| *k).collect();
                                            ancestors.insert(*next_index, updated);
                                            true
                                        }
                                    },
                                    None => {
                                        ancestors.insert(*next_index, current_ancestor.clone());
                                        true
                                    }
                                }
                            }
                        }
                    })
                    .collect();
                stack.append(&mut nexts);
            }
        }

        let mut sg = StableGraph::new();
        let mut ids = HashMap::new();
        let mut reverse_ids = HashMap::new();

        for object_id in self.net_activities() {
            let object = self.activity(object_id).unwrap().clone();
            println!("net: {} {}", object_id, object);
            let snx = sg.add_node(object);
            ids.insert(snx, object_id);
            reverse_ids.insert(object_id, snx);
        }

        for nx in self.graph.node_indices() {
            if let ActivityType::Networking(_) = self.graph.node_weight(nx).unwrap().activity_type {
                let ancestor = ancestors.get(&nx).unwrap();
                let did = self.activity_index(nx).unwrap();
                let dnx = reverse_ids.get(&did).unwrap();

                println!("{} len={}", self.graph.node_weight(nx).unwrap(), ancestor.len());

                for ax in ancestor {
                    if *ax == nx {
                        continue;
                    }

                    let sid = self.activity_index(*ax).unwrap();
                    let snx = reverse_ids.get(&sid).unwrap();
                    sg.add_edge(*snx, *dnx, true);

                    let s = self.node_index(sid).unwrap();
                    let sn = self.graph.node_weight(s).unwrap();

                    let d = self.node_index(did).unwrap();
                    let dn = self.graph.node_weight(d).unwrap();

                    println!("{} -> {}", sn, dn);
                    
                }
            }
        }

        print!("\r                         ");
        println!("\r  object_graph(): done.");
        (sg, ids)
    }

    /// Generate a discovery graph from a simple graph
    pub fn discovery_graph(&self, sg: StableGraph<Activity, f32>, ids: HashMap<NodeIndex, usize>)
        -> (StableGraph<Activity, Duration>, HashMap<NodeIndex, usize>) {
        // Declare sg and ids to return
        let mut rsg = StableGraph::new();
        let mut rids = HashMap::new();

        print!("discovery_graph(): ");
        // Declare a temporary mapping for nodes from old sg to new rsg
        let mut temp_map = HashMap::new();

        // Populate the nodes in rsg
        for nx in sg.node_indices() {
            let node = sg.node_weight(nx).unwrap();
            let object_id = ids.get(&nx).unwrap();
            
            let rnx = rsg.add_node(node.clone());
            rids.insert(rnx, *object_id);

            temp_map.insert(nx, rnx);
            //println!("nx={:?}, rnx={:?}", nx, rnx);
        }

        let exs: Vec<_> = sg.edge_indices().collect();
        println!("total edges={}", exs.len());
        for ex in exs {
            println!("edge {:?}", ex);
            let (sg_src, sg_dst) = sg.edge_endpoints(ex).unwrap();
            let src = self.node_index(*ids.get(&sg_src).unwrap()).unwrap();
            let dst = self.node_index(*ids.get(&sg_dst).unwrap()).unwrap();
            let paths = algo::all_simple_paths::<Vec<_>, _>(&self.graph, src, dst, 0, None)
                .collect::<Vec<_>>();
            println!("paths.len()={}", paths.len());
            //println!("{}->{}:", self.activity(*ids.get(&sg_src).unwrap()).unwrap(), self.activity(*ids.get(&sg_dst).unwrap()).unwrap());
            let direct_paths = paths
                .iter()
                .filter_map(|p| {
                    //print!("  ");
                    // for pn in p {
                    //     print!("{} ", self.graph.node_weight(*pn).unwrap());
                    // }
                    //println!("");
                    
                    let mut discovery = Duration::new(0 ,0);
                    let candidate_path = p
                        .iter()
                        .filter(|nx| {
                            if **nx == src || **nx == dst {
                                return false
                            }
                            //print!("{}({:?}) ", self.graph.node_weight(**nx).unwrap(), self.graph.node_weight(**nx).unwrap().duration);
                            let activity = self.graph.node_weight(**nx).unwrap();
                            if let ActivityType::Networking(_) = activity.activity_type {
                                true
                            } else {
                                discovery += activity.duration;
                                false
                            }
                        })
                        .collect::<Vec<_>>();
                    //println!(": {:?}", discovery);
                    if candidate_path.is_empty() {
                        Some(discovery)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
                //println!("");
            let max;
            if let Some(v) = direct_paths.iter().max() {
                max = *v;
            } else {
                max = Duration::new(0, 0);
            }

            println!("{}->{}: {:?}\n", self.activity(*ids.get(&sg_src).unwrap()).unwrap(), self.activity(*ids.get(&sg_dst).unwrap()).unwrap(), max);
            let rsrc = temp_map.get(&sg_src).unwrap();
            let rdst = temp_map.get(&sg_dst).unwrap();
            rsg.add_edge(*rsrc, *rdst, max);
        }

        println!("done");
        (rsg, rids)
    }

    pub fn find_adjacent_networkings(g: &Graph<Activity, f32>, nx: NodeIndex) -> Vec<NodeIndex> {
        let mut adjacents = Vec::new();
    
        let mut nexts = vec![nx];
        while nexts.len() > 0 {
            let mut next: Vec<NodeIndex> = g
                .neighbors_directed(nexts.pop().unwrap(), Direction::Outgoing)
                .filter(|c| {
                    if let ActivityType::Networking(_) = g.node_weight(*c).unwrap().activity_type
                    {
                        if !adjacents.contains(c) {
                            adjacents.push(*c);
                        }
                        false
                    } else if nexts.contains(c) {
                        false
                    } else {
                        true
                    }
                })
                .collect();
            nexts.append(&mut next);
        }
    
        adjacents
    }

    pub fn net_activities(&self) -> Vec<usize> {
        let mut net_activities = Vec::new();
        // in order of activity_ids
        for i in 0..self.activity_count {
            if let ActivityType::Networking(_) = self
                .graph
                .node_weight(self.node_index(i).unwrap())
                .unwrap()
                .activity_type
            {
                net_activities.push(i);
            }
        }

        net_activities
    }

    pub fn com_activities(&self) -> Vec<usize> {
        let mut com_activities = Vec::new();
        // in order of activity_ids
        for i in 0..self.activity_count {
            if let ActivityType::Networking(_) = self
                .graph
                .node_weight(self.node_index(i).unwrap())
                .unwrap()
                .activity_type
            {
                continue;
            } else {
                com_activities.push(i);
            }
        }

        com_activities
    }

    pub fn activity(&self, activity_id: usize) -> Option<&Activity> {
        let node = self.node_index(activity_id)?;
        Some(self.graph.node_weight(node).unwrap())
    }

    pub fn parents(&self, activity_id: usize) -> Vec<usize> {
        let mut parents = Vec::new();
        let incomings = self.graph.neighbors_directed(
            self.node_index(activity_id).unwrap(),
            petgraph::Direction::Incoming,
        );
        for incoming in incomings {
            parents.push(self.activity_index(incoming).unwrap());
        }
        parents
    }

    pub fn is_net(&self, activity_id: usize) -> Option<bool> {
        let node = self.node_index(activity_id)?;
        if let ActivityType::Networking(_) = self.graph.node_weight(node).unwrap().activity_type {
            Some(true)
        } else {
            Some(false)
        }
    }

    pub fn largest_end_time(&self) -> Duration {
        Duration::from_micros((self.largest * 1000.0) as u64)
    }

    pub fn find_ancestor(&self, activity_id: usize) -> Option<usize> {
        let mut parent_stack = self.parents(activity_id);
        while let Some(parent_id) = parent_stack.pop() {
            if self.is_net(parent_id)? == true {
                return Some(parent_id);
            }
            parent_stack.extend_from_slice(&self.parents(parent_id));
        }
        
        None
    }

    pub fn node_index(&self, target_activity_id: usize) -> Option<NodeIndex> {
        for (activity_id, node_index) in &self.indices_map {
            if *activity_id == target_activity_id {
                return Some(*node_index);
            }
        }
        return None;
    }

    pub fn activity_index(&self, target_node_index: NodeIndex) -> Option<usize> {
        for (activity_id, node_index) in &self.indices_map {
            if *node_index == target_node_index {
                return Some(*activity_id);
            }
        }
        return None;
    }

    pub fn aggregate_computations(&self) -> HashMap<usize, Duration> {
        let mut aggregated = HashMap::new();

        let net_ids = self.net_activities();

        for net_id in net_ids {
            let mut nexts = vec![self.node_index(net_id).unwrap()];
            let mut done = HashSet::new();
            let mut sum = self.activity(net_id).unwrap().duration;
            while nexts.len() != 0 {
                let mut next: Vec<NodeIndex> = self.graph
                    .neighbors_directed(nexts.pop().unwrap(), Direction::Outgoing)
                    .filter(|c| {
                        if let ActivityType::Networking(_) = self.graph.node_weight(*c).unwrap().activity_type {
                            false
                        } else if nexts.contains(c) {
                            false
                        } else {
                            if !done.contains(c) {
                                sum = sum + self.graph.node_weight(*c).unwrap().duration;
                                done.insert(*c);
                            }
                            true
                        }
                    })
                    .collect();
                nexts.append(&mut next);
            }
            aggregated.insert(net_id, sum);
        }
        aggregated
    }

    pub fn estimate_transfer(&self, rtt: Duration) -> HashMap<usize, Duration> {
        let mut estimated = HashMap::new();

        let net_ids = self.net_activities();

        for net_id in net_ids {
            let net = self.activity(net_id).unwrap();
            if let ActivityType::Networking(net_detail) = net.activity_type {
                // Default max_send_udp_payload_size is 1200 and initial window size is 10
                // If the number of bytes is not specified, set 1 RTT.
                if let Some(bytes) = net_detail.status.size {
                    estimated.insert(net_id, rtt.mul_f32((bytes / 12000 + 1) as f32));
                } else {
                    estimated.insert(net_id, rtt);
                }
            }
        }

        estimated
    }
}

pub fn compare_parents(d1: &Dependency, d2: &Dependency, id1: usize, id2: usize) -> bool {
    let mut id1_parents: Vec<_> = d1.parents(id1).into_iter().map(|x| Some(x)).collect();
    let mut id2_parents: Vec<_> = d2.parents(id2).into_iter().map(|x| Some(x)).collect();

    if id1_parents.len() != id2_parents.len() {
        return false;
    }

    for i in 0..id1_parents.len() {
        for j in 0..id2_parents.len() {
            if let Some(v1) = id1_parents[i] {
                if let Some(v2) = id2_parents[j] {
                    if d1.activity(v1).unwrap().url == d2.activity(v2).unwrap().url {
                        id1_parents[i] = None;
                        id2_parents[j] = None;
                        // println!("matched: {}", d1.activity(v1).unwrap().url);
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }

    for i in 0..id1_parents.len() {
        for j in 0..id2_parents.len() {
            if let Some(_) = id1_parents[i] {
                if let Some(_) = id2_parents[j] {
                    return false;
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }

    true
}

/// Parse a single web object. For example,
///
/// {
///   "id": "https://www.wikipedia.org/",
///   "objs": [
///     {
///       "activityId": "Scripting_0",
///       "url": "https://www.wikipedia.org/",
///       "startTime": 65.67,
///       "endTime": 66.511
///     },
///     {
///       "activityId": "Scripting_1",
///       "url": "https://www.wikipedia.org/",
///       "startTime": 75.986,
///       "endTime": 76.27300000000001
///     },
///     ...
///   ]
/// }
fn parse_web_object(object: &Value) -> Option<Vec<Activity>> {
    let id = object.get("id").unwrap();
    if id == "Rendering" || id == "Painting" {
        None
    } else {
        let mut associated_activities = Vec::new();
        let objs = object.get("objs").unwrap().as_array().unwrap();
        for obj in objs {
            if let Some(activity) = Activity::new(obj) {
                associated_activities.push(activity);
            }
        }
        Some(associated_activities)
    }
}

/// Parse the dependency. For example,
///
/// {
///     "id": "Deps",
///     "objs": [
///       { "time": -1, "a1": "Networking_0", "a2": "Loading_0" },
///       { "time": 64.942, "a1": "Loading_0", "a2": "Networking_1" },
///       ...
///     ]
/// }
fn parse_dependency(dependency: &Value) -> Option<Vec<(f32, String, String)>> {
    let mut edges = Vec::new();
    let objs = dependency.get("objs").unwrap().as_array().unwrap();
    for obj in objs {
        let time = obj.get("time").unwrap().as_f64().unwrap() as f32;
        let tail = obj.get("a1").unwrap().as_str().unwrap().to_string();
        let head = obj.get("a2").unwrap().as_str().unwrap().to_string();
        edges.push((time, tail, head));
    }
    Some(edges)
}

fn node_index_by_label(g: &Graph<Activity, f32>, label: &str) -> Option<NodeIndex> {
    g.node_indices()
        .find(|i| format!("{}", g[*i].label) == label)
}

fn step_size(rrs: &Vec<(usize, Duration)>, level: isize) -> Duration {
    let mut sum = Duration::from_millis(0);
    for (_id, deadline) in rrs {
        sum += *deadline;
    }
    let mean: Duration = sum / rrs.len() as u32;

    let (_, max_deadline) = rrs.iter()
    .max_by_key(|(_, value)| value).unwrap();

    let (_, min_deadline) = rrs.iter()
    .min_by_key(|(_, value)| value).unwrap();

    let step_max = (*max_deadline - mean) / 2;
    let step_mean = (mean - *min_deadline) / 2;

    if level < 0 {
        let abs_level = -level as u32;
        if step_mean * abs_level > mean {
            return Duration::from_millis(0);
        } else {
            return mean - abs_level * step_mean;
        }
    } else {
        return mean + step_max * level as u32;
    }
}

fn step_size2(rrs: &Vec<(usize, Duration)>, level: isize) -> Duration {
    if level == -1 {
        return rrs[rrs.len() / 2 as usize].1;
    }

    let (_, max_deadline) = rrs.iter()
    .max_by_key(|(_, value)| value).unwrap();

    let (_, min_deadline) = rrs.iter()
    .min_by_key(|(_, value)| value).unwrap();

    let step_size = (*max_deadline - *min_deadline) / 5;

    return *min_deadline + step_size * level as u32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn object_graph_test() {
        let d = gen_dependency();
        let (g, m) = d.object_graph().unwrap();
        for n in g.node_indices() {
            let outgoings: Vec<_> = g.neighbors_directed(n, Direction::Outgoing).collect();
            println!("{:?}: {:?}", n, outgoings);
        }
    }

    fn gen_dependency() -> Dependency {
        let mut graph = Graph::new();
        let n1 = graph.add_node(gen_network_activity("n1"));
        let n2 = graph.add_node(gen_network_activity("n2"));
        let n3 = graph.add_node(gen_network_activity("n3"));
        let n4 = graph.add_node(gen_network_activity("n4"));
        let n5 = graph.add_node(gen_network_activity("n5"));

        let c1 = graph.add_node(gen_computation_activity("c1"));
        let c2 = graph.add_node(gen_computation_activity("c2"));
        let c3 = graph.add_node(gen_computation_activity("c3"));
        let c4 = graph.add_node(gen_computation_activity("c4"));
        let c5 = graph.add_node(gen_computation_activity("c5"));

        graph.add_edge(n1, c1, 0.0);
        graph.add_edge(n1, c2, 0.0);
        graph.add_edge(c1, n2, 0.0);
        graph.add_edge(c2, n3, 0.0);
        graph.add_edge(n2, c3, 0.0);
        graph.add_edge(c3, n3, 0.0);
        graph.add_edge(c3, n4, 0.0);
        graph.add_edge(n3, c4, 0.0);
        graph.add_edge(n4, c5, 0.0);
        graph.add_edge(c4, n5, 0.0);
        graph.add_edge(c5, n5, 0.0);
        
        let raw = Graph::new();
        let mut indices_map = Vec::new();
        let mut i = 0;
        for node in graph.node_indices() {
            indices_map.push((i, node));
            i += 1;
        }

        Dependency {
            graph,
            activity_count: 10,
            raw,
            indices_map,
            net_parents_map: HashMap::new(),
            largest: 0.0,
        }
    }

    fn gen_network_activity(label: &str) -> Activity {
        Activity {
            url: "test".to_string(),
            start_time: 0.0,
            end_time: 0.0,
            duration: Duration::new(0, 0),
            label: label.to_string(),
            activity_type: ActivityType::Networking(NetDetail {
                status: Status {
                    size: None,
                    code: None,
                },
                mime_type: MimeType::Html,
            })
        }
    }

    fn gen_computation_activity(label: &str) -> Activity {
        Activity {
            url: "test".to_string(),
            start_time: 0.0,
            end_time: 0.0,
            duration: Duration::new(0, 0),
            label: label.to_string(),
            activity_type: ActivityType::Scripting
        }
    }
}