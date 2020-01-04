extern crate lalrpop;

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

use structopt::clap::Shell;

use std::env;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    lalrpop::process_root().unwrap();

    let outdir = env::var_os("OUT_DIR").expect("OUT_DIR environment variable not defined");

    let mut app = cli::Cli::clap();
    app.gen_completions("set_eq", Shell::Bash, &outdir);
    app.gen_completions("set_eq", Shell::Fish, &outdir);
    app.gen_completions("set_eq", Shell::Zsh, &outdir);
}
