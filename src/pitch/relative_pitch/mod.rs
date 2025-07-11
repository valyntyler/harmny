use crate::note::accidental::Accidental;

pub struct RelativePitch(pub(super) i8);

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
