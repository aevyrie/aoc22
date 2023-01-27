use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};

use nohash_hasher::BuildNoHashHasher;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElfId(pub usize);


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Calories(pub usize);

pub struct CalorieList {
    pub map: HashMap<ElfId, Vec<Calories>, BuildNoHashHasher<usize>>,
    pub calorie_totals: BTreeMap<Calories, ElfId>,
}

impl CalorieList {
    pub fn top_n_calories(&self, n: usize) -> Calories {
        let mut cal_iter = self.calorie_totals.iter().rev();
        let mut total = 0;
        for _ in 0..n {
            total += cal_iter.next().unwrap().0 .0;
        }
        Calories(total)
    }

    pub fn load(file: impl Into<PathBuf>) -> Result<CalorieList, anyhow::Error> {
        let data = std::fs::read_to_string(file.into())?;
        let mut map = HashMap::with_hasher(BuildNoHashHasher::default());
        let mut id = ElfId(0);
        let mut buffer = Vec::new();
        for line in data.lines() {
            if line.is_empty() {
                map.insert(id, buffer.clone());
                id.0 += 1;
                buffer.clear();
            } else {
                buffer.push(Calories(line.parse()?))
            }
        }
        let mut calorie_totals = BTreeMap::new();
        for entry in map.iter() {
            let calories = entry
                .1
                .iter()
                .copied()
                .reduce(|acc, e| Calories(acc.0 + e.0))
                .unwrap_or_default();
            calorie_totals.insert(calories, *entry.0);
        }

        Ok(CalorieList {
            map,
            calorie_totals,
        })
    }
}
