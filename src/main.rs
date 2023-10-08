use std::sync::{Mutex, Arc};

use crate::{server::server::Server, utility::threadpool::ThreadPool};

mod cli;
mod server;
mod manager;
mod gateway;
mod utility;

fn main() {
    // Get the arguments into a hashmap:
    let args = cli::get_args();
    // Check for help flag and handle:
    cli::help(&args);
    let mut port_list = Arc::new(Mutex::new(vec![]));
    let instance_count = args.get("max_servers").unwrap().parse::<usize>().unwrap();
    let pool = ThreadPool::new(instance_count);

    for _ in 0..10 {
        let mut pl_clone = port_list.clone();
        pool.execute( move || {
            
            let server = Server::new(&mut pl_clone);

            server.run();
        
        });
    }

}