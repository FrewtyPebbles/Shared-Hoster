use std::{collections::HashMap, sync::{Arc, Mutex}};
use reqwest;
use reqwest::header::HeaderMap;
use rlua::{UserData, UserDataMethods, ToLua, ToLuaMulti, Table};


use crate::{server::request::Request, utility::conversions::header_from_hash_map};

use super::util::{json_to_lua, lua_to_json};


// Lua Request
pub struct LuaRequest {
	pub method:String,
	pub url:String,
	pub headers:HashMap<String, String>,
	pub body:String,
	pub status: u16
}
impl LuaRequest {
	pub fn new(request:&Request) -> LuaRequest {
		return LuaRequest {
			method: request.method.clone(),
			url: request.url.clone(),
			headers: request.headers.clone(),
			body: request.body.clone(),
			status: request.status.clone()
		};
	}
}
impl UserData for LuaRequest {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("json", |_, this, ()| {
            // convert to json
			Ok(())
        });
    }
}
// End lua request

// Lua Curl

pub struct LuaCurl {
	method:String,
	url:String,
	headers:HashMap<String, String>,
	body:String
}
impl LuaCurl {
	pub fn new(method:String, url:String, headers:HashMap<String, String>, body:String) -> LuaCurl {
		return LuaCurl {
			method,
			url,
			headers,
			body
		};
	}
}

impl LuaCurl {
	fn get_response(&self) -> reqwest::blocking::Response {
		let client = reqwest::blocking::Client::new();
			
		// choose the method
		let req = match self.method.to_lowercase().as_str() {
			"post" => client.post(self.url.clone()),
			"put" => client.put(self.url.clone()),
			"patch" => client.patch(self.url.clone()),
			"delete" => client.delete(self.url.clone()),
			_ => client.get(self.url.clone()),
		};

		// send body and headers and get response
		let res = req
			.body(self.body.clone())
			.headers(header_from_hash_map(self.headers.iter()))
			.send().unwrap();
		
		return res
	}
}

impl UserData for LuaCurl {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		//GETTERS AND SETTERS

		// headers
		methods.add_method_mut("set_header", |_, this, (key, value):(String, String)| {
			this.headers.insert(key, value);
			return Ok(());
		});
		
		methods.add_method("get_header", |_, this, (key):(String)| {
			return Ok(this.headers.get(&key).unwrap().clone());
		});

		// url
		methods.add_method_mut("set_url", |_, this, (url):(String)| {
			this.url = url;
			return Ok(());
		});
		
		methods.add_method("get_url", |_, this, ()| {
			return Ok(this.url.clone());
		});

		// method
		methods.add_method_mut("set_method", |_, this, (method):(String)| {
			this.method = method;
			return Ok(());
		});
		
		methods.add_method("get_method", |_, this, ()| {
			return Ok(this.method.clone());
		});

		// body
		methods.add_method_mut("set_body", |_, this, (body):(String)| {
			this.body = body;
			return Ok(());
		});
		
		methods.add_method("get_body", |_, this, ()| {
			return Ok(this.body.clone());
		});

		methods.add_method("run", |_, this, ()| {
			let res = this.get_response();

			Ok(LuaCurlResponse{res:Some(res)})
        });

    }
}

// response struct for lua curl requests
struct LuaCurlResponse{
	res: Option<reqwest::blocking::Response>
}
impl UserData for LuaCurlResponse {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_method("get_content_type", |_, this, ()| {
			match &this.res {
				Some(res) => {
					let res_headers = res.headers();
			
					let content_type = res_headers.get("Content-Type")
						.unwrap().to_str().unwrap().to_string();
					return Ok(content_type);
				},
				None => {
					return Err(rlua::Error::RuntimeError("Response already processed.".to_string()));
				},
			}
        });

		methods.add_method_mut("text", |_, this, ()| {
			match &this.res {
				Some(_) => {
					let text_res = this.res.take().unwrap().text().unwrap();
					return Ok(text_res);
				},
				None => {
					return Err(rlua::Error::RuntimeError("Response already processed.".to_string()));
				},
			}
        });

		methods.add_method_mut::<_,_,rlua::Value,_>("json", |ctx, this, ()| {
			match &this.res {
				Some(_) => {
					let json_res:serde_json::Value = this.res.take().unwrap().json().unwrap();
					let ret_val = json_to_lua(ctx, json_res);
					return Ok(ret_val);
				},
				None => {
					return Err(rlua::Error::RuntimeError("Response already processed.".to_string()));
				},
			}
        });
	}
}
// end Lua Curl

pub struct LuaJson{}
impl UserData for LuaJson {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_method("to_table", |ctx, this, (value):(String)| {
			let ret_val = json_to_lua(ctx, serde_json::from_str(value.as_str()).unwrap());
			return Ok(ret_val);
        });
		methods.add_method("to_string", |ctx, this, (value):(rlua::Value)| {
			let ret_val = serde_json::to_string(&lua_to_json(value)).unwrap();
			return Ok(ret_val);
        });
	}
}