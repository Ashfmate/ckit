use std::{process::{exit, self}, fs};

use clap::{ArgMatches, Command, Arg, ArgAction};
use colored::Colorize;
use serde_json::{Map, Value};

use crate::config_parser::build_compile_command;

use super::sub_commands::SubCmd;

pub struct Build {
	matcher: Option<ArgMatches>,
	pub name: &'static str,
}

impl Build {
	pub fn new(name: &'static str) -> Self { 
		Self { matcher: None, name } 
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

		let res = build_compile_command(map);

		let status = process::Command::new("sh")
		.arg("-c")
		.arg(res)
		.spawn()
		.expect("Could not start compilation command")
		.wait()
		.expect("Did not start compilation command");

		if status.success() {
			println!("Successfully built the project");
		} else {
			eprintln!("{}\n{}",
				"Error when building the project".red(),
				status.to_string());
		}
	}

	fn get_name(&self) -> &'static str {
		self.name
	}

	fn fill_matcher(&mut self, matcher: ArgMatches) {
		let _ = self.matcher.insert(matcher);
	}
}