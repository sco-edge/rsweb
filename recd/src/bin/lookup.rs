use recd;
use std;
use std::path::PathBuf;
use std::str;
use std::fs::File;

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

    #[arg(short, long, default_value_t=0)]
    rtt: usize,
    
    #[arg(long)]
    lateness: Option<isize>,
}

fn main() {
    let args = Args::parse();
    let mut traces = PathBuf::from("/home/sunj/traweb/rsweb/recorder/traces");
    traces.push(args.target);

    for trace in traces.read_dir().unwrap() {
        if let Ok(trace) = trace {
            println!("{}", trace.path().file_name().unwrap().to_string_lossy());
            recd::resource::read_trace(&trace.path());
            println!();
        }
    }
}
