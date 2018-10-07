extern crate dbus;

#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate clap;
extern crate clap_log_flag;
extern crate clap_verbosity_flag;
#[macro_use]
extern crate structopt;

extern crate lalrpop_util;

#[macro_use]
extern crate serde_json;

mod cli;
mod dbus_api;
mod parsing;
mod utils;

mod pa_eq;
mod pa_effects;

use cli::*;

use failure::Error;
use structopt::StructOpt;

#[derive(Debug)]
pub struct Filter {
    preamp: f64,
    frequencies: Vec<u32>,
    coefficients: Vec<f64>,
}

impl Filter {
    fn pad(self, filter_rate: u32) -> Self {
        Filter {
            preamp: self.preamp,
            frequencies: vec![0u32]
                .into_iter()
                .chain(self.frequencies.into_iter())
                .chain(vec![filter_rate / 2u32])
                .collect(),
            coefficients: vec![1f64]
                .into_iter()
                .chain(self.coefficients.into_iter())
                .chain(vec![1f64])
                .collect(),
        }
    }
}

fn main() {
    match start() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error: {}", err);
            for cause in err.iter_causes() {
                eprintln!("     : {}", cause);
            }
            std::process::exit(-1);
        }
    }
}

fn start() -> Result<(), Error> {
    let args = Cli::from_args();
    args.log.log_all(args.verbose.log_level())?;

    use Command::*;

    match args.cmd {
        PaEq(args) => pa_eq::main(args),
        PaEffects(args) => pa_effects::main(args),
    }
}
