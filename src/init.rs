use std::{
	fs::{self, File},
	process,
	env};

use clap::{
	ArgMatches,
	Command,
	Arg,
	ArgAction};

use colored::Colorize;
use serde_json::{Map, Value};

use super::sub_commands::SubCmd;

pub struct Init {
	matcher: Option<ArgMatches>,
	pub name: &'static str
}

impl Init {
	pub fn new(name: &'static str) -> Self { 
		Self { matcher: None, name }
	}
}

impl SubCmd for Init{
	fn cmd(&self) -> Command {
		Command::new(self.get_name())
		.about("Creates a c++ project")
		.arg(
			Arg::new("name")
			.action(ArgAction::Set)
			.default_value(".")
			.num_args(..=1)
		)
		.arg(
			Arg::new("template")
			.short('t')
			.long("template")
			.action(ArgAction::Set)
			.num_args(1)
			.default_value("console")
		)
	}

	fn process(&self) {
		let matcher = self.matcher.as_ref().unwrap();
		let dir = matcher
		.get_one::<String>("name")
		.unwrap()
		.to_string();

		if dir != "." {
			let res = fs::create_dir(&dir);
			if res.is_err() {
				eprintln!("{} [{}]", "Could not create folder".red(), dir.red());
				eprintln!("{res:#?}");
				return;
			}
		}

		let template_path = format!("/home/ashforest/.kit++/templates/{}",
			matcher
			.get_one::<String>("template")
			.unwrap().to_string()
		);

		let status = process::Command::new("sh")
		.arg("-c")
		.arg(format!("cp -r {template_path}/* {dir}"))
		.spawn()
		.expect("cp command faild to start")
		.wait()
		.expect("cp command did not start");

		let proj_name = {
			if dir != "." { dir.clone() }
			else {
				env::current_dir()
				.expect("Insufficient permissions or directory doesn't exist")
				.components().last().unwrap()
				.as_os_str().to_str().unwrap().to_string()
			}
		};

		if status.success() {
			println!("{} {proj_name}", 
				"Successfully kickstarted the project".green());
		} else {
			eprintln!("{}\n{}",
				"Error when creating the project".red(),
				status.to_string());
		}

		let path = format!("{dir}/config.json");
		let buffer = fs::read_to_string(&path)
			.expect("Insufficient permissions for config.json file or it doesn't exist");

		let mut json: Map<String, Value> = serde_json::from_str(&buffer)
			.expect("The config file is not a correct json file");

		json.insert(String::from("PROJECT_NAME"), Value::String(proj_name));

		let configs = File::create(dir + "/config.json")
			.expect("Could not open the file config.json");

		serde_json::to_writer_pretty(configs, &json)
			.expect("Could not serialize config.json file");
	}

	fn get_name(&self) -> &'static str {
		self.name
	}

	fn fill_matcher(&mut self, matcher: ArgMatches) {
		let _ = self.matcher.insert(matcher);
	}
}