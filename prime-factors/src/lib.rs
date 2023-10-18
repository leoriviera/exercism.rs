pub fn factors(n: u64) -> Vec<u64> {
    let mut n_factors = Vec::new();
    let mut ceiling = n;

    loop {
        if ceiling == 1 {
            break n_factors;
        }

        let f = (2..=ceiling).find(|f| ceiling % f == 0).unwrap();
        ceiling /= f;
        n_factors.push(f);
    }
}
