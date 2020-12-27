example sample packs
====================

Some examples of configurations for free sample packs found online. Download
the packs and extract the samples into the respective directory, then run
`vsrs` using the configuration file to generate the transfer stream.

You can use the [genron.sh](./genron.sh) script to generate a starting point
for a sample pack configuration if you have a directory filled with samples.
Remember that you can only have 100 samples on the Volca Sample. Example:

```bash
ls dir_with_samples | genron.sh > dir_with_samples/samples.ron
```

* [analog](./analog/analog.ron) extract the samples from this pack into the directory, note that they have not been ordered in the example:
    [https://volcasamplepack.wordpress.com/2018/10/06/analog-drum-sounds-and-synths-on-volca-sample/](https://volcasamplepack.wordpress.com/2018/10/06/analog-drum-sounds-and-synths-on-volca-sample/)
* [funklet](./funklet) Sounds and patterns from the excellent [https://funklet.com](https://funklet.com)
* [srvchill](./srvchill/srvchill.ron) extract the samples from this pack into the directory, note that they have not been ordered in the example:
    [https://volcasamplepack.wordpress.com/2018/10/06/srv-chill-ambient-downtempo-beats-on-volca-sample/](https://volcasamplepack.wordpress.com/2018/10/06/srv-chill-ambient-downtempo-beats-on-volca-sample/)
* [moa8itwedding](./moa8itwedding/moa8itwedding.ron) extract the samples from the `LQ_16Bit_41-Khz` directory in the WAV pack:
    [https://moa8itwedding.com/free-volca-sample-pack/](https://moa8itwedding.com/free-volca-sample-pack/)
* [test](./test/test.ron) used for test, `kick.wav` can be found [here](https://github.com/korginc/volcasample/blob/master/example/execute_gnulinux/02%20Kick%203.wav)