use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut ret : usize = 0;
    let n = vec!['A', 'C', 'G', 'T'];
    if !n.contains(&nucleotide) {
        return Err(nucleotide)
    }
    for c in dna.chars() {
        match c {
            'A' => {
                if nucleotide == 'A' {ret += 1};
            },
            'C' => {
                if nucleotide == 'C' {ret += 1};
            }
            'G' => {
                if nucleotide == 'G' {ret += 1};
            },
            'T' => {
                if nucleotide == 'T' {ret += 1};
            },
            _ => {
                return Err(c)
            }
        }
    }
    Ok(ret)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut ret = HashMap::new();
    for c in ['A', 'C', 'G', 'T'].iter() {
        match count(*c, dna) {
            Ok(count) => {
                ret.insert(*c, count);
            }
            Err(e) => {
                return Err(e)
            }
        }
    }
    Ok(ret)
}
