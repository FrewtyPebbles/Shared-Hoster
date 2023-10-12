use std::collections::HashMap;

use super::action::Action;

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

pub struct Procedure {
	actions: Vec<ProcedureNode>,
	state: HashMap<String, ProcedureVarType>
}