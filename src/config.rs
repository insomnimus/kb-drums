use crossterm::event::KeyCode;
use indexmap::IndexMap;
use serde_derive::Deserialize;

mod de;

#[derive(serde_derive::Serialize, Deserialize)]
pub struct Config {
	#[serde(default = "de::return_true")]
	pub raw_mode: bool,
	#[serde(default = "de::default_volume")]
	pub volume: u8,
	#[serde(default = "de::default_keys")]
	pub keys: IndexMap<char, Drum>,
	#[serde(default = "de::default_exit")]
	pub exit: KeyCode,
	#[serde(skip)]
	pub device_no: Option<usize>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			raw_mode: true,
			keys: de::default_keys(),
			exit: de::default_exit(),
			volume: de::default_volume(),
			device_no: None,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Drum(pub u8);
