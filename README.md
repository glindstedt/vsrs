vsrs
====

[![Latest Version]][crates.io] [![Documentation]][docs.rs]

Command line utility for generating sample streams for the Volca Sample

Using a configuration file you configure which samples to use, which indexes to put them in,
and whether samples should be compressed. You can also generate a stream with the factory
preset samples by using a .alldata file.

The tool outputs a `.wav` file which is ready to
be used for transferring the data to the Volca Sample. For details on the transfer process refer to
[transferring syrostream to your volca sample](https://github.com/korginc/volcasample#6-transferring-syrostream-to-your-volca-sample)

# Installing

You need to have [Rust](https://www.rust-lang.org) installed, see
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
for installation instructions.

```shell
cargo install vsrs
```

# Running

Loading a configuration file

```shell
vsrs load example.ron
```

Restoring factory settings using a .alldata file.
Files can be found at
[https://github.com/korginc/volcasample/tree/master/alldata](https://github.com/korginc/volcasample/tree/master/alldata)

```shell
vsrs reset all_sample_preset.alldata
```

## Gotchas

* It's good practice to wipe the sample memory with `all_sample_empty.alldata`
  before loading new samples, otherwise there's a higher risk of getting a
  memory full error.

## Practical Examples

Check out the [contrib](./contrib) directory for configuration file examples.

# Configuration format

Supported configuration formats:
* [Ron](https://github.com/ron-rs/ron)
* JSON

### Ron

```rust
// example.ron
#![enable(implicit_some)]
VolcaSample(
    // optional, valid values are 8-16
    default_compression: 16,
    // map which supports keys in the range 0-99
    samples: {
        // Put kick.wav at sample index 0, and compress it to a bit depth of 8
        0: Sample((
            // the file path is relative to the location of the configuration file
            file: "kick.wav",
            // optional, valid values are 8-16
            compression: 8,
        )),
        // Erase the sample at index 1
        1: Erase,
    },
    // sets the default part setting for the reverb function
    // optional, on or off (off if not specified)
    default_part_reverb: on,
    // map of sequence patterns, valid keys are 0-9
    patterns: {
        0: (
            // map of pattern parts, valid keys are 0-9
            parts: {
                0: (
                    // the sample to use for this part, valid values are 0-99
                    sample: 0,
                    // sequence steps, 1 = on, 0 = off
                    steps: [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
                    // part toggle options, all optional
                    loop: on,
                    reverb: off,
                    reverse: on,
                    motion: on,
                    mute: off,
                    // part parameters, all optional
                    level: 127,          // 0-127 (127)
                    pan: 64,             // 1-127 (64=center) (64)
                    speed: 64,           // semitone = 40-88 (64=center) (64), continuous = 129-255 (192=center)
                    amp_eg_attack: 64,   // 0-127 (0)
                    amp_eg_decay: 64,    // 0-127 (127)
                    pitch_eg_int: 64,    // 1-127 (64=center) (64)
                    pitch_eg_attack: 64, // 0-127 (0)
                    pitch_eg_decay: 64,  // 0-127 (127)
                    starting_point: 64,  // 0-127 (0)
                    length: 64,          // 0-127 (127)
                    hi_cut: 64,          // 0-127 (127)
                    // motion sequences for the part, optional
                    motion_sequences: (
                        // valid values: 0-127
                        level_start: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        level_end: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        // valid values: 1-127
                        pan_start: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        pan_end: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        // valid values: semitone = 40-88, continuous = 129-255
                        speed_start: [40, 43, 46, 49, 52, 55, 58, 61, 64, 67, 70, 73, 76, 79, 82, 85],
                        speed_end: [129, 137, 145, 153, 161, 169, 177, 185, 193, 201, 209, 217, 225, 233, 241, 249],
                        // valid values: 0-127
                        amp_eg_attack: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        amp_eg_decay: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        // valid values: 1-127
                        pitch_eg_int: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        // valid values: 0-127
                        pitch_eg_attack: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        pitch_eg_decay: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        start_point: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        length: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                        hi_cut: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                    ),
                ),
            },
        ),
    },
)
```

### JSON

See the [Ron](#ron) section for more details about the values

```json
{
  "default_compression": 16,
  "samples": {
    "0": {
      "Sample": {
          "file": "kick.wav",
          "compression": 8
      }
    },
    "1": "Erase"
  },
  "default_part_reverb": "on",
  "patterns": {
    "0": {
      "parts": {
        "0": {
          "sample": 0,
          "steps": [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1 ],
          "motion": "off",
          "loop": "on",
          "reverb": "off",
          "reverse": "on",
          "mute": "off",
          "level": 127,
          "pan": 64,
          "speed": 64,
          "amp_eg_attack": 64,
          "amp_eg_decay": 64,
          "pitch_eg_int": 64,
          "pitch_eg_attack": 64,
          "pitch_eg_decay": 64,
          "starting_point": 64,
          "length": 64,
          "hi_cut": 64,
          "motion_sequences": {
            "level_start": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "level_end": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "pan_start": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "pan_end": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "speed_start": [ 40, 43, 46, 49, 52, 55, 58, 61, 64, 67, 70, 73, 76, 79, 82, 85 ],
            "speed_end": [ 129, 137, 145, 153, 161, 169, 177, 185, 193, 201, 209, 217, 225, 233, 241, 249 ],
            "amp_eg_attack": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "amp_eg_decay": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "pitch_eg_int": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "pitch_eg_attack": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "pitch_eg_decay": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "start_point": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "length": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
            "hi_cut": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ]
          }
        }
      }
    }
  }
}
```

[docs.rs]: https://docs.rs/vsrs
[crates.io]: https://crates.io/crates/vsrs
[Documentation]: https://docs.rs/vsrs/badge.svg
[Latest Version]: https://img.shields.io/crates/v/vsrs