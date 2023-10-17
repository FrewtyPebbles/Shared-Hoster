use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
	pub method:String,
	pub url:String,
	pub headers:HashMap<String, String>,
	pub body:String,
	pub status: u16 // This is the current status of the request, as the request is processed it may change.
}

impl Request {
	pub fn from(raw:String) -> Result<Request, u16> {
		//This dictates what status code to return with
		let mut status = 200;

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
		let headers = raw_headers_list.map(|s| {
			let mut header_iter = s.split_once(": ");
			let header_key = "";
			let header_val = "";
			
			match header_iter {
				None => {
					status = 400;
					return ("".to_string(),"".to_string());
				}
				Some((hk, hv)) => {
					return (hk.to_string(), hv.to_string());
				}
			}
		}).collect::<HashMap<String, String>>();

		if status != 200 {
			return Err(status);
		}
		
		return Ok(Request {
			// method is the first item in start_line:
			method: start_line.next().unwrap().to_string(),
			// url is second item in start_line:
			url: start_line.next().unwrap().to_string(),
			headers,
			body,
			status
		});
	}
}