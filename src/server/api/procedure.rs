
use std::{collections::HashMap, fs::read_to_string};

use rlua::Lua;

use crate::server::request::Request;

use super::{action::Action, lua::tables::{LuaRequest, LuaCurl}};

pub enum ProcedureNode {
	Execute(Action),
	Condition(Condition)// condition, if true, else
}

pub enum ConditionType {
	Equal(ProcedureVarType, ProcedureVarType),
	GreaterThan(ProcedureVarType, ProcedureVarType),
	LessThan(ProcedureVarType, ProcedureVarType),
	NotEqual(ProcedureVarType, ProcedureVarType),
	And(Vec<Condition>),
	Or(Vec<Condition>)
}

pub struct Condition {
	statement_type:ConditionType,
	args:Vec<ConditionType>,
	true_procedure:Vec<ProcedureNode>,
	false_procedure:Vec<ProcedureNode>,
}

pub enum ProcedureVarType {
	Int(i32),
	String(String),
	Float(f64),
	Bool(bool),
	List(Vec<ProcedureVarType>),
	Map(HashMap<ProcedureVarType, ProcedureVarType>)
}

pub struct DynamicProcedure {
	actions: Vec<ProcedureNode>,
	state: HashMap<String, ProcedureVarType>
}

pub struct LuaProcedure {
	lua:Lua,
	file:String
}


impl LuaProcedure {
	pub fn new(file:String) -> LuaProcedure {
		return LuaProcedure {
			lua:Lua::new(),
			file
		};
	}
	pub fn run(&self, request: Request) {
		
		self.lua.context(|lua_context| {

			let create_curl = lua_context.create_function(|_, (method, url, headers, body):(String, String, HashMap<String,String>, String)|{
				return Ok(LuaCurl::new(method, url, headers, body));
			});

			lua_context.globals().set("REQUEST", LuaRequest::new(request)).unwrap();
			lua_context.globals().set("Curl", create_curl.unwrap()).unwrap();
			lua_context.load(read_to_string(&self.file).unwrap().as_str()).exec()
		}).unwrap();
	}
}

pub enum Procedure {
	Dynamic(DynamicProcedure),
	Lua(LuaProcedure)
}