use std::{
	error::Error,
	process,
};

use crossterm::{
	event::{
		self,
		Event,
		KeyCode,
	},
	terminal::{
		disable_raw_mode,
		enable_raw_mode,
	},
};
use indexmap::IndexMap;
use kb_drums::{
	app,
	config::Drum,
};
use midir::{
	MidiOutput,
	MidiOutputConnection,
};

const NOTE_ON: u8 = 0x99;
const NOTE_OFF: u8 = 0x89;

struct Controller {
	midi: MidiOutputConnection,
	keys: IndexMap<char, Drum>,
	exit: KeyCode,
	volume: u8,
	raw_mode: bool,
}

impl Controller {
	fn from_args() -> Result<Self, Box<dyn Error>> {
		let config = app::parse_config()?;

		let midi_out = MidiOutput::new("kb-drums output")?;
		let out_ports = midi_out.ports();
		if out_ports.is_empty() {
			return Err("no MIDI output device detected".into());
		}

		let out_port = match &config.device_no {
			Some(n) => out_ports.get(*n).ok_or_else(|| {
				format!(
					"specified device no ({}) does not exist; only {} devices detected",
					n,
					out_ports.len()
				)
			})?,
			None => &out_ports[0],
		};

		let out = midi_out.connect(out_port, "kb-drums")?;

		let volume = config.volume;
		let raw_mode = config.raw_mode;
		let exit = config.exit;
		Ok(Self {
			exit,
			midi: out,
			volume,
			keys: config.keys,
			raw_mode,
		})
	}

	fn start(&mut self) {
		if self.raw_mode {
			if let Err(e) = enable_raw_mode() {
				eprintln!("warning: could not enable raw mode: {}", e);
			}
		}

		loop {
			if let Ok(Event::Key(k)) = event::read() {
				if let KeyCode::Char(c) = k.code {
					if let Some(n) = self.keys.get(&c) {
						let _ = self.midi.send(&[NOTE_OFF, n.0, self.volume]);
						let _ = self.midi.send(&[NOTE_ON, n.0, self.volume]);
					}
				} else if k.code == self.exit {
					break;
				}
			}
		}

		if self.raw_mode {
			disable_raw_mode().ok();
		}
	}
}

fn main() {
	let mut controller = match Controller::from_args() {
		Ok(c) => c,
		Err(e) => {
			eprintln!("error: {}", e);
			process::exit(2);
		}
	};

	controller.start();
}
