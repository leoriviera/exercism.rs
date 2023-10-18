use std::collections::HashMap;

static VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;

    for char in dna.chars() {
        if !VALID_NUCLEOTIDES.contains(&char) {
            return Err(char);
        }

        if char == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut h: HashMap<char, usize> = HashMap::new();

    for nucleotide in VALID_NUCLEOTIDES {
        h.insert(nucleotide, count(nucleotide, dna)?);
    }

    Ok(h)
}
