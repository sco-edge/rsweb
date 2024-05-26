use recd;
use std;

fn main() {
    let path1 = std::path::PathBuf::from("/home/sunj/lrs/traces-1/bing.com/run-0");
    let dependency1 = recd::dependency::Dependency::new(path1.join("bing.com.json").as_ref()).unwrap();
    let _resources1 = recd::resource::Resources::new(path1.as_ref(), &dependency1).unwrap();

    let path2 = std::path::PathBuf::from("/home/sunj/lrs/traces-2/bing.com/run-0");
    let dependency2 = recd::dependency::Dependency::new(path2.join("bing.com.json").as_ref()).unwrap();
    let _resources2 = recd::resource::Resources::new(path2.as_ref(), &dependency2).unwrap();

    let deadlines = dependency1.deadlines();
    for deadline in deadlines {
        println!("{}: {:?}", deadline.0, deadline.1);
    }

    let deadlines = dependency2.deadlines();
    for deadline in deadlines {
        println!("{}: {:?}", deadline.0, deadline.1);
    }

    // if let Ok(resources) = recd::resource::parse_transactions_path(path.as_ref()) {
    //     for resource in resources {
    //         println!("{}\nstatus: {} size: {}", resource.request, resource.response.status(), resource.response.body().len());
    //     }
    // }
    
}