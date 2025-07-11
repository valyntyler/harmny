use crate::note::accidental::Accidental;
use crate::pitch::absolute_pitch::AbsolutePitch;

pub struct RelativePitch(pub(super) i8);

impl From<AbsolutePitch> for RelativePitch {
    fn from(value: AbsolutePitch) -> Self {
        Self(value.0.clamp(0, i8::MAX as u8) as i8)
    }
}

impl From<Option<Accidental>> for RelativePitch {
    fn from(value: Option<Accidental>) -> Self {
        Self(match value {
            None => 0,
            Some(accidental) => match accidental {
                Accidental::Sharp => 1,
                Accidental::Flat => -1,
            },
        })
    }
}
