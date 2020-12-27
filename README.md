vsrs
====

[![Latest Version]][crates.io] [![Documentation]][docs.rs]

Command line utility for generating sample streams for the Volca Sample

You configure which samples to use, which indexes to put them in,
and whether samples should be compressed, by using a [Ron file](https://github.com/ron-rs/ron),
with the format documented below in the [Examples](#configuration-file) section.
You can also generate a stream with the factory preset samples by using a .alldata file.
The tool outputs a .wav file which is ready to be used for transferring
the data to the Volca Sample. For details on the transfer process refer to
[transferring syrostream to your volca sample](https://github.com/korginc/volcasample#6-transferring-syrostream-to-your-volca-sample)

# Examples

## Configuration file

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
    }
)
```

## Running the program

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

[docs.rs]: https://docs.rs/vsrs
[crates.io]: https://crates.io/crates/vsrs
[Documentation]: https://docs.rs/vsrs/badge.svg
[Latest Version]: https://img.shields.io/crates/v/vsrs