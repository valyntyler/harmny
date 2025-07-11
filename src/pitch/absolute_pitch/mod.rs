use crate::note::note_name::NoteName;

pub struct AbsolutePitch(u8);

impl From<NoteName> for AbsolutePitch {
    fn from(value: NoteName) -> Self {
        Self(match value {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        })
    }
}
