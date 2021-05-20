#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate structopt;

#[macro_use]
extern crate lalrpop_util;

#[cfg(feature = "pa-eq")]
extern crate dbus;

#[cfg(feature = "pa-effects")]
#[macro_use]
extern crate serde_json;

mod cli;
mod parsing;
mod utils;

#[cfg(feature = "pa-effects")]
mod pa_effects;

use crate::cli::*;

use failure::Error;
use structopt::StructOpt;

#[derive(Debug)]
pub struct Filter {
    preamp: f64,
    frequencies: Vec<u32>,
    coefficients: Vec<f64>,
}

fn main() {
    match start() {
        Ok(()) => (),
        Err(err) => {
            error!("{}", err);
            for cause in err.iter_causes() {
                error!("Caused by: {}", cause);
            }
            std::process::exit(-1);
        }
    }
}

fn start() -> Result<(), Error> {
    let args = Cli::from_args();
    pretty_env_logger::formatted_builder()
        .filter(
            None,
            match args.verbose {
                0 => log::LevelFilter::Warn,
                1 => log::LevelFilter::Info,
                2 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            },
        )
        .try_init()?;


    use crate::Command::*;

    match args.cmd {
        #[cfg(feature = "pa-eq")]
        PaEq(args) => pa_eq::main(args),
        #[cfg(feature = "pa-effects")]
        PaEffects(args) => pa_effects::main(args),
    }
}
