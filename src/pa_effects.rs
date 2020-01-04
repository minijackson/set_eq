use crate::cli::pa_effects::*;
use crate::utils::*;
use crate::Filter;

use failure::{Error, ResultExt};

use serde_json;

use std::io::{self, Write};

const DEFAULT_PRESET: &str = include_str!("../res/default-pa-effects-preset.json");

pub fn main(cmd: Command) -> Result<(), Error> {
    use crate::cli::pa_effects::Command::*;

    match cmd {
        ExportPreset(args) => export_preset(args),
    }
}

fn export_preset(args: ExportPresetCli) -> Result<(), Error> {
    debug!("Parsing base preset");
    let mut preset: serde_json::Value = match args.base_preset {
        Some(file) => {
            info!("Reading base PulseEffects preset from file '{}'", file);
            serde_json::from_str(&read_filearg_to_str(&file)?)
        }
        None => {
            info!("Using default PulseEffects preset");
            serde_json::from_str(&DEFAULT_PRESET)
        }
    }?;

    let filter = read_filter_from_arg(&args.file)?;

    preset["output"]["equalizer"] = filter_to_eq_preset(filter);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "{}", preset).context("Could not output the PulseEffects preset")?;
    Ok(())
}

fn filter_to_eq_preset(mut filter: Filter) -> serde_json::Value {
    if filter.frequencies.len() > 30 {
        info!("More than 30 frequencies specified, taking the approximative approach");
        filter = simplify_filter(&filter);
    }

    let mut equalizer: serde_json::Value = json!({
        "state": "true",
        "num-bands": filter.frequencies.len(),
        "input-gain": 0,
        "output-gain": 0,
        "split-channels": false,
        "left": {},
        "right": {},
    });

    for (i, (frequency, coeff)) in filter
        .frequencies
        .into_iter()
        .zip(filter.coefficients)
        .enumerate()
    {
        let peak = json!({
            "gain": coeff,
            "frequency": frequency,
            "type": "Bell",
            "mode": "RLC (BT)",
            "slope": "x1",
            "solo": "false",
            "mute": "false",
        });
        equalizer["left"][format!("band{}", i)] = peak.clone();
        equalizer["right"][format!("band{}", i)] = peak;
    }

    equalizer
}

fn simplify_filter(filter: &Filter) -> Filter {
    //let partition_size = (filter.frequencies.len() as f64 / 30f64).floor() as usize;
    let mut partition_size = filter.frequencies.len() / 30;
    let step_error = filter.frequencies.len() as f64 % 30f64;
    if step_error != 0f64 {
        trace!("The approximation will be imperfect");
        partition_size += 1;
    }

    //let mut cumulative_error = 0f64;

    let frequencies = filter.frequencies.chunks(partition_size).map(|vec| {
        let sum: u32 = vec.iter().sum();
        sum / vec.len() as u32
    }).collect();

    let coefficients = filter.coefficients.chunks(partition_size).map(|vec| {
        let sum: f64 = vec.iter().sum();
        sum / vec.len() as f64
    }).collect();

    Filter {
        preamp: filter.preamp,
        frequencies, coefficients
    }
}

/*
trait MultiPartitionnable {
    type Item;

    fn multi_partition(self, size: usize, wanted_parts: usize) -> MultiPartition<Self>
    where
        Self: Iterator + Sized,
    {
        MultiPartition {
            iter: self,
            size,
            wanted_parts,
            cumulative_error: 0f64,
        }
    }
}

impl<I> MultiPartitionnable for Iterator<Item = I> {
    type Item = I;
}

struct MultiPartition<I: Iterator> {
    iter: I,
    size: usize,
    wanted_parts: usize,
    cumulative_error: f64,
}

impl<I: Iterator> Iterator for MultiPartition<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {

    }
}
*/
