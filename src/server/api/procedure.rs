
use std::{collections::HashMap, fs::read_to_string, sync::{Arc, Mutex}};

use rlua::Lua;
use rlua_async::ChunkExt;

use crate::server::request::Request;

use super::{action::Action, lua::tables::{LuaRequest, LuaCurl, LuaJson}};

#[derive(Clone)]
pub enum ProcedureNode {
	Execute(Action),
	Condition(Condition)// condition, if true, else
}

#[derive(Clone)]
pub enum ConditionType {
	Equal(ProcedureVarType, ProcedureVarType),
	GreaterThan(ProcedureVarType, ProcedureVarType),
	LessThan(ProcedureVarType, ProcedureVarType),
	NotEqual(ProcedureVarType, ProcedureVarType),
	And(Vec<Condition>),
	Or(Vec<Condition>)
}

#[derive(Clone)]
pub struct Condition {
	statement_type:ConditionType,
	args:Vec<ConditionType>,
	true_procedure:Vec<ProcedureNode>,
	false_procedure:Vec<ProcedureNode>,
}

#[derive(Clone)]
pub enum ProcedureVarType {
	Int(i32),
	String(String),
	Float(f64),
	Bool(bool),
	List(Vec<ProcedureVarType>),
	Map(HashMap<ProcedureVarType, ProcedureVarType>)
}

#[derive(Clone)]
pub struct DynamicProcedure {
	actions: Vec<ProcedureNode>,
	state: HashMap<String, ProcedureVarType>
}

#[derive(Clone)]
pub struct LuaProcedure {
	file:String
}


impl LuaProcedure {
	pub fn new(file:String) -> LuaProcedure {
		return LuaProcedure {
			file
		};
	}
	pub fn run(&self, request: &Request) -> String {
		let lua = Lua::new();

		let res = Arc::new(Mutex::new(String::new()));
		let res_clone = res.clone();
		let res_clone2 = res.clone();
		let res_clone3 = res.clone();
		
		lua.context(|lua_context| {

			let create_curl = lua_context.create_function(|_, (method, url, headers, body):(String, String, HashMap<String,String>, String)|{
				return Ok(LuaCurl::new(method, url, headers, body));
			});

			let set_response = lua_context.create_function(move |_, (response):(String)|{
				*res_clone.lock().unwrap() = response;
				return Ok(());
			});

			let get_response = lua_context.create_function( move |_, ()|{
				let current_response = res_clone2.lock().unwrap().clone();
				return Ok(current_response);
			});

			let append_response = lua_context.create_function( move |_, (response):(String)|{
				*res_clone3.lock().unwrap() = format!("{}{}", *res_clone3.lock().unwrap(), response);
				return Ok(());
			});

			//TABLES
			lua_context.globals().set("REQUEST", LuaRequest::new(request)).unwrap();
			lua_context.globals().set("JSON", LuaJson{}).unwrap();
			lua_context.globals().set("Curl", create_curl.unwrap()).unwrap();

			//Functions
			lua_context.globals().set("set_response", set_response.unwrap()).unwrap();
			lua_context.globals().set("get_response", get_response.unwrap()).unwrap();
			lua_context.globals().set("append_response", append_response.unwrap()).unwrap();
			lua_context.load(read_to_string(&self.file).unwrap().as_str()).exec().unwrap();
		});
		return res.lock().unwrap().clone();
	}
}

#[derive(Clone)]
pub enum Procedure {
	Dynamic(DynamicProcedure),
	Lua(LuaProcedure)
}