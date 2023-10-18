#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    let non_digit = string_digits.chars().find(|c| !c.is_ascii_digit());

    if let Some(char) = non_digit {
        return Err(Error::InvalidDigit(char));
    }

    let result = string_digits
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>()
        .windows(span)
        .fold(0, |acc, w| {
            let product = Iterator::product(w.to_vec().iter());

            if product > acc {
                return product;
            }

            acc
        });

    Ok(result)
}
