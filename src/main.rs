use std::ops::Deref;

use clap::{
	Command,
	builder::styling::{
		Styles,
		AnsiColor,
		Effects},
	ArgMatches};

mod sub_commands;
mod init;
use colored::Colorize;
use sub_commands::SubCmd;
use init::Init;

fn main() {
	let cmds:Vec<Box<dyn SubCmd>> = vec![
		Box::new(Init::new("init")),
		
		];

	let matches = kitpp(&cmds).get_matches();

	process_matches(cmds,matches);
}

fn kitpp(cmds: &Vec<Box<dyn SubCmd>>) -> Command {
	Command::new("kit++")
		.about("C++ Project and package manager")
		.version("0.1.0")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.author("Ashfmate - Git / Ashforest everything else")
		.styles(
			Styles::styled()
			.header(AnsiColor::Green.on_default() | Effects::BOLD))
		.subcommands(
			cmds
				.iter()
				.map(|item| item.deref().cmd())
			)
}

fn process_matches(cmds: Vec<Box<dyn SubCmd>>,matches: ArgMatches) {
	let (name,matcher) = matches
		.subcommand()
		.expect("There should be a subcommand\nThis error shouldn't appear");
	
	let cmd = cmds
		.into_iter()
		.find(|item| item.get_name() == name);

	if let Some(mut cmd) = cmd {
		cmd.fill_matcher(matcher.clone());
		cmd.process();
	} else {
		eprintln!("{} {}",
			name.red(),
			"does not exist as a subcommand".red());
	}
}