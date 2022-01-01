use std::{
	error::Error,
	fs,
	path::PathBuf,
	process,
};

use clap::{
	arg,
	crate_version,
	App,
};

use crate::config::Config;

mod cmd_default_config;
mod cmd_devices;
mod cmd_drums;
mod cmd_keys;

struct Args {
	raw_mode: bool,
	config_path: Option<PathBuf>,
	device_no: Option<usize>,
	volume: Option<u8>,
}

impl Args {
	fn app() -> App<'static> {
		let app = App::new("kb-drums")
			.about("Play MIDI drums from the command line.")
			.version(crate_version!())
			.args(&[
				arg!(-d --device  [NO] "Specify the MIDI output device.").validator(|s| {
					s.parse::<usize>()
						.map(|_| {})
						.map_err(|_| "the value must be a non-negative number")
				}),
				arg!(-c --config [PATH] "Specify a config file."),
				arg!(--noraw "Do not enable raw mode."),
				arg!(-v --volume [VOLUME] "The volume. A number between 0 and 127.").validator(
					|s| {
						s.parse::<u8>()
							.ok()
							.filter(|&n| n < 128)
							.map(|_| {})
							.ok_or_else(|| {
								String::from("the value must be a number between 0 and 127")
							})
					},
				),
			]);

		let app_keys = App::new("keys").about("Show available key names used in the config file.");

		let app_list = App::new("list")
			.visible_alias("ls")
			.about("List available MIDI output devices.");

		let app_drums = App::new("drums").about("Show a list of available drum names.");

		let app_default_config =
			App::new("default-config").about("Display the default configuration.");

		app.subcommand(app_keys)
			.subcommand(app_drums)
			.subcommand(app_list)
			.subcommand(app_default_config)
	}

	fn from_args() -> Self {
		let m = Self::app().get_matches();

		match m.subcommand_name() {
			None => (),
			Some(cmd) => {
				if let Err(e) = match cmd {
					"keys" => {
						cmd_keys::run();
						Ok(())
					}
					"drums" => {
						cmd_drums::run();

						Ok(())
					}
					"list" => cmd_devices::run(),
					"default-config" => cmd_default_config::run(),
					_ => panic!("unhandled subcommand match case: {:?}", cmd),
				} {
					eprintln!("error: {}", e);

					process::exit(2);
				}

				process::exit(0);
			}
		};

		let raw_mode = !m.is_present("no-raw");

		let config_path = m.value_of("config").map(PathBuf::from);

		let volume = m.value_of("volume").map(|s| s.parse::<u8>().unwrap());

		let device_no = m.value_of("device").map(|s| s.parse::<usize>().unwrap());

		Self {
			raw_mode,
			config_path,
			device_no,
			volume,
		}
	}
}

pub fn parse_config() -> Result<Config, Box<dyn Error>> {
	let Args {
		raw_mode,
		volume,
		device_no,
		config_path,
	} = Args::from_args();

	let mut config = match config_path {
		Some(p) => {
			let data = fs::read_to_string(&p)?;

			serde_json::from_str(&data)?
		}
		None => Config::default(),
	};

	if !raw_mode {
		config.raw_mode = false;
	}

	config.device_no = device_no;

	if let Some(v) = volume {
		config.volume = v;
	}

	if config.volume > 127 {
		Err("the value for volume can't be above 127".into())
	} else {
		Ok(config)
	}
}
