use std::collections::HashMap;

use super::procedure::Procedure;




pub struct API {
	procedures:HashMap<String, Procedure>, // key: api route, value: procedure to run.
	token: String
}

impl API {
	pub fn new(token: String) -> API {
		// Initializes the server instance's API.
		return API {
			procedures: HashMap::new(),
			token
		}
	}
	pub fn add_procedure(&self, endpoint: String, procedure: serde_json::Value) {
		//serializes the procedure into an instance of the procedure class and adds it to the server's procedures.
	}
}