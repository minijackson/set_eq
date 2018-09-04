use clap_verbosity_flag;
use clap_log_flag;
use structopt;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// Hello World! How are you doing?
pub struct Cli {
    #[structopt(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
    #[structopt(flatten)]
    pub log: clap_log_flag::Log,
    #[structopt(short = "s")]
    /// Use the given sink.
    ///
    /// By default it will use the last equalized sink it finds
    pub sink: Option<String>,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "load",)]
    /// Load and switch to a given equalizer configuration
    Load(LoadCli),
    #[structopt(name = "reset")]
    /// Switch to a neutral equalizer
    Reset(ResetCli),
}

#[derive(StructOpt, Debug)]
pub struct LoadCli {
    #[structopt(default_value = "-")]
    /// The file from which to load the equalizer configuration
    ///
    /// If "-" is given, read the configuration from the command-line.
    pub file: String,
    #[structopt(
        short = "f",
        raw(
            possible_values = "&EqualizerConfFormat::variants()",
            case_insensitive = "true"
        ),
        default_value = "EqualizerAPO"
    )]
    /// The file format of the equalizer configuration
    pub format: EqualizerConfFormat,
}

arg_enum! {
    #[derive(Debug)]
    pub enum EqualizerConfFormat {
        EqualizerAPO
    }
}

#[derive(StructOpt, Debug)]
pub struct ResetCli {}
