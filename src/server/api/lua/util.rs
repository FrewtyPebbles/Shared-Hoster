use std::collections::HashMap;

use rlua::ToLua;

pub fn json_to_lua<'a>(ctx: rlua::Context<'a>, json_node: serde_json::Value) -> rlua::Value<'a> {
	return match json_node {
		serde_json::Value::Null => rlua::Value::Nil,
		serde_json::Value::Bool(val) => rlua::Value::Boolean(val),
		serde_json::Value::Number(val) => rlua::Value::Number(val.as_f64().unwrap()),
		serde_json::Value::String(val) => {
			let lua_string: rlua::Value<'_> = val.to_lua(ctx).unwrap();
			return  lua_string;
		},
		serde_json::Value::Array(vect) => {
			let mut new_vect = vec![];
			for val in vect {
				new_vect.push(json_to_lua(ctx, val));
			}
			let lua_array = new_vect.to_lua(ctx).unwrap();
			return lua_array;
		},
		serde_json::Value::Object(map) => {
			let mut new_map = HashMap::new();
			for (key, val) in map {
				new_map.insert(key, json_to_lua(ctx, val));
			}
			let lua_map = new_map.to_lua(ctx).unwrap();
			return lua_map;
		},
	}
}

pub fn lua_to_json(table_node: rlua::Value) -> serde_json::Value {
	return match table_node {
		rlua::Value::Nil => serde_json::Value::Null,
		rlua::Value::Boolean(val) => serde_json::json!(val),
		rlua::Value::Number(val) => serde_json::json!(val),
		rlua::Value::String(val) => serde_json::json!(val.to_str().unwrap().to_string()),
		rlua::Value::Table(table) => {
			return match table.contains_key(1).unwrap() {
				true => {
					// return a list
					let mut new_vect = vec![];
					for val in table.sequence_values() {
						new_vect.push(lua_to_json(val.unwrap()));
					}
					let json_array = serde_json::json!(new_vect);
					json_array
				},
				false => {
					// return a table
					let mut new_map = serde_json::Map::new();
					for kv in table.pairs::<rlua::Value, rlua::Value>() {
						let (key, val) = kv.unwrap();
						if let rlua::Value::String(lks) = key {
							new_map.insert(lks.to_str().unwrap().to_string(), lua_to_json(val));
						}
					}
					let json_map = serde_json::json!(new_map);
					json_map
				},
			};
		},
		rlua::Value::LightUserData(_) => todo!(),
		rlua::Value::Integer(_) => todo!(),
		rlua::Value::Function(_) => todo!(),
		rlua::Value::Thread(_) => todo!(),
		rlua::Value::UserData(_) => todo!(),
		rlua::Value::Error(_) => todo!(),
	}
}