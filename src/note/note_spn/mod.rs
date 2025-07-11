use crate::note::accidental::Accidental;
use crate::note::note_name::NoteName;

pub struct NoteSPN {
    pub name: NoteName,
    pub accidental: Option<Accidental>,
}
