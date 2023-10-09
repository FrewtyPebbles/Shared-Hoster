use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Read, Write}, sync::{Mutex, Arc}};


use crate::{server::request::Request, utility::threadpool::ThreadPool};

use super::response::Response;

pub struct Server {
	tcp_listener: TcpListener,
	pub port: u32,
	pub terminate: Arc<Mutex<bool>>,
	server_identity_list: Arc<Mutex<Vec<(String, u32, Arc<Mutex<bool>>)>>>,
	pub token: String
}

impl Server {
	pub fn new(server_identity_list:&mut Arc<Mutex<Vec<(String, u32, Arc<Mutex<bool>>)>>>) -> Server {
		let mut port = 65535;

		while server_identity_list.lock().unwrap().iter().any(|(t, p, ter)| *p == port) {
			port -= 1;
		}

		let server = Server {
			tcp_listener: TcpListener::bind(format!("127.0.0.1:{port}")).unwrap(),
			port,
			terminate: Arc::new(Mutex::new(false)),
			server_identity_list: server_identity_list.clone(),
			token: sha256::digest(format!("server:{port}")),
		};

		// Add server identity to server identity list.
		server_identity_list.lock().unwrap().push((server.token.clone(), server.port.clone(), server.terminate.clone()));

		// return the server instance.
		return server
	}

	pub fn run(&self) {
		println!("New Instance running on port: {}\ntoken:{}", self.port, self.token);
		let pool = ThreadPool::new(30);
		for stream in self.tcp_listener.incoming() {
			let stream = stream.unwrap();
			let port_clone = self.port.clone();
			let token_clone = self.token.clone();
			let server_identity_list_clone = self.server_identity_list.clone();
			let terminate_clone = self.terminate.clone();
			pool.execute(move || {
				handle_request(stream, port_clone, server_identity_list_clone, terminate_clone, token_clone);
			});
			if *self.terminate.lock().unwrap() {break}
		}
		println!("Instance on port {} has successfully shut down.", self.port)
	}

	pub fn stop(&mut self) {
		// Remove port from server identity list.
		let current_list = self.server_identity_list.lock().unwrap().clone();
		for (index, (token, port, terminate)) in current_list.iter().enumerate() {
			if self.port == *port {
				self.server_identity_list.lock().unwrap().remove(index);
				break;
			}
		}

		let mut terminate = self.terminate.lock().unwrap();
		*terminate = true;
		
	}
}

pub fn stop_server(server_port:u32, server_identity_list: &Arc<Mutex<Vec<(String, u32, Arc<Mutex<bool>>)>>>, server_terminate: &Arc<Mutex<bool>>) {
	// Remove port from server identity list.

	let mut terminate_mut = server_terminate.lock().unwrap();
	*terminate_mut = true;

	let current_list = server_identity_list.lock().unwrap().clone();
	for (index, (token, port, terminate)) in current_list.iter().enumerate() {
		if server_port == *port {
			server_identity_list.lock().unwrap().remove(index);
			break;
		}
	}

	dbg!(server_identity_list);
	
}

fn handle_request(mut stream: TcpStream, port:u32, server_identity_list: Arc<Mutex<Vec<(String, u32, Arc<Mutex<bool>>)>>>, terminate: Arc<Mutex<bool>>, token: String) {
	let request_result = serialize_request(&stream);

	let mut response = Response::new();

	response.status = 200;

	// check if there were any problems with the request
	match request_result {
		Err(status) => {
			// request status is bad
			response.status = status;
			response.body = "Bad Request.".to_string();
			stream.write_all(response.render().as_bytes()).unwrap();
		}
		Ok(request) => {
			// request status in the 200 range
		
			response.headers.push("Access-Control-Allow-Origin: *".to_string());
			response.headers.push("Content-Type: text/html; charset=utf-8".to_string());
			response.body = format!("<!DOCTYPE html>
				<html lang=\"en\">
				<head>
					<meta charset=\"UTF-8\">
					<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
					<meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\">
					<title>Instance {}</title>
				</head>
				<body>
					This is a DomBuilder Instance on port {}.
					<script></script>
				</body>
				</html>", port, port).to_string();
		
			
			if request.method == "UNHOST" {
				if request.body == token {
					response.body = "Server Will shut down on next request.".to_string();
					stream.write_all(response.render().as_bytes()).unwrap();
					stop_server(port, &server_identity_list, &terminate);
				} else {
					response.body = "UNHOST request requires the correct server token in the body.".to_string();
					stream.write_all(response.render().as_bytes()).unwrap();
				}
			} else {
				stream.write_all(response.render().as_bytes()).unwrap();
			}
		
		}
	};
}

// Request handling: 
fn serialize_request(mut stream: &TcpStream) -> Result<Request, u16> {
	let mut buf_reader = BufReader::new(&mut stream);
	
	
	let http_request: Vec<_> = buf_reader.by_ref()
		.lines()
		.map(|result| result.unwrap())
		.take_while(|line| !line.is_empty())
		.collect();

	let mut content_length = 0;

	for header in &http_request {
		if header.starts_with("Content-Length") {
			let mut content_itterator = header.split(": ");
			content_itterator.next();
			content_length = content_itterator.next().unwrap().trim().parse::<usize>().unwrap();
			break;
		}
	}

	let mut body = vec![0; content_length];
	
	buf_reader.read_exact(&mut body).unwrap();

	let raw_http_request = http_request.join("\n") + "\n\n" + &String::from_utf8(body).unwrap();


	return Request::from(raw_http_request);
}