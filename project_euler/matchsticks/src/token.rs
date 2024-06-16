#[derive(Debug, Clone, Copy)]
pub enum Token {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Add,
    Multiply,
}

impl Token {
    pub fn stick_value(&self) -> u32 {
        match *self {
            Self::Zero => 6,
            Self::One => 2,
            Self::Two => 5,
            Self::Three => 5,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 3,
            Self::Eight => 7,
            Self::Nine => 6,
            Self::Add => 2,
            Self::Multiply => 2,
        }
    }

    pub fn value(&self) -> Option<u32> {
        match *self {
            Self::Zero => Some(0),
            Self::One => Some(1),
            Self::Two => Some(2),
            Self::Three => Some(3),
            Self::Four => Some(4),
            Self::Five => Some(5),
            Self::Six => Some(6),
            Self::Seven => Some(7),
            Self::Eight => Some(8),
            Self::Nine => Some(9),
            Self::Add => None,
            Self::Multiply => None,
        }
    }

    pub fn from_value(value: u32) -> Option<Token> {
        match value {
            0 => Some(Self::Zero),
            1 => Some(Self::One),
            2 => Some(Self::Two),
            3 => Some(Self::Three),
            4 => Some(Self::Four),
            5 => Some(Self::Five),
            6 => Some(Self::Six),
            7 => Some(Self::Seven),
            8 => Some(Self::Eight),
            9 => Some(Self::Nine),
            _ => None,
        }
    }
}
