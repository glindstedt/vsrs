//!
//! Command line utility for generating sample streams for the Volca Sample
//!
//! Using a configuration file you configure which samples to use, which indexes to put them in,
//! and whether samples should be compressed. You can also generate a stream with the factory
//! preset samples by using a .alldata file. The tool outputs a .wav file which is ready to
//! be used for transferring the data to the Volca Sample. For details on the transfer process refer to
//! [transferring syrostream to your volca sample](https://github.com/korginc/volcasample#6-transferring-syrostream-to-your-volca-sample)
//!
//! # Running the program
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
//! # Configuration format
//!
//! Supported configuration formats:
//! * [Ron](https://github.com/ron-rs/ron)
//! * JSON
//!
//! ### Ron
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
//!     },
//!     // sets the default part setting for the reverb function
//!     // optional, on or off (off if not specified)
//!     default_part_reverb: on,
//!     // map of sequence patterns, valid keys are 0-9
//!     patterns: {
//!         0: (
//!             // map of pattern parts, valid keys are 0-9
//!             parts: {
//!                 0: (
//!                     // the sample to use for this part, valid values are 0-99
//!                     sample: 0,
//!                     // sequence steps, 1 = on, 0 = off
//!                     steps: [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
//!                     // part toggle options, all optional
//!                     loop: on,
//!                     reverb: off,
//!                     reverse: on,
//!                     motion: on,
//!                     mute: off,
//!                     // part parameters, all optional
//!                     level: 127,          // 0-127 (127)
//!                     pan: 64,             // 1-127 (64=center) (64)
//!                     speed: 64,           // semitone = 40-88 (64=center) (64), continuous = 129-255 (192=center)
//!                     amp_eg_attack: 64,   // 0-127 (0)
//!                     amp_eg_decay: 64,    // 0-127 (127)
//!                     pitch_eg_int: 64,    // 1-127 (64=center) (64)
//!                     pitch_eg_attack: 64, // 0-127 (0)
//!                     pitch_eg_decay: 64,  // 0-127 (127)
//!                     starting_point: 64,  // 0-127 (0)
//!                     length: 64,          // 0-127 (127)
//!                     hi_cut: 64,          // 0-127 (127)
//!                     // motion sequences for the part, optional
//!                     motion_sequences: (
//!                         // valid values: 0-127
//!                         level_start: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         level_end: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         // valid values: 1-127
//!                         pan_start: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         pan_end: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         // valid values: semitone = 40-88, continuous = 129-255
//!                         speed_start: [40, 43, 46, 49, 52, 55, 58, 61, 64, 67, 70, 73, 76, 79, 82, 85],
//!                         speed_end: [129, 137, 145, 153, 161, 169, 177, 185, 193, 201, 209, 217, 225, 233, 241, 249],
//!                         // valid values: 0-127
//!                         amp_eg_attack: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         amp_eg_decay: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         // valid values: 1-127
//!                         pitch_eg_int: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         // valid values: 0-127
//!                         pitch_eg_attack: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         pitch_eg_decay: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         start_point: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         length: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                         hi_cut: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
//!                     ),
//!                 ),
//!             },
//!         ),
//!     },
//! )
//! ```
//!
//! ### JSON
//!
//! See the [Ron](#ron) section for more details about the values
//!
//! ```json
//! {
//!   "default_compression": 16,
//!   "samples": {
//!     "0": {
//!       "Sample": {
//!           "file": "kick.wav",
//!           "compression": 8
//!       }
//!     },
//!     "1": "Erase"
//!   },
//!   "default_part_reverb": "on",
//!   "patterns": {
//!     "0": {
//!       "parts": {
//!         "0": {
//!           "sample": 0,
//!           "steps": [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1 ],
//!           "motion": "off",
//!           "loop": "on",
//!           "reverb": "off",
//!           "reverse": "on",
//!           "mute": "off",
//!           "level: 127,
//!           "pan": 64,
//!           "speed": 64,
//!           "amp_eg_attack": 64,
//!           "amp_eg_decay": 64,
//!           "pitch_eg_int": 64,
//!           "pitch_eg_attack": 64,
//!           "pitch_eg_decay": 64,
//!           "starting_point": 64,
//!           "length": 64,
//!           "hi_cut": 64,
//!           "motion_sequences": {
//!             "level_start": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "level_end": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "pan_start": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "pan_end": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "speed_start": [ 40, 43, 46, 49, 52, 55, 58, 61, 64, 67, 70, 73, 76, 79, 82, 85 ],
//!             "speed_end": [ 129, 137, 145, 153, 161, 169, 177, 185, 193, 201, 209, 217, 225, 233, 241, 249 ],
//!             "amp_eg_attack": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "amp_eg_decay": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "pitch_eg_int": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "pitch_eg_attack": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "pitch_eg_decay": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "start_point": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "length": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
//!             "hi_cut": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ]
//!           }
//!         }
//!       }
//!     }
//!   }
//! }
//! ```
//!
use std::ffi::OsStr;
use std::fs::{read, read_to_string, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{App, Arg, ArgMatches, SubCommand};
use korg_syro::SyroStream;
use log::{debug, info};
use simple_logger::SimpleLogger;
use wav;

mod parse;
use parse::*;

fn get_data(file_name: &str) -> anyhow::Result<VolcaSample> {
    let data_string =
        read_to_string(file_name).with_context(|| format!("Cannot open file '{}'", file_name))?;
    let extension = Path::new(file_name)
        .extension()
        .and_then(OsStr::to_str)
        .expect("No file extension, cannot infer format");
    let data = match extension {
        "ron" => ron::de::from_str::<VolcaSample>(data_string.as_str())
            .with_context(|| format!("Cannot deserialize ron data in file '{}'", file_name))?,
        "json" => serde_json::from_str::<VolcaSample>(data_string.as_str())
            .with_context(|| format!("Cannot deserialize json data in file '{}'", file_name))?,
        _ => return Err(anyhow::anyhow!("Unkonwn file format")),
    };
    info!("Loaded data from file '{}'", file_name);
    Ok(data)
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
    let volca_sample = get_data(input_file)?;

    let mut syro_stream = SyroStream::default();

    debug!("Parsing samples...");
    if let Some(samples) = volca_sample.samples {
        for (index, sample_action) in samples {
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
    }

    debug!("Parsing patterns...");
    if let Some(patterns) = volca_sample.patterns {
        let default_reverb = volca_sample.default_part_reverb.unwrap_or(ToggleDef::Off);
        for (index, pattern_definition) in patterns {
            let pattern = parse_pattern_definition(index, &pattern_definition, default_reverb)?;
            syro_stream.add_pattern(index as usize, pattern)?;
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
        &wav::BitDepth::Sixteen(syro_out),
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
        &wav::BitDepth::Sixteen(syro_out),
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
    let matches = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
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
