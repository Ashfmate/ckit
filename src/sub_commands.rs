use clap::{Command, ArgMatches};

pub trait SubCmd {
	fn cmd(&self) -> Command;
	fn process(&self);
	fn get_name(&self) -> &'static str;
	fn fill_matcher(&mut self, matcher: ArgMatches);
}