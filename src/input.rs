use device_query::Keycode;

pub enum Input {
    Letter(&'static str),
    Number(u8),
    Special(String),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Modifier {
    Shift,
    Ctrl,
    Meta,
    LAlt,
    RAlt,
    CapsLock,
}

impl Modifier {
    pub fn from_keycode(keycode: &Keycode) -> Option<Modifier> {
        match keycode {
            Keycode::LShift | Keycode::RShift => Some(Modifier::Shift),
            Keycode::LControl | Keycode::RControl => Some(Modifier::Ctrl),
            Keycode::LOption | Keycode::LAlt => Some(Modifier::LAlt),
            Keycode::ROption | Keycode::RAlt => Some(Modifier::RAlt),
            Keycode::Command | Keycode::RCommand | Keycode::LMeta | Keycode::RMeta => {
                Some(Modifier::Meta)
            }
            Keycode::CapsLock => Some(Modifier::CapsLock),
            _ => None,
        }
    }
}

impl Input {
    pub fn from_key(keycode: &Keycode) -> Self {
        match keycode {
            Keycode::A => Self::Letter("a"),
            Keycode::B => Self::Letter("b"),
            Keycode::C => Self::Letter("c"),
            Keycode::D => Self::Letter("d"),
            Keycode::E => Self::Letter("e"),
            Keycode::F => Self::Letter("f"),
            Keycode::G => Self::Letter("g"),
            Keycode::H => Self::Letter("h"),
            Keycode::I => Self::Letter("i"),
            Keycode::J => Self::Letter("j"),
            Keycode::K => Self::Letter("k"),
            Keycode::L => Self::Letter("l"),
            Keycode::M => Self::Letter("m"),
            Keycode::N => Self::Letter("n"),
            Keycode::O => Self::Letter("o"),
            Keycode::P => Self::Letter("p"),
            Keycode::Q => Self::Letter("q"),
            Keycode::R => Self::Letter("r"),
            Keycode::S => Self::Letter("s"),
            Keycode::T => Self::Letter("t"),
            Keycode::U => Self::Letter("u"),
            Keycode::V => Self::Letter("v"),
            Keycode::W => Self::Letter("w"),
            Keycode::X => Self::Letter("x"),
            Keycode::Y => Self::Letter("y"),
            Keycode::Z => Self::Letter("z"),

            Keycode::Key0 | Keycode::Numpad0 => Self::Number(0),
            Keycode::Key1 | Keycode::Numpad1 => Self::Number(1),
            Keycode::Key2 | Keycode::Numpad2 => Self::Number(2),
            Keycode::Key3 | Keycode::Numpad3 => Self::Number(3),
            Keycode::Key4 | Keycode::Numpad4 => Self::Number(4),
            Keycode::Key5 | Keycode::Numpad5 => Self::Number(5),
            Keycode::Key6 | Keycode::Numpad6 => Self::Number(6),
            Keycode::Key7 | Keycode::Numpad7 => Self::Number(7),
            Keycode::Key8 | Keycode::Numpad8 => Self::Number(8),
            Keycode::Key9 | Keycode::Numpad9 => Self::Number(9),

            Keycode::Up | Keycode::PageUp => Self::Special("arrow_up".into()),
            Keycode::Down | Keycode::PageDown => Self::Special("arrow_down".into()),
            Keycode::Left => Self::Special("arrow_left".into()),
            Keycode::Right => Self::Special("arrow_right".into()),

            Keycode::Enter | Keycode::NumpadEnter => Self::Special("enter".into()),
            Keycode::Tab => Self::Special("tab".into()),
            Keycode::Backspace => Self::Special("backspace".into()),

            _ => Self::Special("default".into()),
        }
    }
}
