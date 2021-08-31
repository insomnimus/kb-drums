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
	config::{
		Config,
		ControlKeys,
	},
};
use midir::{
	MidiOutput,
	MidiOutputConnection,
};

const PROGRAM_CHANGE: u8 = 0xC9;
const NOTE_ON: u8 = 0x99;

const NOTE_OFF: u8 = 0x89;

struct Controller {
	midi: MidiOutputConnection,
	keys: IndexMap<KeyCode, u8>,
	control: ControlKeys,
	volume: u8,
	raw_mode: bool,
	cursor: i32,
	presets: Vec<u8>,
}

impl Controller {
	fn from_args() -> Result<Self, Box<dyn Error>> {
		let Config {
			raw_mode,
			volume,
			keys,
			control_keys,
			device_no,
			presets,
		} = app::parse_config()?;

		let midi_out = MidiOutput::new("kb-drums output")?;

		let out_ports = midi_out.ports();

		if out_ports.is_empty() {
			return Err("no MIDI output device detected".into());
		}

		let out_port = match device_no {
			Some(n) => out_ports.get(n).ok_or_else(|| {
				format!(
					"specified device no ({}) does not exist; only {} devices detected",
					n,
					out_ports.len()
				)
			})?,
			None => &out_ports[0],
		};

		let out = midi_out.connect(out_port, "kb-drums")?;

		let keys: IndexMap<_, _> = keys
			.into_iter()
			.map(|(c, d)| (KeyCode::Char(c), d.0))
			.collect();

		Ok(Self {
			control: control_keys,
			midi: out,
			volume,
			keys,
			raw_mode,
			cursor: 0,
			presets,
		})
	}

	fn start(&mut self) {
		self.print_banner();
		// set the MIDI volume to max.
		let _ = self.midi.send(&[0xB9, 0x07, 127]);

		if self.raw_mode {
			if let Err(e) = enable_raw_mode() {
				eprintln!("warning: could not enable raw mode: {}", e);
			}
		}

		// Load the first preset.
		if let Some(n) = self.presets.get(0) {
			if let Err(e) = self.midi.send(&[PROGRAM_CHANGE, *n]) {
				println!("warning: couldn't load the first preset({}): {}", n, e);
			}
		}

		loop {
			let k = match event::read() {
				Ok(Event::Key(k)) => k,
				_ => continue,
			};

			if let Some(&n) = self.keys.get(&k.code) {
				let _ = self.midi.send(&[NOTE_OFF, n, self.volume]);

				let _ = self.midi.send(&[NOTE_ON, n, self.volume]);
			} else if k.code == self.control.exit {
				break;
			} else if k.code == self.control.next_preset {
				self.next_preset();
			} else if k.code == self.control.prev_preset {
				self.prev_preset();
			} else if k.code == self.control.volume_up {
				self.change_volume(12);
			} else if k.code == self.control.volume_down {
				self.change_volume(-12);
			}
		}

		if self.raw_mode {
			disable_raw_mode().ok();
		}
	}

	fn next_preset(&mut self) {
		if self.presets.is_empty() {
			println!("No presets detected.");

			return;
		}

		self.cursor = (self.cursor + 1) % self.presets.len() as i32;

		let n = self.presets[self.cursor as usize];

		match self.midi.send(&[PROGRAM_CHANGE, n]) {
			Ok(_) => println!("Changed the preset to {}", n),
			Err(e) => println!("Error changing the preset to {}: {}", n, e),
		};
	}

	fn prev_preset(&mut self) {
		if self.presets.is_empty() {
			println!("No preset detected.");

			return;
		}

		self.cursor -= 1;

		while self.cursor < 0 {
			self.cursor += self.presets.len() as i32;
		}

		let n = self.presets[self.cursor as usize];

		match self.midi.send(&[PROGRAM_CHANGE, n]) {
			Ok(_) => println!("Changed the preset to {}", n),
			Err(e) => println!("Error changing the preset to {}: {}", n, e),
		};
	}

	fn change_volume(&mut self, amount: i16) {
		let x = self.volume as i16 + amount;

		self.volume = if amount < 0 {
			if x < 0 {
				0
			} else {
				x as u8
			}
		} else if x > 127 {
			127
		} else {
			x as u8
		};

		println!("volume = {}%", (self.volume as usize) * 100 / 127);
	}

	fn print_banner(&self) {
		println!("Loaded {} presets.", self.presets.len());
		println!(
			"Volume up: {:?}\nVolume down: {:?}",
			self.control.volume_up, self.control.volume_down
		);
		println!(
			"Next preset: {:?}\nPrev preset: {:?}",
			self.control.next_preset, self.control.prev_preset
		);
		println!("Press {:?} to quit.", self.control.exit);
		println!("Ready, set, jam!");
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
