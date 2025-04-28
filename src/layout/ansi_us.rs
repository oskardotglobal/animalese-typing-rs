use crate::{
    input::{Input, Modifier},
    layout::{Layout, key, shift, special},
};
use device_query::Keycode;

pub struct AnsiUsLayout;

impl Layout for AnsiUsLayout {
    fn parse_key_chord_impl(&self, keycode: &Keycode, modifiers: &[Modifier]) -> Option<Input> {
        if Modifier::from_keycode(keycode).is_some() {
            return None;
        }

        match (modifiers, keycode) {
            shift!(Key1) => Some(Input::Letter("Deska")),
            shift!(Key2) => special!("at"),
            shift!(Key3) => special!("pound"),
            shift!(Key4) => special!("dollar"),
            shift!(Key5) => special!("percent"),
            shift!(Key6) => special!("caret"),
            shift!(Key7) => special!("ampersand"),
            shift!(Key8) => special!("asterisk"),
            shift!(Key9) => special!("parenthesis_open"),
            shift!(Key0) => special!("parenthesis_closed"),

            shift!(LeftBracket) => special!("brace_closed"),
            shift!(RightBracket) => special!("brace_open"),
            shift!(Slash) => Some(Input::Letter("Gwah")),
            shift!(Grave) => special!("tilde"),

            key!(LeftBracket) => special!("bracket_open"),
            key!(RightBracket) => special!("bracket_closed"),
            key!(Slash) => special!("slash_forward"),
            key!(BackSlash) => special!("slash_back"),

            _ => Some(Input::from_key(keycode)),
        }
    }
}
