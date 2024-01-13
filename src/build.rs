use std::{process::{exit, self}, fs};

use clap::{ArgMatches, Command, Arg, ArgAction};
use colored::Colorize;
use serde_json::{Map, Value};

use super::sub_commands::SubCmd;

pub struct Build {
	matcher: Option<ArgMatches>,
	pub name: &'static str,
}

impl Build {
	pub fn new(name: &'static str) -> Self { 
		Self { matcher: None, name } 
	}

	fn build_compile_command(&self, map: Map<String,Value>) -> String {
		let mut builtup_command = String::new();
		if let Some(compiler) = map.get("CXX_COMPILER") {
			let compiler = compiler
				.as_str()
				.expect(format!("{}\n{}",
					"The \"CXX_COMPILER\" value must be a path to the compiler as a string".red(),
					"Alternatively you could enter the compiler's name if it exits in /usr/bin".yellow()).as_str());
			builtup_command.push_str(format!("{compiler} ").as_str());
			builtup_command.push(' ');
		}

		if let Some(standard) = map.get("CXX_STANDARD") {
			let standard = standard
				.as_number()
				.expect(format!("{}",
					"The \"CXX_STANDARD\" value must be a number".red()).as_str())
				.as_i64()
				.expect(format!("{}","\"CXX_STANDARD\" value must be integral").as_str());
			builtup_command.push_str(format!("-std=c++{standard} ").as_str());
		}

		if let Some(main_path) = map.get("MAIN_PATH") {
			let main_path = main_path
				.as_str()
				.expect(format!("{}",
					"The \"MAIN_PATH\" value must be the path the cpp file with the main function".red()).as_str());
			builtup_command.push_str(format!("{main_path} ").as_str());
		}

		if let Some(debug_path) = map.get("DEBUG_PATH") {
			let debug_path = debug_path
			.as_str()
			.expect(format!("{}",
			"The \"DEBUG_PATH\" value must be the path to the folder where the executable will be in").as_str());
			builtup_command.push_str(format!("-o {debug_path}/").as_str())
		}
		if let Some(exec_name) = map.get("EXECUTABLE_NAME") {
			let exec_name = exec_name
				.as_str()
				.expect(format!("{}",
					"The \"EXECUTABLE_NAME\" value must be the name of the executable in string").as_str());
			builtup_command.push_str(format!("{exec_name} ").as_str())
		}

		builtup_command
	}
}



impl SubCmd for Build {
	fn cmd(&self) -> Command {
		Command::new(self.get_name())
		.about("Builds the c++ project")
		.arg(
			Arg::new("proj_path")
			.action(ArgAction::Set)
			.default_value(".")
			.num_args(..=1)
		)
	}

	fn process(&self) {
		// Get the parsed option and arguments
		let matcher = self
			.matcher
			.as_ref()
			.expect("Function process must be used after fill_matcher");
		// Get the path from the "proj_path" argument
		let path = matcher
			.get_one::<String>("proj_path")
			.unwrap()
			.to_string();

		// Check if the user mistakingly put "config.json" as the path
		if path.ends_with("config.json") {
			eprintln!("{}\n{}",
			"Please only enter the path to the config.json file".yellow(),
			"You can use [kit++ build] and it will look for config.json in the current directory");
			exit(1);
		}

		let contents = fs::read_to_string(path + "/config.json")
			.expect("config.json file must have permissions to read");

		let map: Map<String, Value> = serde_json::from_str(&contents)
			.expect("The config.json file is not a proper json file");

		let res = self.build_compile_command(map);

		process::Command::new("sh")
		.arg("-c")
		.arg(res)
		.spawn()
		.expect("Could not start compilation command")
		.wait()
		.expect("Did not start compilation command");
	}

	fn get_name(&self) -> &'static str {
		self.name
	}

	fn fill_matcher(&mut self, matcher: ArgMatches) {
		let _ = self.matcher.insert(matcher);
	}
}