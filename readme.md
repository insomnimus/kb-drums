# kb-drums

A cli app to play drums with your keyboard.

## Installation

You have two options:

### Using git

```sh
git clone https://github.com/insomnimus/kb-drums
cd kb-drums
git checkout main # or you can choose `rt`
cargo install --path . --locked
```

### Using cargo

`cargo install --locked --git https://github.com/insomnimus/kb-drums --branch main # or rt`

## Usage

You need to have a midi daemon running (windows ships with a midi device but it's pretty crappy, i recommend [Virtual MIDI Synth] instead).

Then just run `kb-drums` and hit some keys.

# Key Mapping

You can specify a config file to kb-drums with the `--config` option.
You can generate a default config file by running `kb-drums default-config`.

In the `keys` map, the keys are any letter keys (a-z)
and the values are either a drum name (run `kb-drums drums` to see them all) or any valid MIDI note numbers (not strings, numbers).

[Virtual MIDI Synth]: https://coolsoft.altervista.org/en/virtualmidisynth
