use std::collections::HashMap;

use reqwest::header::HeaderMap;
use rlua::{UserData, UserDataMethods, ToLua, ToLuaMulti};


use crate::{server::request::Request, utility::conversions::header_from_hash_map};


// Lua Request
pub struct LuaRequest {
	pub method:String,
	pub url:String,
	pub headers:HashMap<String, String>,
	pub body:String,
	pub status: u16
}
impl LuaRequest {
	pub fn new(request:Request) -> LuaRequest {
		return LuaRequest {
			method: request.method,
			url: request.url,
			headers: request.headers,
			body: request.body,
			status: request.status
		};
	}
}
impl UserData for LuaRequest {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("json", |_, this, _: ()| {
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
impl UserData for LuaCurl {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("run", |_, this, _: ()| {
			
			let client = reqwest::blocking::Client::new();

			// choose the method
			let req = match this.method.to_lowercase().as_str() {
				"post" => client.post(this.url.clone()),
				"put" => client.put(this.url.clone()),
				"patch" => client.patch(this.url.clone()),
				"delete" => client.delete(this.url.clone()),
				_ => client.get(this.url.clone()),
			};

			// send body and headers and get response
			let res = req
				.body(this.body.clone())
				.headers(header_from_hash_map(this.headers.iter()))
				.send().unwrap();
			
			let res_headers = res.headers();
			
			let content_type = res_headers.get("Content-Type")
				.unwrap().to_str().unwrap();
			
			if content_type.starts_with("text/html") {
				Ok(res.text().unwrap())
			} else if content_type.starts_with("application/json") {
				Ok(res.json().unwrap())
			} else {
				Ok(res.text().unwrap())
			}
        });
    }
}
// end Lua Curl