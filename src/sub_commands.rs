use clap::{Command, ArgMatches};

trait SubCmd {
	fn cmd() -> Command;
	fn process(matcher: &ArgMatches);
}

struct Init;
struct Build;
struct Run;