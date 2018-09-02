extern crate dbus;

#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate clap;
extern crate clap_log_flag;
extern crate clap_verbosity_flag;
extern crate structopt;

mod dbus_api;
mod parsing;
mod utils;

use utils::*;

use dbus_api::sink::OrgPulseAudioExtEqualizing1Equalizer;
use failure::Error;
use structopt::StructOpt;

use std::fs::File;
use std::io;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// Hello World! How are you doing?
struct Cli {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[structopt(flatten)]
    log: clap_log_flag::Log,
    #[structopt(short = "s")]
    /// Use the given sink.
    ///
    /// By default it will use the last equalized sink it finds
    sink: Option<String>,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "load",)]
    /// Load and switch to a given equalizer configuration
    Load(LoadCli),
    #[structopt(name = "reset")]
    /// Switch to a neutral equalizer
    Reset(ResetCli),
}

#[derive(StructOpt, Debug)]
struct LoadCli {
    #[structopt(default_value = "-")]
    /// The file from which to load the equalizer configuration
    ///
    /// If "-" is given, read the configuration from the command-line.
    file: String,
    #[structopt(
        short = "f",
        raw(
            possible_values = "&EqualizerConfFormat::variants()",
            case_insensitive = "true"
        ),
        default_value = "EqualizerAPO"
    )]
    /// The file format of the equalizer configuration
    format: EqualizerConfFormat,
}

arg_enum! {
    #[derive(Debug)]
    enum EqualizerConfFormat {
        EqualizerAPO
    }
}

#[derive(StructOpt, Debug)]
struct ResetCli {}

#[derive(Fail, Debug)]
#[fail(display = "No equalized sink found")]
struct NoEqualizedSink;

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

fn main() -> Result<(), Error> {
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
        let mut lock = stdin.lock();
        read_filter(&mut lock)?
    } else {
        let mut file = File::open(args.file)?;
        read_filter(&mut file)?
    };

    let filter_rate = conn_sink.get_filter_sample_rate()?;
    send_filter(&conn_sink, filter.pad(filter_rate))?;

    Ok(())
}
