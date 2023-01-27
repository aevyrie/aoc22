use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Sections {
    pub pairs: Vec<(u128, u128)>,
}

impl Sections {
    pub fn load(file: impl Into<PathBuf>) -> Result<Self, anyhow::Error> {
        let mut pairs = Vec::new();
        let data = std::fs::read_to_string(file.into())?;
        for line in data.lines() {
            let (elf_1, elf_2) = line.split_once(',').unwrap();
            let elf_1 = elf_1.split_once('-').unwrap();
            let elf_1 = (elf_1.0.parse().unwrap(), elf_1.1.parse().unwrap());
            let elf_2 = elf_2.split_once('-').unwrap();
            let elf_2 = (elf_2.0.parse().unwrap(), elf_2.1.parse().unwrap());
            pairs.push((elf_1.0..=elf_1.1, elf_2.0..=elf_2.1));
        }
        // Convert from ranges to bit masks, for example:
        // 2..3 -> 01100000 -> 96
        let pairs = pairs
            .drain(..)
            .map(|(r1, r2)| {
                let r1 = r1.map(|i: u32| 2u128.pow(i)).reduce(|a, x| a + x).unwrap();
                let r2 = r2.map(|i: u32| 2u128.pow(i)).reduce(|a, x| a + x).unwrap();
                (r1, r2)
            })
            .collect();
        Ok(Sections { pairs })
    }

    pub fn fully_contained(&self) -> impl Iterator<Item=&(u128, u128)> + '_ {
        self.pairs
            .iter()
            .filter(|(a, b)| {
                let c = a & b;
                c == *a || c == *b
            })
    }

    pub fn overlapped(&self) -> impl Iterator<Item=&(u128, u128)> + '_ {
        self.pairs
            .iter()
            .filter(|(a, b)| {
                a & b != 0
            })
    }
}
