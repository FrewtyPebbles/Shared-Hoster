use std::sync::{Mutex, Arc};

use manager::manager::Manager;

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

    let max_servers = args.get("max_servers").unwrap().parse::<usize>().unwrap();

    let mut manager = Manager::new(max_servers);

    manager.deploy_all();

}