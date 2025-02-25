use rand::{seq::SliceRandom, Rng};

use crate::programming_language::ProgrammingLanguage;

pub fn select_programming_language<R: Rng>(
    available_languages: &[ProgrammingLanguage],
    rng: &mut R,
) -> ProgrammingLanguage {
    *available_languages
        .choose_weighted(rng, |lang| lang.as_fraction())
        .expect("should be able to choose a random programming language")
}

#[cfg(test)]
mod tests {
    use statrs::distribution::ContinuousCDF;

    use crate::programming_language;

    use super::*;

    const COUNT: usize = 100_000;
    const P_VALUE: f64 = 0.05;

    #[test]
    fn test_programming_language_single() {
        let python = programming_language::ProgrammingLanguage::Python(100);
        let rust = programming_language::ProgrammingLanguage::Rust(0);

        let languages = [python, rust];
        let mut rng = rand::thread_rng();
        for _ in 0..COUNT {
            let selection = select_programming_language(&languages, &mut rng);
            assert_eq!(selection, python, "expected Python but found {}", selection);
        }
    }

    #[test]
    fn test_programming_language_multi() {
        let python = programming_language::ProgrammingLanguage::Python(30);
        let rust = programming_language::ProgrammingLanguage::Rust(70);

        let languages = [python, rust];
        let mut rng = rand::thread_rng();
        let mut observations = vec![0_isize; languages.len()];
        for _ in 0..COUNT {
            let selection = select_programming_language(&languages, &mut rng);
            let idx = languages
                .iter()
                .position(|lang| *lang == selection)
                .expect("to find a language that matches the selection");
            observations[idx] += 1;
        }

        let expected: Vec<_> = languages
            .iter()
            .map(|lang| (lang.as_fraction() as f64 / 100.0) * COUNT as f64)
            .collect();

        let chi_squared: f64 = observations
            .iter()
            .zip(expected)
            .map(|(got, want)| (got.pow(2) as f64 / want) - COUNT as f64)
            .sum();
        let chi_dist = statrs::distribution::ChiSquared::new((languages.len() - 1) as f64).unwrap();
        let p = 1.0 - P_VALUE;
        let cutoff = chi_dist.inverse_cdf(p);
        assert!(
            chi_squared <= cutoff,
            "expected {} <= {}",
            chi_squared,
            cutoff
        );
    }
}
