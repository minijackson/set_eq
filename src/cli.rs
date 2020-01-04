use structopt;
use structopt::clap::arg_enum;

#[derive(StructOpt, Debug)]
/// A command-line tool to manipulate PulseAudio's equalizers
pub struct Cli {
    #[structopt(long, short, parse(from_occurrences))]
    /// Pass many times for more log output
    ///
    /// By default, it'll only report errors and warnings. Passing `-v` one time
    /// also prints infos, `-vv` enables debug logging, and `-vvv` trace.
    pub verbose: u8,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[cfg(feature = "pa-eq")]
    #[structopt(name = "pa-eq")]
    /// PulseAudio equalizer related commands
    ///
    /// Warning: the PulseAudio equalizer has been deprecated for a while,
    /// and is known to sometimes cause crashes, latency or audible
    /// artifacts
    PaEq(pa_eq::Command),
    #[cfg(feature = "pa-effects")]
    #[structopt(name = "pa-effects")]
    /// PulseEffects equalizer related commands
    PaEffects(pa_effects::Command),
}

arg_enum! {
    #[derive(Debug)]
    pub enum EqualizerConfFormat {
        EqualizerAPO
    }
}

#[cfg(feature = "pa-eq")]
pub mod pa_eq {
    use super::EqualizerConfFormat;

    #[derive(StructOpt, Debug)]
    pub enum Command {
        #[structopt(name = "load")]
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
            long = "format",
            possible_values = &EqualizerConfFormat::variants(),
            case_insensitive = true,
            default_value = "EqualizerAPO"
        )]
        /// The file format of the equalizer configuration
        pub format: EqualizerConfFormat,
        #[structopt(short = "s", long = "sink")]
        /// Use the given sink.
        ///
        /// By default it will use the last equalized sink it finds
        pub sink: Option<String>,
    }

    #[derive(StructOpt, Debug)]
    pub struct ResetCli {
        #[structopt(short = "s", long = "sink")]
        /// Use the given sink.
        ///
        /// By default it will use the last equalized sink it finds
        pub sink: Option<String>,
    }
}

#[cfg(feature = "pa-effects")]
pub mod pa_effects {
    use super::EqualizerConfFormat;

    #[derive(StructOpt, Debug)]
    pub enum Command {
        #[structopt(name = "export-preset")]
        /// Export a PulseEffects preset
        ExportPreset(ExportPresetCli),
    }

    #[derive(StructOpt, Debug)]
    pub struct ExportPresetCli {
        #[structopt(default_value = "-")]
        /// The file from which to load the equalizer configuration
        ///
        /// If "-" is given, read the configuration from the command-line.
        pub file: String,
        #[structopt(
            short = "f",
            long = "format",
            possible_values = &EqualizerConfFormat::variants(),
            case_insensitive = true,
            default_value = "EqualizerAPO"
        )]
        /// The file format of the equalizer configuration
        pub format: EqualizerConfFormat,
        #[structopt(short = "p", long = "base-preset")]
        /// Use a given file as a base for PulseEffects preset instead of the
        /// default one.
        ///
        /// If "-" is given, read the base preset from the command-line.
        pub base_preset: Option<String>,
        #[structopt(short = "o", long = "output")]
        /// Write the preset to the given file instead of the standard output
        pub output: Option<String>,
    }
}
