mod ansi_us;
mod iso_de;

use crate::input::{Input, Modifier};
use device_query::Keycode;
use moka::sync::Cache;

pub use ansi_us::AnsiUsLayout;
pub use iso_de::IsoDeLayout;

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
