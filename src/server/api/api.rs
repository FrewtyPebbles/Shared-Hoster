use std::collections::HashMap;

use tokio::runtime::Runtime;

use crate::server::request::Request;

use super::procedure::{Procedure, LuaProcedure};



#[derive(Clone)]
pub struct API {
	procedures:HashMap<String, Procedure>, // key: api route, value: procedure to run.
	token: String,
}

impl API {
	pub fn new(token: String) -> API {
		// Initializes the server instance's API.
		return API {
			procedures: HashMap::new(),
			token
		}
	}

	pub async fn fulfill_api_request(&self, req: &Request) -> String {
		let procedure_enum = self.procedures.get(&req.url).unwrap().clone();
		match procedure_enum {
			Procedure::Dynamic(proc) => {
				//run dynamic procedure
				return String::new();
			},
			Procedure::Lua(proc) => {
				let req_clone = req.clone();
				return tokio::task::spawn_blocking(move||{
					return proc.run(&req_clone).to_string();
				}).await.unwrap();
			},
		}
	}

	pub fn add_dynamic_procedure(&self, endpoint: String, procedure: serde_json::Value) {
		//serializes the procedure into an instance of the procedure class and adds it to the server's procedures.
	}

	pub fn add_lua_procedure(&mut self, endpoint: String, path: String) {
		self.procedures.insert(format!("/api/{}", endpoint), Procedure::Lua(LuaProcedure::new(path)));
	}
}