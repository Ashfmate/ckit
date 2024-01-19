use colored::Colorize;
use serde_json::{Map,Value};

pub fn build_compile_command(map: Map<String,Value>) -> String {
	let keys_behaviours: Vec<(&'static str, fn(&Value) -> String)> = vec![
		("CXX_COMPILER"	, compiler	),
		("CXX_STANDARD"	, standard	),
		("MAIN_PATH"	, main_path	),
		("DEBUG_PATH"	, debug_path),
		("PROJECT_NAME"	, proj_name	),
	];

	keys_behaviours
	.into_iter()
	.fold("".to_owned(), |acc, item| {
		acc + &item.1(map.get(item.0).unwrap())
	})
}

fn compiler(value: &Value) -> String {
	let res = value
		.as_str()
		.expect(format!("{}\n{}",
			"The \"CXX_COMPILER\" value must be a path to the compiler as a string".red(),
			"Alternatively you could enter the compiler's name if it exits in /usr/bin".yellow()).as_str());
	format!("{res} ")
}

fn standard(value: &Value) -> String {
	let res = value
		.as_str()
		.expect(format!("{}",
			"The \"CXX_STANDARD\" value must be a string for compatibility reasons".red()).as_str());
	format!("-std=c++{res} ")
}

fn main_path(value: &Value) -> String {
	let res = value
		.as_str()
		.expect(format!("{}",
			"The \"MAIN_PATH\" value must be the path the cpp file with the main function".red()).as_str());
	format!("{res} ")
}

fn debug_path(value: &Value) -> String {
	let res = value
		.as_str()
		.expect(format!("{}",
		"The \"DEBUG_PATH\" value must be the path to the folder where the executable will be in").as_str());
	format!("-o {res}/")
}

fn proj_name(value: &Value) -> String {
	let res = value
		.as_str()
		.expect(format!("{}",
			"The \"PROJECT_NAME\" value must be name of the executable".red()).as_str());
	format!("{res} ")
}
