use crate::Filter;

use crate::cli::EqualizerConfFormat;
use crate::parsing::EqualizerApoParser;

use failure::Error;
use lalrpop_util;

use std::fmt;
use std::fs::File;
use std::io;

#[derive(Fail, Debug)]
#[fail(
    display = "Could not parse using the {} format: {}",
    format,
    message
)]
struct ParseError {
    format: EqualizerConfFormat,
    message: String,
}

pub fn read_filearg_to_str(file: &str) -> Result<String, Error> {
    use std::io::Read;

    let mut buffer = String::new();
    if file == "-" {
        debug!("Reading file from the command line");
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer)?;
    } else {
        let mut file = File::open(file)?;
        file.read_to_string(&mut buffer)?;
    }
    Ok(buffer)
}

pub fn read_filter_from_arg(file: &str) -> Result<Filter, Error> {
    info!("Reading filter from '{}' in the EqualizerAPO format", file);
    let content = read_filearg_to_str(file)?;
    parse_filter(&content)
}

pub fn parse_filter(content: &str) -> Result<Filter, Error> {
    // TODO: lifetime issue when "throwing" parse error
    let filter = EqualizerApoParser::new()
        .parse(&content)
        .map_err(|e| convert_parse_error(EqualizerConfFormat::EqualizerAPO, &e))?;
    trace!("Parsed filter: {:?}", filter);

    Ok(filter)
}

fn convert_parse_error<L, T, E>(
    format: EqualizerConfFormat,
    error: &lalrpop_util::ParseError<L, T, E>,
) -> ParseError
where
    L: fmt::Display,
    T: fmt::Display,
    E: fmt::Display,
{
    ParseError {
        format,
        message: format!("{}", error),
    }
}
