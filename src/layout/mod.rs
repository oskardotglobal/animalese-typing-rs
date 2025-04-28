mod ansi_us;
mod iso_de;

use crate::{
    input::{Input, Modifier},
    layout::{ansi_us::AnsiUsLayout, iso_de::IsoDeLayout},
};

use clap::ValueEnum;
use device_query::Keycode;
use moka::sync::Cache;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Layouts {
    AnsiUs,
    IsoDe,
}

impl From<Layouts> for Box<dyn Layout> {
    fn from(value: Layouts) -> Self {
        match value {
            Layouts::AnsiUs => Box::new(AnsiUsLayout),
            Layouts::IsoDe => Box::new(IsoDeLayout),
        }
    }
}

pub trait Layout {
    fn parse_key_chord(
        &self,
        keycode: &Keycode,
        modifiers: &Cache<Modifier, bool>,
    ) -> Option<Input> {
        self.parse_key_chord_impl(
            keycode,
            modifiers
                .iter()
                .map(|(k, _)| *k)
                .collect::<Vec<Modifier>>()
                .as_slice(),
        )
    }

    fn parse_key_chord_impl(&self, keycode: &Keycode, modifiers: &[Modifier]) -> Option<Input>;
}

macro special($name:expr) {
    Some(Input::Special($name.into()))
}

macro shift($key:tt) {
    ([Modifier::Shift], Keycode::$key)
}

macro key($key:tt) {
    (_, Keycode::$key)
}
