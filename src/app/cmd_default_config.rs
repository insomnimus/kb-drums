use std::error::Error;

use crate::config::Config;

pub fn run() -> Result<(), Box<dyn Error>> {
	let c = Config::default();
	toml::to_string_pretty(&c)
		.map(|s| {
			println!("{}", s);
		})
		.map_err(|e| e.into())
}
