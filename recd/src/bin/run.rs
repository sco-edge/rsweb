use recd;

fn main() {
    let path = "/home/sunj/lrs/traces-1/bing.com/run-0";
    let _res = recd::resource::parse_transactions_path(path.as_ref());
}