use std::path::PathBuf;

pub struct Signal {
    data: Vec<char>,
}

impl Signal {
    pub fn load(file: impl Into<PathBuf>) -> Result<Self, anyhow::Error> {
        let data = std::fs::read_to_string(file.into())?.chars().collect();
        Ok(Signal{ data })
    }

    pub fn markers(&self, run_length: usize) -> impl Iterator<Item=usize> + '_ {
        let mut kernel = std::collections::VecDeque::new();

        let mut data = self.data.iter().enumerate();
        for _ in 0..run_length-1 {
            kernel.push_front(data.next().unwrap())
        }
        data.filter_map(move |input| {
            kernel.push_front(input);
            for char in kernel.iter() {
                if kernel.iter().filter(|c| c.1 == char.1).count() > 1 {
                    kernel.pop_back();
                    return None
                }
            }
            kernel.pop_back();
            return Some(input.0 + 1)
        })
    }
}

