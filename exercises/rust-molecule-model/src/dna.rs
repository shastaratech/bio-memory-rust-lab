#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Base {
    A,
    C,
    G,
    T,
}

impl Base {
    pub fn from_char(raw: char) -> Option<Self> {
        match raw {
            'A' | 'a' => Some(Self::A),
            'C' | 'c' => Some(Self::C),
            'G' | 'g' => Some(Self::G),
            'T' | 't' => Some(Self::T),
            _ => None,
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Self::A => 'A',
            Self::C => 'C',
            Self::G => 'G',
            Self::T => 'T',
        }
    }

    pub fn complement(self) -> Self {
        match self {
            Self::A => Self::T,
            Self::T => Self::A,
            Self::C => Self::G,
            Self::G => Self::C,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnaStrand {
    bases: Vec<Base>,
}

impl DnaStrand {
    pub fn new(bases: Vec<Base>) -> Self {
        Self { bases }
    }

    pub fn from_symbols(sequence: &str) -> Result<Self, String> {
        let mut bases = Vec::with_capacity(sequence.len());

        for (index, raw) in sequence.chars().enumerate() {
            let Some(base) = Base::from_char(raw) else {
                return Err(format!("unsupported DNA base at index {index}: {raw}"));
            };
            bases.push(base);
        }

        Ok(Self::new(bases))
    }

    pub fn bases(&self) -> &[Base] {
        &self.bases
    }

    pub fn len(&self) -> usize {
        self.bases.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bases.is_empty()
    }

    pub fn symbols(&self) -> String {
        self.bases.iter().map(|base| base.symbol()).collect()
    }

    pub fn complement(&self) -> Self {
        Self::new(self.bases.iter().map(|base| base.complement()).collect())
    }

    pub fn gc_count(&self) -> usize {
        self.bases
            .iter()
            .filter(|base| matches!(base, Base::G | Base::C))
            .count()
    }

    pub fn gene_region(&self, start: usize, end: usize) -> Option<&[Base]> {
        if start > end || end > self.bases.len() {
            return None;
        }

        Some(&self.bases[start..end])
    }

    pub fn codons(&self) -> Vec<[Base; 3]> {
        self.bases
            .chunks_exact(3)
            .map(|chunk| [chunk[0], chunk[1], chunk[2]])
            .collect()
    }

    pub fn apply_mutation(&mut self, mutation: Mutation) -> Result<(), String> {
        match mutation {
            Mutation::Substitute { at, base } => {
                let Some(target) = self.bases.get_mut(at) else {
                    return Err(format!("substitution index out of range: {at}"));
                };
                *target = base;
            }
            Mutation::Insert { at, base } => {
                if at > self.bases.len() {
                    return Err(format!("insertion index out of range: {at}"));
                }
                self.bases.insert(at, base);
            }
            Mutation::Delete { at } => {
                if at >= self.bases.len() {
                    return Err(format!("deletion index out of range: {at}"));
                }
                self.bases.remove(at);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mutation {
    Substitute { at: usize, base: Base },
    Insert { at: usize, base: Base },
    Delete { at: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_typed_bases_from_symbols() {
        let strand = DnaStrand::from_symbols("ACGT").expect("valid DNA sequence");

        assert_eq!(strand.bases(), &[Base::A, Base::C, Base::G, Base::T]);
        assert_eq!(strand.symbols(), "ACGT");
    }

    #[test]
    fn rejects_unsupported_symbols() {
        let error = DnaStrand::from_symbols("ACXT").expect_err("X should not parse");

        assert_eq!(error, "unsupported DNA base at index 2: X");
    }

    #[test]
    fn computes_complement_and_gc_count() {
        let strand = DnaStrand::from_symbols("ACGTTAGC").expect("valid DNA sequence");

        assert_eq!(strand.complement().symbols(), "TGCAATCG");
        assert_eq!(strand.gc_count(), 4);
    }

    #[test]
    fn borrows_gene_regions_as_slices() {
        let strand = DnaStrand::from_symbols("ACGTTAGC").expect("valid DNA sequence");

        assert_eq!(
            strand.gene_region(1, 4),
            Some(&[Base::C, Base::G, Base::T][..])
        );
        assert_eq!(strand.gene_region(4, 99), None);
    }

    #[test]
    fn groups_complete_codons() {
        let strand = DnaStrand::from_symbols("ATGCGTA").expect("valid DNA sequence");

        assert_eq!(
            strand.codons(),
            vec![[Base::A, Base::T, Base::G], [Base::C, Base::G, Base::T]]
        );
    }

    #[test]
    fn applies_mutation_edits() {
        let mut strand = DnaStrand::from_symbols("ACGT").expect("valid DNA sequence");

        strand
            .apply_mutation(Mutation::Substitute {
                at: 1,
                base: Base::T,
            })
            .expect("substitution should work");
        strand
            .apply_mutation(Mutation::Insert {
                at: 2,
                base: Base::G,
            })
            .expect("insertion should work");
        strand
            .apply_mutation(Mutation::Delete { at: 0 })
            .expect("deletion should work");

        assert_eq!(strand.symbols(), "TGGT");
    }
}
