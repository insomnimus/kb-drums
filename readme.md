# kb-drums

A cli app to play drums with your keyboard.

## Installation

You have two options:

### Install via git clone

```sh
git clone https://github.com/insomnimus/kb-drums
cd kb-drums
git checkout main # or you can choose `rt`
cargo install --path .
```

### Install directly with cargo

`cargo install --git https://github.com/insomnimus/kb-drums --branch main # or rt`

## Usage

You need to have a midi daemon running (windows ships with a midi device but it's pretty crappy, i recommend virtual midi synth instead).

Then just run `kb-drums` and hit some keys.
