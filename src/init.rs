use std::{
	fs,
	process,
	env};

use clap::{
	ArgMatches,
	Command,
	Arg,
	ArgAction};

use colored::Colorize;

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

		if status.success() {
			let dir = if dir != "." { dir }
			else {
				env::current_dir()
				.expect("Could not get current dir")
				.components().last().unwrap().as_os_str().to_str().unwrap().to_string()
			};
			println!("{} {dir}", "Successfully kickstarted the project".green());
		} else {
			eprintln!("{}\n{}",
				"Error when creating the project".red(),
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