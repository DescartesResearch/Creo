use ndarray::Array1;
use statrs::distribution::{ContinuousCDF, FisherSnedecor};

pub struct LeveneResult {
    pub degrees_of_freedom: f64,
    pub estimate: f64,
    pub p_value: f64,
}

pub fn levene_test(samples: &[Array1<f64>]) -> LeveneResult {
    let k = samples.len();
    let mut n_total = 0;
    let mut zij = Vec::with_capacity(k);
    let mut ni = Vec::with_capacity(k);
    let mut zbari = Vec::with_capacity(k);
    let mut zbar = 0.0;
    for sample in samples.iter() {
        let n = sample.len();
        n_total += n;
        let n = n as f64;
        ni.push(n);
        let mean = sample.mean().expect("non empty sample array");
        let diff: Array1<f64> = sample.iter().map(|xi| (xi - mean).abs()).collect();

        let diff_mean = diff.mean().unwrap();

        zbari.push(diff_mean);
        zbar += diff_mean * n;
        zij.push(diff);
    }
    let zbar = zbar / (n_total as f64);
    let df = (n_total - k) as f64;
    let numer: f64 = df
        * (ni
            .iter()
            .zip(zbari.iter())
            .map(|(ni, meani)| ni * (meani - zbar).powi(2))
            .sum::<f64>());

    let dvar: f64 = zij
        .iter()
        .zip(zbari.iter())
        .map(|(zj, zbar)| zj.iter().map(|z| (z - zbar).powi(2)).sum::<f64>())
        .sum();
    let denom = ((k as f64) - 1.0) * dvar;

    let estimate = numer / denom;

    let distribution = FisherSnedecor::new((k - 1) as f64, df).unwrap();
    let p_value = distribution.sf(estimate);

    LeveneResult {
        degrees_of_freedom: df,
        estimate,
        p_value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use statrs::prec;
    #[test]
    fn test_levene() {
        let x = vec![
            134.0, 146.0, 104.0, 119.0, 124.0, 161.0, 107.0, 83.0, 113.0, 129.0, 97.0, 123.0,
        ];
        let y = vec![70.0, 118.0, 101.0, 85.0, 107.0, 132.0, 94.0];
        let result = levene_test(&[x.into(), y.into()]);
        assert_eq!(result.degrees_of_freedom, 17.0);
        assert!(prec::almost_eq(
            result.estimate,
            0.014721055064513417,
            1e-15
        ));
        assert!(prec::almost_eq(result.p_value, 0.9048519802923365, 1e-15));
    }

    #[test]
    fn test_levene_scipy_generic() {
        let a = vec![8.88, 9.12, 9.04, 8.98, 9.00, 9.08, 9.01, 8.85, 9.06, 8.99];
        let b = vec![8.88, 8.95, 9.29, 9.44, 9.15, 9.58, 8.36, 9.18, 8.67, 9.05];
        let c = vec![8.95, 9.12, 8.95, 8.85, 9.03, 8.84, 9.07, 8.98, 8.86, 8.98];
        let result = levene_test(&[a.into(), b.into(), c.into()]);
        assert!(prec::almost_eq(result.p_value, 0.001983795817472731, 1e-15));
    }

    #[test]
    fn test_levene_scipy_vitamin_c() {
        let small_dose = vec![
            4.2, 11.5, 7.3, 5.8, 6.4, 10.0, 11.2, 11.2, 5.2, 7.0, 15.2, 21.5, 17.6, 9.7, 14.5,
            10.0, 8.2, 9.4, 16.5, 9.7,
        ];
        let medium_dose = vec![
            16.5, 16.5, 15.2, 17.3, 22.5, 17.3, 13.6, 14.5, 18.8, 15.5, 19.7, 23.3, 23.6, 26.4,
            20.0, 25.2, 25.8, 21.2, 14.5, 27.3,
        ];
        let large_dose = vec![
            23.6, 18.5, 33.9, 25.5, 26.4, 32.5, 26.7, 21.5, 23.3, 29.5, 25.5, 26.4, 22.4, 24.5,
            24.8, 30.9, 26.4, 27.3, 29.4, 23.0,
        ];
        let result = levene_test(&[small_dose.into(), medium_dose.into(), large_dose.into()]);
        assert!(prec::almost_eq(result.estimate, 0.7327658667070045, 1e-15));
        assert!(prec::almost_eq(result.p_value, 0.4850495728974247, 1e-13));
    }

    #[test]
    fn test_data_tab_example() {
        let a = vec![21.0, 23.0, 17.0, 11.0, 9.0, 27.0, 22.0, 12.0, 20.0, 4.0];
        let b = vec![18.0, 22.0, 19.0, 26.0, 13.0, 24.0, 23.0, 17.0, 21.0, 15.0];
        let c = vec![17.0, 16.0, 23.0, 7.0, 26.0, 9.0, 25.0, 21.0, 14.0, 20.0];

        let result = levene_test(&[a.into(), b.into(), c.into()]);
        assert!(prec::almost_eq(result.p_value, 0.153, 1e-3));
        assert!(prec::almost_eq(result.estimate, 2.016, 1e-3));
    }
}
