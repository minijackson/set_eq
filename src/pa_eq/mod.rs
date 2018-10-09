mod dbus_api;

use cli::pa_eq::*;
use utils::*;
use Filter;

use self::dbus_api::equalizing_manager::OrgPulseAudioExtEqualizing1Manager;
use self::dbus_api::server_lookup::OrgPulseAudioServerLookup1;
use self::dbus_api::sink::OrgPulseAudioExtEqualizing1Equalizer;

use dbus::{BusType, ConnPath, Connection};

use failure::{Error, ResultExt};

#[derive(Fail, Debug)]
#[fail(display = "No equalized sink found")]
struct NoEqualizedSink;

pub fn main(cmd: Command) -> Result<(), Error> {
    use cli::pa_eq::Command::*;

    warn!("The PulseAudio equalizer has been deprecated for a while, and is known to sometimes cause crashes, latency or audible artifacts");

    match cmd {
        Load(args) => load(args),
        Reset(args) => reset(args),
    }
}

pub fn reset(_args: ResetCli) -> Result<(), Error> {
    let conn = connect()?;
    let conn_sink = get_equalized_sink(&conn)?;
    let filter_rate = conn_sink.get_filter_sample_rate()?;
    let filter = Filter {
        preamp: 1f64,
        frequencies: vec![],
        coefficients: vec![],
    }
    .pad(filter_rate);

    send_filter(&conn_sink, filter)?;

    Ok(())
}

pub fn load(args: LoadCli) -> Result<(), Error> {
    let conn = connect()?;
    let conn_sink = get_equalized_sink(&conn)?;

    let filter = read_filter_from_arg(&args.file)?;

    let filter_rate = conn_sink.get_filter_sample_rate()?;
    send_filter(&conn_sink, filter.pad(filter_rate))?;

    Ok(())
}

fn connect() -> Result<Connection, Error> {
    Ok(connect_impl().context(
        "Could not connect to PulseAudio's D-Bus socket. Have you loaded the 'module-dbus-protocol' module?"
    )?)
}

fn connect_impl() -> Result<Connection, Error> {
    let pulse_sock_path = get_pulse_dbus_sock()?;
    info!("PulseAudio's D-Bus socket path is: {}", pulse_sock_path);

    trace!("Connecting to PulseAudio's D-Bus socket");
    Ok(Connection::open_private(&pulse_sock_path)?)
}

fn get_equalized_sink(conn: &Connection) -> Result<ConnPath<&Connection>, Error> {
    Ok(get_equalized_sink_impl(conn).context(
        "Could not find an equalized sink. Have you loaded the 'module-equalizer-sink' module?",
    )?)
}

fn get_equalized_sink_impl(conn: &Connection) -> Result<ConnPath<&Connection>, Error> {
    let conn_manager = conn.with_path("org.PulseAudio.Core1", "/org/pulseaudio/equalizing1", 2000);

    // TODO: make that a command-line option
    trace!("Getting (one of) the equalized sink(s)");
    let mut sinks = conn_manager.get_equalized_sinks()?;
    let sink_path = sinks.pop().ok_or(NoEqualizedSink {})?;
    info!("Using equalized sink: {:?}", sink_path.as_cstr());

    trace!("Connecting to equalized sink");
    Ok(conn.with_path("org.PulseAudio.Core1", sink_path, 2000))
}

fn send_filter(conn_sink: &ConnPath<&Connection>, filter: Filter) -> Result<(), Error> {
    let channel = conn_sink.get_nchannels()?;
    info!("Using channel: {}", channel);
    trace!("Sending filter: {:?}", filter);
    conn_sink.seed_filter(
        channel,
        filter.frequencies,
        filter
            .coefficients
            .into_iter()
            .map(decibel_to_ratio)
            .collect(),
        decibel_to_ratio(filter.preamp),
    )?;
    Ok(())
}

fn get_pulse_dbus_sock() -> Result<String, Error> {
    trace!("Connecting to the D-Bus' session bus");
    let conn = Connection::get_private(BusType::Session)?;
    let conn = conn.with_path("org.PulseAudio1", "/org/pulseaudio/server_lookup1", 2000);

    trace!("Checking PulseAudio's D-Bus socket path");
    Ok(conn.get_address()?)
}
