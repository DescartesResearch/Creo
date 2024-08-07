use rand::distributions::Distribution;


pub fn generate_random_numbers(n: i32, mut min: i64, mut max: i64) -> Vec<i64> {
    if min > max {
        min = min.wrapping_add(max);
        max = min.wrapping_sub(max);
        min = min.wrapping_sub(max);
    }
    if min == max { max += 1; }
    rand::distributions::Uniform::new(min, max).sample_iter(rand::thread_rng()).take(n as usize).collect()
}
