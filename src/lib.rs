extern crate clap;

pub trait Executor {
    type Context;
    fn run(self, context: Self::Context);
}

pub trait StompCommand {
    fn command() -> clap::App<'static, 'static>;
    fn parse(matches: clap::ArgMatches) -> Self;
}

pub trait StompCommands {
    fn commands() -> Vec<clap::App<'static, 'static>>;
    fn parse(matches: clap::ArgMatches) -> Self;
}

pub trait RunApp {
    fn run_app();
}

impl<C> RunApp for C where C: StompCommand + Executor<Context = ()> {
    fn run_app() {
        let app = <C as StompCommand>::command();
        let matches = clap::App::get_matches(app);
        let app = <C as StompCommand>::parse(matches);
        <C as Executor>::run(app, ());
    }
}
