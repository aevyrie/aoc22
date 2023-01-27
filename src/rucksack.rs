use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default)]
pub struct Rucksack {
    pub left: Vec<char>,
    pub right: Vec<char>,
}

pub struct RucksackInventory {
    pub list: Vec<Rucksack>,
    pub priority_map: HashMap<char, usize>,
}

pub struct RucksackAnalysis {
    pub errors: Vec<char>,
    pub badges: Vec<char>,
}

impl RucksackInventory {
    pub fn load(file: impl Into<PathBuf>) -> Result<Self, anyhow::Error> {
        let mut list = Vec::new();
        let data = std::fs::read_to_string(file.into())?;
        for line in data.lines() {
            let (l, r) = line.split_at(line.len() / 2);
            let rucksack = Rucksack {
                left: l.chars().collect(),
                right: r.chars().collect(),
            };
            list.push(rucksack);
        }
        let priority_map = Self::priority_map();
        Ok(RucksackInventory { list, priority_map })
    }

    pub fn analyze_rucksack(&self) -> RucksackAnalysis {
        let mut errors = Vec::new();
        let mut badges = Vec::new();
        for group in self.list.chunks_exact(3) {
            for sack in group.iter() {
                let error = sack.left.iter().find(|c| sack.right.contains(c)).unwrap();
                errors.push(*error);
            }
            let badge = group[0].left.iter().chain(group[0].right.iter()).find(|c| {
                let elf_2 = group[1].left.contains(c) || group[1].right.contains(c);
                let elf_3 = group[2].left.contains(c) || group[2].right.contains(c);
                elf_2 && elf_3
            }).unwrap();
            badges.push(*badge);
        }
        RucksackAnalysis { errors, badges }
    }

    pub fn sum_priorities(&self, chars: Vec<char>) -> usize {
        let mut sum = 0;
        for char in chars.iter() {
            sum += self.priority_map.get(char).unwrap();
        }
        sum
    }

    pub fn priority_map() -> HashMap<char, usize> {
        vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
            'Z',
        ]
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect()
    }
}
