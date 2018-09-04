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

mod cli;
mod dbus_api;
mod parsing;
mod utils;

use utils::*;
use dbus_api::sink::OrgPulseAudioExtEqualizing1Equalizer;
use cli::*;

use failure::Error;
use structopt::StructOpt;

use std::fs::File;
use std::io;

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
        Load(args) => load(args),
        Reset(args) => reset(args),
    }
}

fn reset(args: ResetCli) -> Result<(), Error> {
    let conn = connect()?;
    let conn_sink = get_equalized_sink(&conn)?;
    let filter_rate = conn_sink.get_filter_sample_rate()?;
    let filter = Filter {
        preamp: 1f64,
        frequencies: vec![],
        coefficients: vec![],
    }.pad(filter_rate);

    send_filter(&conn_sink, filter)?;

    Ok(())
}

fn load(args: LoadCli) -> Result<(), Error> {
    let conn = connect()?;
    let conn_sink = get_equalized_sink(&conn)?;

    let filter = if args.file == "-" {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        read_filter(&mut handle)?
    } else {
        let mut file = File::open(args.file)?;
        read_filter(&mut file)?
    };

    let filter_rate = conn_sink.get_filter_sample_rate()?;
    send_filter(&conn_sink, filter.pad(filter_rate))?;

    Ok(())
}
