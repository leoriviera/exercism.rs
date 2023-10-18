pub fn collatz(n: u64) -> Option<u64> {
    let mut current_n: Option<u64> = Some(n);
    let mut iterations = 0;

    loop {
        match current_n {
            None => break None,
            Some(0) => break None,
            Some(1) => break Some(iterations),
            Some(m) if m % 2 == 0 => current_n = Some(m / 2),
            Some(m) => current_n = m.checked_mul(3)?.checked_add(1),
        }

        iterations += 1;
    }
}
