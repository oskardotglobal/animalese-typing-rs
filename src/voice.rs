use std::fmt::{Display, Formatter, Result};

pub enum AnimaleseVoice {
    Female1,
    Female2,
    Female3,
    Female4,
    Male1,
    Male2,
    Male3,
    Male4,
}

impl Display for AnimaleseVoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AnimaleseVoice::Female1 => write!(f, "female/voice_1"),
            AnimaleseVoice::Female2 => write!(f, "female/voice_2"),
            AnimaleseVoice::Female3 => write!(f, "female/voice_3"),
            AnimaleseVoice::Female4 => write!(f, "female/voice_4"),
            AnimaleseVoice::Male1 => write!(f, "male/voice_1"),
            AnimaleseVoice::Male2 => write!(f, "male/voice_2"),
            AnimaleseVoice::Male3 => write!(f, "male/voice_3"),
            AnimaleseVoice::Male4 => write!(f, "male/voice_4"),
        }
    }
}

impl From<u8> for AnimaleseVoice {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Female1,
            2 => Self::Female2,
            3 => Self::Female3,
            4 => Self::Female4,
            5 => Self::Male1,
            6 => Self::Male2,
            7 => Self::Male3,
            8 => Self::Male4,
            _ => unreachable!(),
        }
    }
}
