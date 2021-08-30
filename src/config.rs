use crossterm::event::KeyCode;
use indexmap::IndexMap;
use serde_derive::{
	Deserialize,
	Serialize,
};

mod de;

#[derive(Serialize, Deserialize)]
#[serde(default)]

pub struct ControlKeys {
	pub exit: KeyCode,
	pub next_preset: KeyCode,
	pub prev_preset: KeyCode,
	pub volume_up: KeyCode,
	pub volume_down: KeyCode,
}

#[derive(Serialize, Deserialize)]

pub struct Config {
	#[serde(default = "de::return_true")]
	pub raw_mode: bool,
	#[serde(default = "de::default_volume")]
	pub volume: u8,
	#[serde(default)]
	pub control_keys: ControlKeys,
	#[serde(default = "de::default_keys")]
	pub keys: IndexMap<char, Drum>,
	#[serde(default = "de::default_presets")]
	pub presets: Vec<u8>,

	#[serde(skip)]
	pub device_no: Option<usize>,
}

impl Default for ControlKeys {
	fn default() -> Self {

		Self {
			exit: KeyCode::Esc,
			next_preset: KeyCode::Right,
			prev_preset: KeyCode::Left,
			volume_up: KeyCode::Up,
			volume_down: KeyCode::Down,
		}
	}
}

impl Default for Config {
	fn default() -> Self {

		Self {
			raw_mode: true,
			keys: de::default_keys(),
			control_keys: ControlKeys::default(),
			volume: de::default_volume(),
			device_no: None,
			presets: de::default_presets(),
		}
	}
}

#[derive(Debug, Clone, Copy)]

pub struct Drum(pub u8);
