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

#[cfg(test)]
mod tests {
    use crate::programming_lanuage;

    use super::*;

    const ITER: usize = 10_000;

    #[test]
    fn test_programming_language_single() {
        let python = programming_lanuage::ProgrammingLanguage::Python(100);
        let rust = programming_lanuage::ProgrammingLanguage::Rust(0);

        let languages = [python, rust];
        let mut rng = rand::thread_rng();
        for _ in 0..ITER {
            let selection = select_programming_language(&languages, &mut rng);
            assert_eq!(selection, python, "expected Python but found {}", selection);
        }
    }

    #[test]
    fn test_programming_language_multi() {
        let python = programming_lanuage::ProgrammingLanguage::Python(50);
        let rust = programming_lanuage::ProgrammingLanguage::Rust(50);

        let languages = [python, rust];
        let mut rng = rand::thread_rng();
        let mut py_count = 0;
        let mut rs_count = 0;
        for _ in 0..ITER {
            let selection = select_programming_language(&languages, &mut rng);
            if selection == python {
                py_count += 1;
                continue;
            }
            if selection == rust {
                rs_count += 1;
                continue;
            }
            panic!("unexpected language: {}", selection);
        }

        // Maximum allowed difference: 10%
        let split = ITER / languages.len();
        let epsilon = split / 10;
        let min = split - epsilon;
        let max = split + epsilon;

        assert!(
            min <= py_count,
            "expected python count to be at least {}, but was {}",
            min,
            py_count
        );
        assert!(
            max >= py_count,
            "expected python count to be at most {}, but was {}",
            max,
            py_count
        );
        assert!(
            min <= rs_count,
            "expected rust count to be at least {}, but was {}",
            min,
            rs_count
        );
        assert!(
            max >= rs_count,
            "expected rust count to be at most {}, but was {}",
            max,
            rs_count
        );
    }
}
