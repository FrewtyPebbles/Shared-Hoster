use core::time;
use std::{sync::{Arc, Mutex}, thread};

use crate::{utility::threadpool::ThreadPool, server::server::{Server, stop_server}};



pub struct Manager {
	max_servers:usize,
	server_pool:ThreadPool,
	server_identity_list: Arc<Mutex<Vec<(String, u32, Arc<Mutex<bool>>)>>>
}

impl Manager {
	pub fn new(max_servers:usize) -> Manager {
		return Manager {
			max_servers,
			server_pool:ThreadPool::new(max_servers),
			server_identity_list: Arc::new(Mutex::new(vec![]))
		};
	}

	pub fn deploy_server(&mut self) {
		let mut sil_clone = self.server_identity_list.clone();

        self.server_pool.execute( move || {

			let server = Server::new(&mut sil_clone);

            server.run();

        });
	}

	pub fn deploy_all(&mut self) {
		for _ in 0..self.max_servers {
			thread::sleep(time::Duration::from_millis(1));
			self.deploy_server();
		}
	}

	pub fn remove_server_by_token(&mut self, server_token:String) -> Result<(), &'static str> {
		for (index, (token, port, terminate)) in self.server_identity_list.lock().unwrap().iter().enumerate() {
			if server_token == *token {
				stop_server(*port, &self.server_identity_list, terminate);
				return Ok(());
			}
		}
		return Err("A server with the token provided does not exist.");
	}

	pub fn remove_server_by_port(&mut self, server_port:u32) -> Result<(), &'static str> {
		for (index, (token, port, terminate)) in self.server_identity_list.lock().unwrap().iter().enumerate() {
			if server_port == *port {
				stop_server(*port, &self.server_identity_list, terminate);
				return Ok(());
			}
		}
		return Err("A server with the port provided does not exist.");
	}
}