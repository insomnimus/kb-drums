use crate::DRUMS;

pub fn run() {
	let width: usize = DRUMS.iter().map(|drum| drum.0.len()).max().unwrap();

	for drum in DRUMS {
		println!(
			"{name:width$} |  {n}",
			name = drum.0,
			width = width,
			n = drum.1
		);
	}
}
