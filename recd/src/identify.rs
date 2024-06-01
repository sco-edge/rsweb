use crate::activity::*;
use crate::dependency::*;
use crate::resource::*;

use std::time::Duration;
use std::collections::HashMap;
use std::path::PathBuf;

use petgraph::graph::NodeIndex;
use petgraph::Direction;

use url;
use url::Url;

pub fn identify_rrs(target: &str, traces: &PathBuf, rtt: usize, lateness: Option<isize>) -> Option<(Vec<Duration>, Duration, Vec<usize>, Duration)> {
    let mut rrs = Vec::new();
    let mut leaf_indices = Vec::new();
    let primary_p = traces.join(target).join("run-0");
    if !primary_p.exists() || !primary_p.join(format!("{}.json", target)).exists() {
        // println!("{} {} does not have profiled output.", target, 0);
        return None;
    }

    let primary_d = Dependency::new(primary_p.join(format!("{}.json", target)).as_ref()).unwrap();
    let primary_r = Resources::new(primary_p.as_ref(), &primary_d).unwrap();
    let (primary_sg, primary_ids) = primary_d.simplified_graph();

    let mut primary_root_indices = Vec::new();
    for nx in primary_sg.node_indices() {
        if primary_sg
            .neighbors_directed(nx, Direction::Incoming)
            .peekable()
            .peek()
            .is_none()
        {
            primary_root_indices.push(nx);
        }
    }

    let mut primary_reachables_for_root = HashMap::new();
    for root_index in &primary_root_indices {
        // let root = sg1.node_weight(*root_index).unwrap();
        // let nx = d1.activity(*ids1.get(root_index).unwrap()).unwrap();
        // println!("{}", nx.label);

        // depth-first search
        let mut reached = Vec::<NodeIndex>::new();
        reached.push(*root_index);

        let mut stack: Vec<NodeIndex> = vec![*root_index];
        while stack.len() > 0 {
            let current_index = stack.pop().unwrap();
            let mut nexts: Vec<NodeIndex> = primary_sg
                .neighbors_directed(current_index, Direction::Outgoing)
                .filter(|next_index| !reached.contains(&next_index))
                .collect();
            // println!("{:?}: {:?}", current_index, nexts);
            reached.extend(nexts.iter().cloned());
            stack.append(&mut nexts);
        }
        for node in &reached {
            let activity = primary_sg.node_weight(*node).unwrap();
            if let ActivityType::Networking(_) = activity.activity_type {
                // println!("  {:?}, {} {} [{}, {}]", node, activity.label, activity.url, activity.start_time, activity.end_time);
            }
        }

        primary_reachables_for_root.insert(*root_index, reached);
    }

    for i in 1..5 {
        let secondary_p = traces.join(target).join(format!("run-{}", i));
        if !secondary_p.exists() || !secondary_p.join(format!("{}.json", target)).exists() {
            // println!("{} {} does not have profiled output.", target, i);
            continue;
        }

        let secondary_d = Dependency::new(secondary_p.join(format!("{}.json", target)).as_ref()).unwrap();
        let secondary_r = Resources::new(secondary_p.as_ref(), &secondary_d).unwrap();
        let (secondary_sg, secondary_ids) = secondary_d.simplified_graph();
    
        let mut secondary_root_indices = Vec::new();
        for nx in secondary_sg.node_indices() {
            if secondary_sg
                .neighbors_directed(nx, Direction::Incoming)
                .peekable()
                .peek()
                .is_none()
            {
                secondary_root_indices.push(nx);
            }
        }

        let mut secondary_reachables_for_root = HashMap::new();
        for root_index in &secondary_root_indices {    
            // depth-first search
            let mut reached = Vec::<NodeIndex>::new();
            reached.push(*root_index);
    
            let mut stack: Vec<NodeIndex> = vec![*root_index];
            while stack.len() > 0 {
                let current_index = stack.pop().unwrap();
                let mut nexts: Vec<NodeIndex> = secondary_sg
                    .neighbors_directed(current_index, Direction::Outgoing)
                    .filter(|next_index| !reached.contains(&next_index))
                    .collect();
                reached.extend(nexts.iter().cloned());
                stack.append(&mut nexts);
            }
    
            secondary_reachables_for_root.insert(*root_index, reached);
        }

        for (primary_root, primary_reachables) in &primary_reachables_for_root {

            let root1_activity = primary_sg.node_weight(*primary_root).unwrap();
            let mut secondary_root = None;
            for (candidate, _) in &secondary_reachables_for_root {
                let secondary_root_activity = secondary_sg.node_weight(*candidate).unwrap();
                if compare_base_urls(&root1_activity.url, &secondary_root_activity.url) {
                    secondary_root = Some(candidate);
                    break;
                }
            }
    
            if let Some(secondary_root) = secondary_root {
                let mut secondary_reachables: Vec<_> = secondary_reachables_for_root.get(secondary_root).unwrap().into_iter().map(|x| Some(x)).collect();

                // println!("P: {} S: {}", primary_reachables.len(), secondary_reachables.len());
                
                for primary_index in primary_reachables {
                    let mut exact_matched = false;
                    let primary = primary_sg.node_weight(*primary_index).unwrap();

                    let primary_id = primary_ids.get(primary_index).unwrap();
                    let primary_t = primary_r.get(primary_id).unwrap();

                    if rrs.iter().any(|&(x, _)| x == *primary_id) {
                        continue;
                    }

                    // Static match
                    for secondary_index_wrapped in &mut secondary_reachables {
                        if let None = secondary_index_wrapped {
                            continue;
                        }
                        let secondary_index = secondary_index_wrapped.unwrap();
                        let secondary_id = secondary_ids.get(secondary_index).unwrap();
                        if !compare_parents(&primary_d, &secondary_d, *primary_id, *secondary_id) {
                            // let secondary = secondary_sg.node_weight(*secondary_index).unwrap();
                            // println!("Different parents {}", secondary.url);
                            continue;
                        }

                        let secondary = secondary_sg.node_weight(*secondary_index).unwrap();

                        // Exact match. It is static resource.
                        if compare_base_urls(&primary.url, &secondary.url) {
                            // println!("Exact match: {}", primary.url);
                            _ = std::mem::replace(secondary_index_wrapped, None);
                            exact_matched = true;
                            break;
                        }
                    }

                    if exact_matched == true {
                        continue;
                    }

                    let mut max_similarity = None;
                    let mut similar_index = None;

                    // RR match
                    let mut secondary_reachables: Vec<_> = secondary_reachables.iter().filter(|x| x.is_some()).collect();
                    // println!("{}", primary.url);
                    for (secondary_i, secondary_index_wrapped) in &mut secondary_reachables.iter().enumerate() {
                        if let None = secondary_index_wrapped {
                            continue;
                        }
                        let secondary_index = secondary_index_wrapped.unwrap();
                        let secondary_id = secondary_ids.get(secondary_index).unwrap();
                        let secondary_t = secondary_r.get(secondary_id).unwrap();
                        let secondary = secondary_sg.node_weight(*secondary_index).unwrap();
                        if !compare_hosts(&primary.url, &secondary.url) {
                            continue
                        }

                        // if !compare_parents(&primary_d, &secondary_d, *primary_id, *secondary_id) {
                        //     let secondary = secondary_sg.node_weight(*secondary_index).unwrap();
                        //     println!("  P {}", secondary.url);
                        //     continue;
                        // }
                        
                        // println!("  {} ", secondary.url);
                        
                        if !compare_extensions(&primary.url, &secondary.url) {
                            // println!("Other reason: {}", secondary.url);
                            continue
                        } else {
                            // println!("Passed: {}", secondary.url);
                        }
    
                        let current_similarity = compare_transactions(&primary_t, &secondary_t);
                        if let Some(v) = max_similarity {
                            if current_similarity > v {
                                max_similarity = Some(current_similarity);
                                similar_index = Some(secondary_i);
                            }
                        } else {
                            max_similarity = Some(current_similarity);
                            similar_index = Some(secondary_i);
                        }
                    }
                    if let Some(secondary_i) = similar_index {
                        // let indexx = secondary_ids.get(secondary_reachables[secondary_i].unwrap()).unwrap();
                        // println!("rr matched:  {}", secondary_d.activity(*indexx).unwrap().url);
                        rrs.push((*primary_id, i));
                        _ = std::mem::replace(&mut secondary_reachables[secondary_i], &None);
                    }
                }
            }
        }
    }

    // for (primary_id, i) in &rrs {
    //     let primary_t = primary_r.get(primary_id).unwrap();
    //     println!("{} {}", i, std::str::from_utf8(&primary_t.path().unwrap()).unwrap());
    // }

    let rrs: Vec<usize> = rrs.iter().map(|(a, _b)| *a).collect();

    let mut rr_deadlines = Vec::new();

    let (deadlines, plt) = primary_d.deadlines(rtt);
    for deadline in &deadlines {
        if rrs.contains(&deadline.0) {
            if *deadline.1 != std::time::Duration::new(0, 0) {    
                // let s = primary_d.node_index(deadline.0).unwrap();
                // let sn = primary_d.graph.node_weight(s).unwrap();
                // println!("{}: {:?} {}", deadline.0, deadline.1, sn.url);
                rr_deadlines.push((*deadline.0, *deadline.1));
            }
        }
    }

    let leaves: Vec<_> = primary_sg.node_indices().filter(|nx| {
            primary_sg.neighbors_directed(*nx, Direction::Outgoing)
            .peekable()
            .peek()
            .is_none()})
        .map(|x| primary_ids.get(&x).unwrap().clone())
        .collect();

    // for leaf in &leaves {
    //     println!("{} {:?} {}", leaf, deadlines.get(leaf).unwrap(), primary_d.activity(*leaf).unwrap().url);
    // }

    // println!("{:?}", rrs);
    for (index, primary_id) in rrs.iter().enumerate() {
        if leaves.contains(&primary_id) {
            leaf_indices.push(index);
            // println!("{} {}", index, primary_d.activity(*primary_id).unwrap().url);
        }
    }

    // println!("leaves ({}) {:?}", leaf_indices.len(), leaf_indices);

    // for primary_id in &rrs {        
        
    //     let primary_t = primary_r.get(primary_id).unwrap();
    //     println!("{} {}", i, std::str::from_utf8(&primary_t.path().unwrap()).unwrap());
    // }

        let delayed_plt = match lateness {
            Some(v) => {
                if rr_deadlines.len() == 0 {
                    plt
                } else {
                    primary_d.delayed_load(rtt, &rr_deadlines, v)
                }
            },
            None => plt,
        };

    let rr_deadlines = rr_deadlines.into_iter().map(|(_a, b)| b).collect();

    Some((rr_deadlines, plt, leaf_indices, delayed_plt))

    // let result: Vec<usize> = rrs.iter()
    //     .filter(|(primary_id, secondary_t)| {
    //         let primary_t = primary_r.get(primary_id).unwrap();
    //         primary_t.response.body().len() != secondary_t.response.body().len()
    //     })
    //     .map(|&(a, _b)| a)
    //     .collect();

    // rrs.retain(|(primary_id, secondary_t)| {
    //     let primary_t = primary_r.get(primary_id).unwrap();
    //     primary_t.response.body().len() != secondary_t.response.body().len()
    // });

    // Some(rrs)
}

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
        // println!("{} {}", parsed_url1.path(), parsed_url2.path());
        return false;
    }

    true
}

fn compare_hosts(url1: &String, url2: &String) -> bool {
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

    true
}

fn compare_extensions(url1: &String, url2: &String) -> bool {
    let parsed_url1 = match Url::parse(url1) {
        Ok(v) => v,
        Err(_) => return false
    };
    let parsed_url2 = match Url::parse(url2) {
        Ok(v) => v,
        Err(_) => return false
    };

    if let Some(v1) = parsed_url1.path_segments() {
        if let Some(v2) = parsed_url2.path_segments() {
            // println!("    {} {}", v1.last().unwrap().split(".").last().unwrap(), v2.last().unwrap().split(".").last().unwrap());
            if v1.last().unwrap().split(".").last().unwrap() != v2.last().unwrap().split(".").last().unwrap() {
                // println!("Not passed: {}", url2);
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    if parsed_url1.host_str().unwrap() != parsed_url2.host_str().unwrap() {
        println!("host");
        return false;
    }

    // if parsed_url1.path() != parsed_url2.path() {
    //     return false;
    // }

    true
}

fn compare_transactions(t1: &Transaction, t2: &Transaction) -> usize {
    if t1.authority() != t2.authority() {
        return 0;
    }

    if std::str::from_utf8(&t1.method().unwrap()).unwrap() != "GET" || std::str::from_utf8(&t2.method().unwrap()).unwrap() != "GET" {
        return 0;
    }

    let path1 = std::str::from_utf8(t1.path().unwrap()).unwrap();
    let path2 = std::str::from_utf8(t2.path().unwrap()).unwrap();

    // It should be exact match. This function is only called in RR
    if path1 == path2 {
        return 0;
    }

    let path_tokens1: Vec<_> = path1.split("/").collect();
    let path_tokens2: Vec<_> = path2.split("/").collect();

    if path_tokens1.len() != path_tokens2.len() {
        return 0;
    }

    let mut similarity = 0;
    for i in 0..path_tokens1.len() {
        if path_tokens1[i] == path_tokens2[i] {
            similarity += 1;
        }
    }

    if similarity > 0 {
        // println!("similarity: {} {} {}", similarity, path1, path2);
        return similarity
    }

    // if t1.response.body().len() == t2.response.body().len() {
    //     return false;
    // }

    // println!("Diff: {} {} {} {} {}", std::str::from_utf8(&t1.path().unwrap()).unwrap(), std::str::from_utf8(&t1.method().unwrap()).unwrap(), t1.response.body().len(), std::str::from_utf8(&t2.method().unwrap()).unwrap(), t2.response.body().len());
    similarity
}

