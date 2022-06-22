use crate::gramatik::Gramatik;

/// Represents a language.
pub struct Language {}

impl From<Gramatik> for Language {
    fn from(g: Gramatik) -> Self {
        Self {}
    }
}
