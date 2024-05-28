use recd;
use std;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    target: String,

    #[arg(short, long)]
    list: bool,

    #[arg(short, long)]
    exact: bool,

    #[arg(short, long, default_value_t=10)]
    timeout: usize,
}

fn rr_summary(target: &str) -> Vec<Duration> {
    let p1 = std::path::PathBuf::from(format!("/home/sunj/traces/traces-1/{}/run-0", target));
    let d1 = recd::dependency::Dependency::new(p1.join(format!("{}.json", target)).as_ref()).unwrap();
    let r1 = recd::resource::Resources::new(p1.as_ref(), &d1).unwrap();

    let p2 = std::path::PathBuf::from(format!("/home/sunj/traces/traces-2/{}/run-0", target));
    let d2 = recd::dependency::Dependency::new(p2.join(format!("{}.json", target)).as_ref()).unwrap();
    let r2 = recd::resource::Resources::new(p2.as_ref(), &d2).unwrap();

    let rr_tuples = recd::differences::compare_dependencies(&d1, &r1, &d2, &r2).unwrap();
    let rrs: Vec<usize> = rr_tuples.iter().map(|(a, _)| *a).collect::<std::collections::HashSet<_>>().into_iter().collect();

    // println!("{:?}", rrs);

    let mut rr_deadlines = Vec::new();

    let deadlines = d1.deadlines();
    for deadline in deadlines {
        if rrs.contains(&deadline.0) {
            // let s = d1.node_index(deadline.0).unwrap();
            // let sn = d1.graph.node_weight(s).unwrap();
            // println!("{}: {:?} {}", deadline.0, deadline.1, sn.url);
            if deadline.1 != std::time::Duration::new(0, 0) {
                rr_deadlines.push(deadline.1);
            }
        }
    }

    rr_deadlines
}

fn measure_execution_time<F>(func: F, input: &str, timeout_duration: Duration) -> Result<Vec<Duration>, &'static str>
where
    F: FnOnce(&str) -> Vec<Duration> + Send + 'static,
{
    let (sender, receiver): (Sender<Result<Vec<Duration>, &'static str>>, Receiver<Result<Vec<Duration>, &'static str>>) = mpsc::channel();

    let sender_clone = sender.clone();
    let input_string = input.to_string();
    thread::spawn(move || {
        let result = func(&input_string);
        sender_clone.send(Ok(result)).unwrap();
    });

    let sender_clone = sender.clone();
    thread::spawn(move || {
        thread::sleep(timeout_duration);
        let _ = sender_clone.send(Err("Function execution timed out.")); 
    });

    receiver.recv().unwrap()
}

fn main() {
    let args = Args::parse();
    let timeout = Duration::from_secs(args.timeout as u64);
    if !args.list {
        match measure_execution_time(rr_summary, &args.target, timeout) {
            Ok(res) => println!("{} ({}) {:?}", args.target, res.len(), res),
            Err(_) => println!("{} failed", args.target),
        } 
    } else {
        let file_path = args.target;

        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Failed to open the file.");
                return;
            }
        };
        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    match measure_execution_time(rr_summary, &line, timeout) {
                        Ok(res) => println!("{} ({}) {:?}", line, res.len(), res),
                        Err(_) => println!("{} failed", line),
                    } 
                }
                Err(_) => {
                    eprintln!("Failed to read line.");
                    return;
                }
            }
        }
    }    
}