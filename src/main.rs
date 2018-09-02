extern crate dbus;

#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

extern crate clap;
extern crate clap_log_flag;
extern crate clap_verbosity_flag;
extern crate structopt;

use structopt::StructOpt;

use failure::Error;

//use dbus::stdintf::org_freedesktop_dbus::Properties;
use dbus::Connection;

use dbus_api::sink::OrgPulseAudioExtEqualizing1Equalizer;

mod dbus_api;
mod parsing;
mod utils;

use utils::*;

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "load")]
    Load(LoadCli),
    #[structopt(name = "reset")]
    Reset(ResetCli),
}

#[derive(StructOpt, Debug)]
struct LoadCli {
}

#[derive(StructOpt, Debug)]
struct ResetCli {
}

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[structopt(flatten)]
    log: clap_log_flag::Log,
    #[structopt(subcommand)]
    cmd: Command,
}

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
        Reset(args)=> reset(args),
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
    let filter = read_filter()?;
    let filter_rate = conn_sink.get_filter_sample_rate()?;
    send_filter(&conn_sink, filter.pad(filter_rate))?;

    Ok(())
}

/*
fn introspect(conn: &dbus::ConnPath<&Connection>) {
    let mut thing = conn
        .method_call_with_args(
            &"org.freedesktop.DBus.Introspectable".into(),
            &"Introspect".into(),
            |_| {},
        ).unwrap();
    thing.as_result().unwrap();

    println!("{}", thing.iter_init().read::<String>().unwrap());
}
*/
