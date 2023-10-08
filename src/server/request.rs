use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
	pub method:String,
	pub url:String,
	pub headers:Vec<String>,
	pub body:String
}

impl Request {
	pub fn from(raw:String) -> Request {
		let processed_raw = raw.replace("\r", "");
		let mut body_split = processed_raw.split("\n\n");
		// Includes start_line and headers:
		let raw_headers = body_split.next().unwrap();

		// Get the body option to check if a body exists.
		let body_option = body_split.next();

		// Check if body exists and set if it does:
		let body = if body_option.is_none() {
			String::new()
		} else {
			body_option.unwrap().to_string()
		};

		let mut raw_headers_list = raw_headers.split("\n");

		// Get the itterator for the start line.
		let mut start_line = raw_headers_list.next().unwrap().split_whitespace();


		// get the headers after the start line
		let headers = raw_headers_list.map(|s| s.to_string()).collect::<Vec<String>>();

		return Request {
			// method is the first item in start_line:
			method: start_line.next().unwrap().to_string(),
			// url is second item in start_line:
			url: start_line.next().unwrap().to_string(),
			headers,
			body
		};
	}
}