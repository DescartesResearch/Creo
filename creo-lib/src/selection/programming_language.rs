use rand::{seq::SliceRandom, Rng};

use crate::programming_lanuage::ProgrammingLanguage;

pub fn select_programming_language<R: Rng>(
    availabe_languages: &[ProgrammingLanguage],
    rng: &mut R,
) -> ProgrammingLanguage {
    *availabe_languages
        .choose_weighted(rng, |lang| lang.as_fraction())
        .expect("should be able to choose a random programming language")
}
