use lazy_static::lazy_static;
use rand::distributions::Distribution;

lazy_static!{
    static ref RANGE: rand::distributions::Uniform<f64> = rand::distributions::Uniform::new(0.0, 1.0);
}

pub fn invert_random_matrix(size: i32) -> impl serde::Serialize {
    if size < 1 {
        panic!("Matrix size must be greater or equal to `1`, but was {}.", size)
    }
    let size = size as usize;
    let mut rng = rand::thread_rng();

    let matrix = nalgebra::DMatrix::from_iterator(size, size, RANGE.sample_iter(&mut rng).take(size*size));

    let inverse = matrix.try_inverse().unwrap();
    inverse
}

