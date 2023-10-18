use num::integer::Roots;

fn is_prime(n: &i32) -> bool {
    (2..=n.sqrt()).all(|i| n % i != 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(is_prime).nth(n as usize).unwrap() as u32
}
