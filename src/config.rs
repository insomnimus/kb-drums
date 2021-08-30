use crossterm::event::KeyCode;
use indexmap::IndexMap;
use serde_derive::{
	Deserialize,
	Serialize,
};

mod de;

#[derive(Serialize, Deserialize)]
pub struct ControlKeys {
	#[serde(default = "de::default_exit")]
	pub exit: KeyCode,
	#[serde(default = "de::default_next_preset")]
	pub next_preset: KeyCode,
	#[serde(default = "de::default_prev_preset")]
	pub prev_preset: KeyCode,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	#[serde(default = "de::return_true")]
	pub raw_mode: bool,
	#[serde(default = "de::default_volume")]
	pub volume: u8,
	#[serde(default = "de::default_keys")]
	pub keys: IndexMap<char, Drum>,
	#[serde(default)]
	pub control_keys: ControlKeys,
	#[serde(skip)]
	pub device_no: Option<usize>,
	#[serde(default = "de::default_presets")]
	pub presets: Vec<u8>,
}

impl Default for ControlKeys {
	fn default() -> Self {
		Self {
			exit: de::default_exit(),
			next_preset: de::default_next_preset(),
			prev_preset: de::default_prev_preset(),
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
