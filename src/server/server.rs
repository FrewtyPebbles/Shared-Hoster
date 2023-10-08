use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Read, Write}, sync::{Mutex, Arc}};

use crate::{server::request::Request, utility::threadpool::ThreadPool};

use super::response::Response;

pub struct Server {
	tcp_listener: TcpListener,
	port:u32,
	terminate:Arc<Mutex<bool>>,
	port_list:Arc<Mutex<Vec<u32>>>
}

impl Server {
	pub fn new(port_list:&mut Arc<Mutex<Vec<u32>>>) -> Server {
		let mut port = 8080;
		while port_list.lock().unwrap().contains(&port) {
			port -= 1;
		}

		// Add port to port list.
		port_list.lock().unwrap().push(port);

		// return the server instance.
		return Server {
			tcp_listener: TcpListener::bind(format!("127.0.0.1:{port}")).unwrap(),
			port,
			terminate: Arc::new(Mutex::new(false)),
			port_list: port_list.clone()
		};
	}

	pub fn run(&self) {
		println!("New Instance running on port: {}", self.port);
		let pool = ThreadPool::new(30);
		for stream in self.tcp_listener.incoming() {
			let stream = stream.unwrap();
			let port_clone = self.port.clone();
			let port_list_clone = self.port_list.clone();
			let terminate_clone = self.terminate.clone();
			pool.execute(move || {
				handle_request(stream, port_clone, port_list_clone, terminate_clone);
			});
			if self.terminate.lock().unwrap().clone() {break}
		}
		println!("Instance on port {} has successfully shut down.", self.port)
	}

	pub fn stop(&mut self) {
		// Remove port from port list.
		let index = self.port_list.lock().unwrap().iter().position(|x| *x == self.port).unwrap();
		self.port_list.lock().unwrap().remove(index);

		let mut terminate = self.terminate.lock().unwrap();
		*terminate = true;
		
	}
}

fn stop_server(port:u32, port_list: Arc<Mutex<Vec<u32>>>, terminate: Arc<Mutex<bool>>) {
	// Remove port from port list.
	let index_option = port_list.lock().unwrap().clone().iter().position(|x| *x == port);
	if index_option.is_none() {return;}
	port_list.lock().unwrap().remove(index_option.unwrap());

	let mut terminate_mut = terminate.lock().unwrap();
	*terminate_mut = true;
	
}

fn handle_request(mut stream: TcpStream, port:u32, port_list: Arc<Mutex<Vec<u32>>>, terminate: Arc<Mutex<bool>>) {
	let request = serialize_request(&stream);
	
	let mut response = Response::new();

	response.status = 200;
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

	stream.write_all(response.render().as_bytes()).unwrap();
	if request.method == "UNHOST" {
		stop_server(port, port_list, terminate)
	}
}

// Request handling: 
fn serialize_request(mut stream: &TcpStream) -> Request {
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