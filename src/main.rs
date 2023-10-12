
use manager::manager::Manager;


mod cli;
mod server;
mod manager;
mod gateway;
mod utility;

// TODO: Switch server from a thread pool to asyncronous tokio sockets.


fn main() {
    // Get the arguments into a hashmap:
    let args = cli::get_args();
    // Check for help flag and handle:
    cli::help(&args);

    let max_servers = args.get("max_servers").unwrap().parse::<usize>().unwrap();

    let mut manager = Manager::new(max_servers);

    manager.deploy_all();

}