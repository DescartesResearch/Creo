use std::collections::HashMap;

struct Sieve{
    q: u64,
    seen: HashMap<u64, Vec<u64>>
}

impl Sieve {
    fn new() -> Self {
        Self { q: 1, seen: HashMap::default() }
    }
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let val = loop  {
            self.q += 1;
            if !self.seen.contains_key(&self.q) {
                self.seen.insert(self.q*self.q, vec![self.q]);
                break self.q
            } else {
                for p in self.seen[&self.q].clone() {
                    let v = self.seen.entry(self.q + p).or_default();
                    v.push(p);
                }
            };
        };
        Some(val)
    }
}

pub fn generate_first_primes(n: i32) -> Vec<u64> {
    let sieve = Sieve::new();
    sieve.into_iter().take(n as usize).collect()
}

