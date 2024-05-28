use crate::activity::*;
use crate::dependency::*;
use crate::resource::*;

use std::collections::HashMap;

use petgraph::graph::NodeIndex;
use petgraph::Direction;

use url;
use url::Url;

pub fn compare_dependencies(d1: &Dependency, r1: &Resources, d2: &Dependency, r2: &Resources) -> Option<Vec<(usize, usize)>> {
    let mut rrs = Vec::<(usize, usize)>::new();

    // let roots1 = d1.root_indices();
    // let roots2 = d2.root_indices();

    // for root in roots1 {
    //     println!("1 {:?}, {}", root, d1.graph.node_weight(root).unwrap().url);

    //     // depth-first search
    //     let mut reached = Vec::<NodeIndex>::new();
    //     reached.push(root);

    //     let mut stack: Vec<NodeIndex> = vec![root];
    //     while stack.len() > 0 {
    //         let current_index = stack.pop().unwrap();
    //         let mut nexts: Vec<NodeIndex> = d1.graph
    //             .neighbors_directed(current_index, Direction::Outgoing)
    //             .filter(|next_index| !reached.contains(&next_index))
    //             .collect();
    //         // println!("{:?}: {:?}", current_index, nexts);
    //         reached.extend(nexts.iter().cloned());
    //         stack.append(&mut nexts);
    //     }
    //     for node in &reached {
    //         let activity = d1.graph.node_weight(*node).unwrap();
    //         if let ActivityType::Networking(_) = activity.activity_type {
    //             println!("  {:?}, {} {} [{}, {}]", node, activity.label, activity.url, activity.start_time, activity.end_time);
    //         }
    //     }
    // }

    let (sg1, ids1) = d1.simplified_graph();
    let (sg2, ids2) = d2.simplified_graph();

    let mut root_indices1 = Vec::new();
    for nx in sg1.node_indices() {
        if sg1
            .neighbors_directed(nx, Direction::Incoming)
            .peekable()
            .peek()
            .is_none()
        {
            root_indices1.push(nx);
        }
    }
    let mut root_indices2 = Vec::new();
    for nx in sg2.node_indices() {
        if sg2
            .neighbors_directed(nx, Direction::Incoming)
            .peekable()
            .peek()
            .is_none()
        {
            root_indices2.push(nx);
        }
    }

    let mut reachables_for_root1 = HashMap::new();
    for root_index in &root_indices1 {
        // let root = sg1.node_weight(*root_index).unwrap();
        // let nx = d1.activity(*ids1.get(root_index).unwrap()).unwrap();
        // println!("{}", nx.label);

        // depth-first search
        let mut reached = Vec::<NodeIndex>::new();
        reached.push(*root_index);

        let mut stack: Vec<NodeIndex> = vec![*root_index];
        while stack.len() > 0 {
            let current_index = stack.pop().unwrap();
            let mut nexts: Vec<NodeIndex> = sg1
                .neighbors_directed(current_index, Direction::Outgoing)
                .filter(|next_index| !reached.contains(&next_index))
                .collect();
            // println!("{:?}: {:?}", current_index, nexts);
            reached.extend(nexts.iter().cloned());
            stack.append(&mut nexts);
        }
        for node in &reached {
            let activity = sg1.node_weight(*node).unwrap();
            if let ActivityType::Networking(_) = activity.activity_type {
                // println!("  {:?}, {} {} [{}, {}]", node, activity.label, activity.url, activity.start_time, activity.end_time);
            }
        }

        reachables_for_root1.insert(*root_index, reached);
    }

    let mut reachables_for_root2 = HashMap::new();
    for root_index in &root_indices2 {
        // let root = sg1.node_weight(*root_index).unwrap();
        // let nx = d1.activity(*ids1.get(root_index).unwrap()).unwrap();
        // println!("{}", nx.label);

        // depth-first search
        let mut reached = Vec::<NodeIndex>::new();
        reached.push(*root_index);

        let mut stack: Vec<NodeIndex> = vec![*root_index];
        while stack.len() > 0 {
            let current_index = stack.pop().unwrap();
            let mut nexts: Vec<NodeIndex> = sg2
                .neighbors_directed(current_index, Direction::Outgoing)
                .filter(|next_index| !reached.contains(&next_index))
                .collect();
            // println!("{:?}: {:?}", current_index, nexts);
            reached.extend(nexts.iter().cloned());
            stack.append(&mut nexts);
        }
        for node in &reached {
            let activity = sg2.node_weight(*node).unwrap();
            if let ActivityType::Networking(_) = activity.activity_type {
                // println!("  {:?}, {} {} [{}, {}]", node, activity.label, activity.url, activity.start_time, activity.end_time);
            }
        }

        reachables_for_root2.insert(*root_index, reached);
    }

    // for (root1, reachables1) in &reachables_for_root1 {
    //     let root1_activity = sg1.node_weight(*root1).unwrap();
    //     println!("1 {}", root1_activity.url);
    // }

    // for (root2, reachables2) in &reachables_for_root2 {
    //     let root2_activity = sg2.node_weight(*root2).unwrap();
    //     println!("2 {}", root2_activity.url);
    // }

    for (root1, reachables1) in &reachables_for_root1 {
        let root1_activity = sg1.node_weight(*root1).unwrap();
        let mut root2 = None;
        for (candidate, _) in &reachables_for_root2 {
            let root2_activity = sg2.node_weight(*candidate).unwrap();
            if compare_base_urls(&root1_activity.url, &root2_activity.url) {
                root2 = Some(candidate);
                break;
            }
        }

        if let Some(root2) = root2 {
            let reachables2 = reachables_for_root2.get(root2).unwrap();
            
            // println!("{:?} {} {}", root2, sg1.node_weight(*root1).unwrap().url, sg2.node_weight(*root2).unwrap().url);
            for r1_index in reachables1 {
                let r1 = sg1.node_weight(*r1_index).unwrap();
                for r2_index in reachables2 {
                    let r2 = sg2.node_weight(*r2_index).unwrap();
                    // println!("{} {}", r1.url, r2.url);
                    if compare_base_urls(&r1.url, &r2.url) {
                        let rid1 = ids1.get(r1_index).unwrap();
                        let rid2 = ids2.get(r2_index).unwrap();
                        rrs.push((*rid1, *rid2));
                        // println!("  {:?} ({}) {:?} ({}) {} {}", r1_index, rid1, r2_index, rid2, r1.url, r2.url);
                    }
                }
            }
        }
    }

    // for (rr1, rr2) in &rrs {
    //     let t1 = r1.get(rr1).unwrap();
    //     let t2 = r2.get(rr2).unwrap();

    //     if t1.response.body().len() != t2.response.body().len() {
    //         println!("{}\n{}\n{} {}", t1.request, t2.request, t1.response.body().len(), t2.response.body().len());
    //     }
    //     println!();
    // }

    rrs.retain(|(rr1, rr2)| {
        let t1 = r1.get(rr1).unwrap();
        let t2 = r2.get(rr2).unwrap();
        t1.response.body().len() != t2.response.body().len()
    });
    
    Some(rrs)
}

fn compare_base_urls(url1: &String, url2: &String) -> bool {

    let parsed_url1 = match Url::parse(url1) {
        Ok(v) => v,
        Err(_) => return false
    };
    let parsed_url2 = match Url::parse(url2) {
        Ok(v) => v,
        Err(_) => return false
    };

    if parsed_url1.scheme() != parsed_url2.scheme() {
        return false;
    }

    if parsed_url1.host_str().unwrap() != parsed_url2.host_str().unwrap() {
        return false;
    }

    if parsed_url1.path() != parsed_url2.path() {
        return false;
    }

    true
}
