use recd;
use std;
use std::io;
use std::fs;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::str;

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

    let transactions = recd::resource::parse_transactions_path(&traces).unwrap();

    for transaction in &transactions {
        if let Ok(v) = str::from_utf8(transaction.response().body()) {
            println!("{}", v);
        }
    }

}
