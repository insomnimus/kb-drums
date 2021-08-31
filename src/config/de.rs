use indexmap::{
	indexmap,
	IndexMap,
};
use serde::{
	de::Error,
	Deserializer,
	Serialize,
	Serializer,
};
use serde_derive::Deserialize;

use super::Drum;
use crate::DRUMS;

#[derive(Deserialize)]
#[serde(untagged)]

enum DrumPre<'a> {
	Num(u8),
	Str(&'a str),
}

impl<'de> serde::Deserialize<'de> for Drum {
	fn deserialize<D>(des: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		match DrumPre::deserialize(des)? {
			DrumPre::Num(n) => Ok(Self(n)),
			DrumPre::Str(s) => DRUMS
				.iter()
				.find(|note| note.0.eq(s))
				.map(|note| Self(note.1))
				.ok_or_else(|| D::Error::custom(format!("{:?} is not a known drum name", s))),
		}
	}
}

pub(super) fn return_true() -> bool {
	true
}

pub(super) fn default_keys() -> IndexMap<char, Drum> {
	indexmap! {
		'j' => Drum(36),
		'h'=> Drum(36),
		'n' => Drum(35),
		'm' => Drum(35),
		'l' => Drum(38),
		'i' => Drum(46),
		'f' => Drum(51),
		'k' => Drum(51),
		'a' => Drum(49),
		'q' => Drum(41),
		'g' => Drum(43),
		'e' => Drum(45),
		's' => Drum(44),
		'p' => Drum(44),
		'r' => Drum(47),
		'w' => Drum(40),
		'o' => Drum(40),
		't' => Drum(48),
		'b' => Drum(44),
		'd' => Drum(56),
	}
}

pub(super) fn default_volume() -> u8 {
	127
}

impl Serialize for Drum {
	fn serialize<S>(&self, se: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match DRUMS.iter().find(|note| note.1.eq(&self.0)) {
			Some((name, _)) => se.serialize_str(name),
			None => se.serialize_u8(self.0),
		}
	}
}

pub(super) fn default_presets() -> Vec<u8> {
	vec![0, 8, 16, 24, 32, 40, 48]
}
