# kb-drums

Play drums with unnoticeable latency on your keyboard!

# Installation
For pre-built binaries for your platform, visit the [releases page](https://github.com/insomnimus/kb-drums/releases).

To build kb-drums from source, keep on reading.

# Building the Project

First, if you're on a linux system, make sure you have alsa dev libraries installed:
```sh
# debian and derivatives
apt install libasound2-dev

# RHEL and friends
dnf install alsa-lib-devel
```

You have two options:

## Using git

```sh
git clone https://github.com/insomnimus/kb-drums
cd kb-drums
git checkout main
cargo install --path . --locked
```

## Using cargo

`cargo install --locked --git https://github.com/insomnimus/kb-drums --branch main`

# Requirements For Running The App

You only need a MIDI device running as a service/daemon.
Windows devices come with a default MIDI device, but the quality is pretty bad so [Virtual MIDI Synth][] or [OmniMIDI] is recommended.

On *NIX or OSX, [Fluidsynth][] is very nice, although there are a lot of alternatives.

[Fluidsynth][] also works on Apple devices.

# Usage

```output
kb-drums 0.6.1

Play MIDI drums from the command line.

USAGE:
    kb-drums.exe [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -c, --config <config>       Specify a custom config file.
    -d, --device-no <device>    The MIDI device no. Defaults to the first available device.
    -h, --help                  Print help information
        --no-raw                Disable raw mode.
    -v, --volume <volume>       A number between 0 and 127, 127=max.
    -V, --version               Print version information

SUBCOMMANDS:
    default-config    Display the default configuration.
    drums             Show a list of available drum names.
    help              Print this message or the help of the given subcommand(s)
    keys              Show available key names used in the config file.
    list              List available MIDI output devices. [aliases: ls]
```

# Key Mapping

You can specify a config file to kb-drums with the `--config` option.
You can generate a default config file by running `kb-drums default-config`.

In the `keys` map, the keys are any letter keys (a-z)
and the values are either a drum name (run `kb-drums drums` to see them all) or any valid MIDI note numbers (not strings, numbers).

[Virtual MIDI Synth]: https://coolsoft.altervista.org/en/virtualmidisynth
[OmniMIDI]: https://github.com/KeppySoftware/OmniMIDI
[Fluidsynth]: https://www.fluidsynth.org/
