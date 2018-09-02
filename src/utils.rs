use ::{Filter, NoEqualizedSink};

use parsing::EqualizerApoParser;

use ::dbus_api::equalizing_manager::OrgPulseAudioExtEqualizing1Manager;
use ::dbus_api::server_lookup::OrgPulseAudioServerLookup1;
use ::dbus_api::sink::OrgPulseAudioExtEqualizing1Equalizer;

use dbus::{BusType, Connection, ConnPath};
use failure::{Error, ResultExt};

use std::io::{self, Read};

pub fn connect() -> Result<Connection, Error> {
    let pulse_sock_path =
        get_pulse_dbus_sock().context("While looking up PulseAudio's D-Bus socket path")?;
    info!("PulseAudio's D-Bus socket path is: {}", pulse_sock_path);

    trace!("Connecting to PulseAudio's D-Bus socket");
    Ok(Connection::open_private(&pulse_sock_path)?)
}

pub fn get_equalized_sink<'a>(conn: &'a Connection) -> Result<ConnPath<'a, &'a Connection>, Error> {
    let conn_manager = conn.with_path("org.PulseAudio.Core1", "/org/pulseaudio/equalizing1", 2000);

    // TODO: make that a command-line option
    trace!("Getting (one of) the equalized sink(s)");
    let mut sinks = conn_manager.get_equalized_sinks()?;
    let sink_path = sinks.pop().ok_or(NoEqualizedSink {})?;
    info!("Using equalized sink: {:?}", sink_path.as_cstr());

    trace!("Connecting to equalized sink");
    Ok(conn.with_path("org.PulseAudio.Core1", sink_path, 2000))
}

pub fn send_filter(conn_sink: &ConnPath<&Connection>, filter: Filter) -> Result<(), Error> {
    let channel = conn_sink.get_nchannels()?;
    info!("Using channel: {}", channel);
    trace!("Sending filter");
    conn_sink.seed_filter(
        channel,
        filter.frequencies,
        filter.coefficients,
        filter.preamp,
    )?;
    Ok(())
}

pub fn read_filter() -> Result<Filter, Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    info!("Reading filter in GraphicEQ format from the command line");
    handle.read_to_string(&mut buffer)?;

    // TODO: lifetime issue when "throwing" parse error
    let filter = EqualizerApoParser::new().parse(&buffer).unwrap();
    info!("Parsed filter: {:?}", filter);

    Ok(filter)
}

fn get_pulse_dbus_sock() -> Result<String, Error> {
    trace!("Connecting to the D-Bus' session bus");
    let conn = Connection::get_private(BusType::Session)?;
    let conn = conn.with_path("org.PulseAudio1", "/org/pulseaudio/server_lookup1", 2000);

    trace!("Checking PulseAudio's D-Bus socket path");
    Ok(conn.get_address()?)
}
