//!
//! Command line utility for generating sample streams for the Volca Sample
//!
//! You configure which samples to use, which indexes to put them in,
//! and whether samples should be compressed, by using a [Ron file](https://github.com/ron-rs/ron),
//! with the format documented below in the [Examples](#configuration-file) section.
//! You can also generate a stream with the factory preset samples by using a .alldata file.
//! The tool outputs a .wav file which is ready to be used for transferring
//! the data to the Volca Sample. For details on the transfer process refer to
//! [transferring syrostream to your volca sample](https://github.com/korginc/volcasample#6-transferring-syrostream-to-your-volca-sample)
//!
//! # Examples
//!
//! ## Configuration file
//!
//! ```rust
//! // example.ron
//! #![enable(implicit_some)]
//! VolcaSample(
//!     // optional, valid values are 8-16
//!     default_compression: 16,
//!     // map which supports keys in the range 0-99
//!     samples: {
//!         // Put kick.wav at sample index 0, and compress it to a bit depth of 8
//!         0: Sample((
//!             // the file path is relative to the location of the configuration file
//!             file: "kick.wav",
//!             // optional, valid values are 8-16
//!             compression: 8,
//!         )),
//!         // Erase the sample at index 1
//!         1: Erase,
//!     }
//! )
//! ```
//!
//! ## Running the program
//!
//! Loading a configuration file
//!
//! ```shell
//! vsrs load example.ron
//! ```
//!
//! Restoring factory settings using a .alldata file.
//! Files can be found at
//! [https://github.com/korginc/volcasample/tree/master/alldata](https://github.com/korginc/volcasample/tree/master/alldata)
//!
//! ```shell
//! vsrs reset all_sample_preset.alldata
//! ```
//!
use std::collections::HashMap;
use std::fs::{read, read_to_string, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{App, Arg, ArgMatches, SubCommand};
use korg_syro::SyroStream;
use log::{debug, info};
use ron::de::from_str;
use serde::Deserialize;
use simple_logger::SimpleLogger;
use wav;

#[derive(Debug, Deserialize)]
struct Sample {
    file: String,
    compression: Option<u32>,
}

#[derive(Debug, Deserialize)]
enum SampleAction {
    Sample(Sample),
    Erase,
}

#[derive(Debug, Deserialize)]
struct VolcaSample {
    // Default compression to apply for all
    default_compression: Option<u32>,
    samples: HashMap<u32, SampleAction>,
}

fn get_ron_data(file_name: &str) -> anyhow::Result<VolcaSample> {
    let ron_string = read_to_string(file_name)
        .with_context(|| format!("Cannot open ron file '{}'", file_name))?;
    let ron_data = from_str::<VolcaSample>(ron_string.as_str())
        .with_context(|| format!("Cannot deserialize ron data in file '{}'", file_name))?;
    info!("Loaded data from file '{}'", file_name);
    Ok(ron_data)
}

fn read_sample(file_path: &Path) -> anyhow::Result<(wav::Header, Vec<i16>)> {
    let mut file = File::open(&file_path).with_context(|| "Cannot open input file")?;
    let (header, bit_depth) = wav::read(&mut file).with_context(|| "Cannot read input file")?;

    //TODO conversions
    let data = match bit_depth {
        wav::BitDepth::Eight(_) => Err(anyhow::anyhow!("8 bit stream not supported yet")),
        wav::BitDepth::Sixteen(d) => Ok(d),
        wav::BitDepth::TwentyFour(_) => Err(anyhow::anyhow!("24 bit stream not supported yet")),
        wav::BitDepth::Empty => Err(anyhow::anyhow!("empty?? bit stream not supported yet")),
    }?;
    Ok((header, data))
}

fn get_output_file(arg_matches: &ArgMatches, input_file: &str) -> String {
    let mut dir = std::env::current_dir().unwrap_or(PathBuf::from("."));
    arg_matches
        .value_of("output")
        .map(|s| String::from(s))
        .or_else(|| {
            Path::new(input_file)
                .file_stem()
                .and_then(|os_s| os_s.to_str())
                .and_then(|s| {
                    dir.push(format!("{}.wav", s));
                    dir.to_str().map(|s| String::from(s))
                })
        })
        .unwrap()
}

fn load(input_file: &str, output_file: &str) -> anyhow::Result<()> {
    let input_dir = Path::new(input_file).parent().unwrap_or(Path::new("."));
    let volca_sample = get_ron_data(input_file)?;

    let mut syro_stream = SyroStream::default();

    for (index, sample_action) in volca_sample.samples {
        match sample_action {
            SampleAction::Sample(sample) => {
                let file_path = input_dir.join(sample.file).into_boxed_path();
                let (header, data) = read_sample(&file_path)?;
                let compression = sample.compression.or(volca_sample.default_compression);

                debug!(
                    "Sample {} '{}', duration = {}s, compression = {:?}, wav: {:?}",
                    index,
                    file_path.to_string_lossy(),
                    data.len() as f32 / header.sampling_rate as f32,
                    compression,
                    header
                );
                syro_stream.add_sample(index, data, header.sampling_rate, compression)?;
            }
            SampleAction::Erase => {
                debug!("Erase {}", index);
                syro_stream.erase_sample(index)?;
            }
        }
    }

    let syro_out = syro_stream.generate()?;

    let header = wav::Header::new(1, 2, 44100, 16);
    debug!(
        "Output '{}', duration = {}s, wav: {:?}",
        output_file,
        syro_out.len() as f32 / header.sampling_rate as f32,
        header
    );

    let output = File::create(&output_file)?;

    wav::write(
        header,
        wav::BitDepth::Sixteen(syro_out),
        &mut BufWriter::new(output),
    )?;
    info!("Wrote output to file '{}'", output_file);

    Ok(())
}

fn reset(input_file: &str, output_file: &str, compression: Option<u32>) -> anyhow::Result<()> {
    let data = read(input_file)?;
    let syro_out = SyroStream::reset(data, compression)?;
    let header = wav::Header::new(1, 2, 44100, 16);
    debug!(
        "Output '{}', duration = {}s, wav: {:?}",
        output_file,
        syro_out.len() as f32 / header.sampling_rate as f32,
        header
    );

    let output = File::create(&output_file)?;

    wav::write(
        header,
        wav::BitDepth::Sixteen(syro_out),
        &mut BufWriter::new(output),
    )?;
    info!("Wrote output to file '{}'", output_file);

    Ok(())
}

fn compress_validator(v: String) -> Result<(), String> {
    let value = v
        .parse::<u32>()
        .map_err(|_| String::from("Cannot parse into u32"))?;
    if value < 8 || value > 16 {
        Err(String::from("Invalid value, must be between 8 and 16"))
    } else {
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let output_arg = Arg::with_name("output")
        .short("o")
        .long("out_file")
        .takes_value(true);
    let matches = App::new("vsrs")
        .version("0.1.0")
        .subcommand(
            SubCommand::with_name("load")
                .about("Load sample configuration file")
                .arg(Arg::with_name("input").required(true).index(1))
                .arg(&output_arg),
        )
        .subcommand(
            SubCommand::with_name("reset")
                .about("Reset to factory settings using a .alldata file")
                .arg(Arg::with_name("input").required(true).index(1))
                .arg(&output_arg)
                .arg(
                    Arg::with_name("compress")
                        .short("c")
                        .long("compress")
                        .validator(compress_validator)
                        .takes_value(true)
                        .help("compression of .alldata file"),
                ),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("verbosity level"),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("verbose") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        4 | _ => log::LevelFilter::Trace,
    };
    SimpleLogger::new().with_level(log_level).init().unwrap();

    if let Some(matches) = matches.subcommand_matches("load") {
        let input_file = matches.value_of("input").unwrap();
        let output_file = get_output_file(matches, input_file);
        info!("Loading from '{}'", input_file);
        load(input_file, output_file.as_str())?;
    }

    if let Some(matches) = matches.subcommand_matches("reset") {
        let input_file = matches.value_of("input").unwrap();
        let output_file = get_output_file(matches, input_file);
        let compress = match matches.value_of("compress") {
            Some(s) => Some(s.parse::<u32>()?),
            None => None,
        };
        info!("Resetting from '{}'", input_file);
        reset(input_file, output_file.as_str(), compress)?;
    }

    Ok(())
}
