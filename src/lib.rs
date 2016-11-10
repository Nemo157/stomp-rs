extern crate clap;

use clap::{ App, ArgMatches };

pub trait StompCommand {
    fn command() -> App<'static, 'static>;
    fn parse(matches: &ArgMatches) -> Self;
}

pub trait StompCommands {
    fn commands() -> Vec<App<'static, 'static>>;
    fn parse(name: &str, matches: &ArgMatches) -> Self;
}

pub trait ParseApp {
    fn parse() -> Self;
}

impl<C> ParseApp for C where C: StompCommand {
    fn parse() -> Self {
        C::parse(&App::get_matches(C::command()))
    }
}

impl<C> StompCommands for Option<C> where C: StompCommands {
    fn commands() -> Vec<App<'static, 'static>> {
        C::commands()
    }
    fn parse(name: &str, matches: &ArgMatches) -> Self {
        Some(C::parse(name, matches))
    }
}
