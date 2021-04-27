use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use kb_drums::keys::map_key;
use midir::{MidiOutput, MidiOutputConnection};
use std::env;
use std::process::exit;

const NOTE_ON: u8 = 0x99;
const NOTE_OFF: u8 = 0x89;
const VELOCITY: u8 = 0x7f; // max velocit

struct Drum {
    con: MidiOutputConnection,
}

impl Drum {
    fn start(&mut self) {
        loop {
            let event = match read() {
                Ok(e) => e,
                _ => continue,
            };
            match event {
                Event::Key(k) => match k.code {
                    KeyCode::Char(c) => {
                        if let Some(n) = map_key(c) {
                            self.play(n);
                        }
                    }
                    KeyCode::Esc => return,
                    _ => (),
                },
                _ => (),
            };
        }
    }

    fn play(&mut self, note: u8) {
        let _ = self.con.send(&[NOTE_OFF, note, VELOCITY]);
        let _ = self.con.send(&[NOTE_ON, note, VELOCITY]);
    }
}

fn list_ports() {
    let midi_out = MidiOutput::new("kb-drums output").unwrap_or_else(|e| {
        eprintln!("error: {:?}", e);
        exit(1);
    });

    let out_ports = midi_out.ports();
    if out_ports.is_empty() {
        println!("no output ports detected");
        exit(0);
    }
    for (i, p) in out_ports.iter().enumerate() {
        println!("{}: {}", i, midi_out.port_name(p).unwrap());
    }
    exit(0);
}

fn show_help() {
    println!(
        "kb-drums, play midi drums with your keyboad
usage:
	kb-drums [options] [port_no]
options are:
	-l, --list: list available midi ports
	-h, --help: show this message"
    );
    exit(0);
}

fn main() {
    let port_no: usize = match env::args().nth(1) {
        Some(a) => match &a[..] {
            "-h" | "--help" => {
                show_help();
                0
            }
            "-l" | "--list" => {
                list_ports();
                0
            }
            _ => str::parse(&a[..]).unwrap_or_else(|_| {
                eprintln!("{}: not a port number", &a);
                exit(1);
            }),
        },
        _ => 0,
    };

    let midi_out = MidiOutput::new("kb-drums output").unwrap_or_else(|e| {
        eprintln!("error: {:?}", e);
        exit(1);
    });

    let out_ports = midi_out.ports();

    if out_ports.len() == 0 {
        eprintln!("no midi output ports detected");
        exit(1);
    }
    if out_ports.len() <= port_no {
        eprintln!(
            "invalid port no '{}', have {} ports available, use with --list to see them",
            port_no,
            out_ports.len()
        );
        exit(1);
    }

    let out_port = &out_ports[port_no];
    let mut out = midi_out.connect(out_port, "kb-drums").unwrap_or_else(|e| {
        eprintln!("error connecting to midi out port: {:?}", e);
        exit(1);
    });

    let mut drum = Drum { con: out };

    enable_raw_mode();
    drum.start();
    disable_raw_mode();
}
