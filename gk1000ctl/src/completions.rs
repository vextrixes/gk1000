use clap::Args;
use clap_complete::{Generator, Shell, generate};
use std::io;

pub fn print_completions<G: Generator>(generator: G, cmd: &mut clap::Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    #[clap(value_parser)]
    pub shell: Shell,
}
