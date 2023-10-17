use std::collections::HashMap;

pub struct Response {
	pub status:u16,
	pub headers: Vec<String>,
	pub body: String
}

impl Response {

	pub fn new() -> Response {
		Response {
			status: 200,
			headers: vec![] as Vec<String>,
			body: String::new()
		}
	}

	fn get_status_msg(&self) -> String {
		return String::from(match self.status {

			// Information responses:
			100 => "Continue",
			101 => "Switching Protocols",
			102 => "Processing",
			103 => "Early Hints",

			// Successful responses:
			200 => "OK",
			201 => "Created",
			202 => "Accepted",
			203 => "Non-Authoritative Information",
			204 => "No Content",
			205 => "Reset Content",
			206 => "Partial Content",
			207 => "Multi-Status",
			208 => "Already Reported",
			226 => "IM Used",

			// Redirect responses:
			300 => "Multiple Choices",
			301 => "Moved Permanently",
			302 => "Found",
			303 => "See Other",
			304 => "Not Modified",
			307 => "Temporary Redirect",
			308 => "Permanent Redirect",

			// Client Error responses:
			400 => "Bad Request",
			401 => "Unauthorized",
			402 => "Payment Required",
			403 => "Forbidden",
			404 => "Not Found",
			405 => "Method Not Allowed",
			406 => "Not Acceptable",
			407 => "Proxy Authentication Required",
			408 => "Request Timeout",
			409 => "Conflict",
			410 => "Gone",
			411 => "Length Required",
			412 => "Precondition Failed",
			413 => "Payload Too Large",
			414 => "URI Too Long",
			415 => "Unsupported Media Type",
			416 => "Range Not Satisfiable",
			417 => "Expectation Failed",
			418 => "I'm a teapot",
			421 => "Misdirected Request",
			422 => "Unprocessable Content",
			423 => "Locked",
			424 => "Failed Dependency",
			425 => "Too Early",
			426 => "Upgrade Required",
			428 => "Precondition Required",
			429 => "Too Many Requests",
			431 => "Request Header Fields Too Large",
			451 => "Unavailable For Legal Reasons",

			// Server Error responses:
			500 => "Internal Server Error",
			501 => "Not Implemented",
			502 => "Bad Gateway",
			503 => "Service Unavailable",
			504 => "Gateway Timeout",
			505 => "HTTP Version Not Supported",
			506 => "Variant Also Negotiates",
			507 => "Insufficient Storage",
			508 => "Loop Detected",
			510 => "Not Extended",
			511 => "Network Authentication Required",
			_ => ""
		})
	}

	pub fn render(&self) -> String {
		let status = format!("HTTP/1.1 {} {}", self.status, self.get_status_msg());
		
		// create a string of headers except content-length:
		let headers = if self.headers.is_empty() {
				"".to_string()
			} else {
				format!("{}\n", self.headers.join("\n"))
			};
		
		return format!("{}\n{}Content-Length: {}\n\n{}", status, headers, self.body.len(), self.body);
	}
}