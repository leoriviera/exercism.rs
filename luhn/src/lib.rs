/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let string = code.trim();

    if string.len() <= 1 { 
        return false;
    }

    let has_valid_chars = string.chars().all(|c| c.is_ascii_digit() || c.is_ascii_whitespace());

    if !has_valid_chars { 
        return false;
    }

    let digits = string.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>();

    let length = digits.len();

    let luhn = digits.iter().enumerate().map(|(i, c)| { 
        let reverse_index = length - i;

        let digit = c.to_digit(10).unwrap();

        if reverse_index % 2 == 1 { 
            digit
        } else { 
            let product = digit * 2;

            if product > 9 {
                print!("product {:?}", (product, digit, i, i% 2));
                product - 9
            } else { 
                product
            }
        }
    })
    .sum::<u32>();
    
    luhn % 10 == 0

}
