use crate::note::note_name::NoteName;
use crate::pitch::relative_pitch::RelativePitch;

pub struct AbsolutePitch(u8);

impl From<RelativePitch> for AbsolutePitch {
    fn from(value: RelativePitch) -> Self {
        Self(value.0.clamp(0, i8::MAX) as u8)
    }
}

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
