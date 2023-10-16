use std::collections::HashMap;

use crate::server::request::Request;

use super::procedure::{Procedure, LuaProcedure};




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

	pub fn fulfill_api_request(&self, endpoint: String, req:Request) {
		match self.procedures.get(&endpoint).unwrap() {
			Procedure::Dynamic(proc) => {
				//run dynamic procedure
			},
			Procedure::Lua(proc) => {
				proc.run(req);
			},
		}
	}

	pub fn add_dynamic_procedure(&self, endpoint: String, procedure: serde_json::Value) {
		//serializes the procedure into an instance of the procedure class and adds it to the server's procedures.
	}

	pub fn add_lua_procedure(&mut self, endpoint: String, path: String) {
		self.procedures.insert(endpoint, Procedure::Lua(LuaProcedure::new(path)));
	}
}