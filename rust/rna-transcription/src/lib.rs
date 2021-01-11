#[derive(Debug, PartialEq)]
pub struct Dna{
    s: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    s: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let n = vec!['A', 'C', 'T', 'G'];
        let mut i = 0;
        for c in dna.chars() {
            if !n.contains(&c) {
                return Err(i)
            }
            i += 1;
        }
        Ok(Dna {s: dna.to_string()})
    }

    pub fn into_rna(self) -> Rna {
        let mut ret = String::new();
        for c in self.s.chars() {
            //G -> C, C -> G, T -> A, A -> U
            match c {
                'G' => ret.push('C'),
                'C' => ret.push('G'),
                'T' => ret.push('A'),
                'A' => ret.push('U'),
                _ => continue,
            }
        }
        Rna {s: ret}
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let n = vec!['A', 'C', 'G', 'U'];
        let mut i = 0;
        for c in rna.chars() {
            if !n.contains(&c) {
                return Err(i)
            }
            i += 1;
        }
        Ok(Rna {s: rna.to_string()})
    }
}
