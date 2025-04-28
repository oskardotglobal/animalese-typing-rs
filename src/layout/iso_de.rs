use crate::{
    input::{Input, Modifier},
    layout::{Layout, key, shift, special},
};
use device_query::Keycode;

pub struct IsoDeLayout;

macro alt_gr($key:tt) {
    ([Modifier::RAlt], Keycode::$key)
}

impl Layout for IsoDeLayout {
    fn parse_key_chord_impl(&self, keycode: &Keycode, modifiers: &[Modifier]) -> Option<Input> {
        if Modifier::from_keycode(keycode).is_some() {
            return None;
        }

        match (modifiers, keycode) {
            shift!(Key0) => special!("parenthesis_closed"),
            alt_gr!(Key0) => special!("brace_closed"),

            shift!(Key1) => Some(Input::Letter("Deska")),
            shift!(Key4) | alt_gr!(E) => special!("dollar"),
            shift!(Key5) => special!("percent"),
            shift!(Key6) => special!("ampersand"),

            shift!(Key7) => special!("slash_forward"),
            alt_gr!(Key7) => special!("brace_open"),

            shift!(Key8) => special!("parenthesis_open"),
            alt_gr!(Key8) => special!("bracket_open"),

            shift!(Key9) => special!("parenthesis_closed"),
            alt_gr!(Key9) => special!("bracket_closed"),

            shift!(Minus) => Some(Input::Letter("Gwah")),
            alt_gr!(Minus) => special!("slash_back"),

            shift!(RightBracket) => special!("asterisk"),
            alt_gr!(RightBracket) => special!("tilde"),

            alt_gr!(Q) => special!("at"),

            key!(Grave) => special!("caret"),
            key!(BackSlash) => special!("pound"),
            key!(LeftBracket) => Some(Input::Letter("u")),
            key!(Semicolon) => Some(Input::Letter("o")),
            key!(Apostrophe) => Some(Input::Letter("a")),

            _ => Some(Input::from_key(keycode)),
        }
    }
}
