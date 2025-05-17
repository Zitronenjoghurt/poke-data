#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Language {
    JapaneseKata = 1,
    JapaneseRomaji = 2,
    Korean = 3,
    Chinese = 4,
    French = 5,
    German = 6,
    Spanish = 7,
    Italian = 8,
    #[default]
    English = 9,
    Czech = 10,
}

impl From<u8> for Language {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::JapaneseKata,
            2 => Self::JapaneseRomaji,
            3 => Self::Korean,
            4 => Self::Chinese,
            5 => Self::French,
            6 => Self::German,
            7 => Self::Spanish,
            8 => Self::Italian,
            9 => Self::English,
            10 => Self::Czech,
            _ => Self::default(),
        }
    }
}
