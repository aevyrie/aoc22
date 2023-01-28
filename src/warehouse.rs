use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Move {
    pub crates: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Crane {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Clone)]
pub struct Crates {
    pub stacks: [Vec<char>; 9],
    pub instructions: Vec<Move>,
}

impl Crates {
    pub fn load(file: impl Into<PathBuf>) -> Result<Self, anyhow::Error> {
        let mut stacks: [Vec<char>; 9] = Default::default();
        let mut instructions = Vec::new();
        let data = std::fs::read_to_string(file.into())?;
        let mut lines = data.lines().into_iter().peekable();

        // Load stacks (until we hit the empty line)
        while let Some(line) = lines.next().filter(|l| !l.is_empty()) {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|c| c.1.is_alphabetic())
                .for_each(|(i, c)| stacks[i].push(c));
        }

        // The stacks were loaded the upside down, so we need to flip them.
        stacks.iter_mut().for_each(|stack| stack.reverse());

        lines.for_each(|line| {
            let mut moves = line.split_whitespace().skip(1).step_by(2);
            let instruction = Move {
                crates: moves.next().and_then(|s| s.parse().ok()).unwrap(),
                from: moves.next().and_then(|s| s.parse().ok()).unwrap(),
                to: moves.next().and_then(|s| s.parse().ok()).unwrap(),
            };
            instructions.push(instruction);
        });

        Ok(Crates {
            stacks,
            instructions,
        })
    }

    pub fn execute_moves(&self, crane: Crane) -> Self {
        let mut new_state = self.to_owned();
        for inst in new_state.instructions.drain(..) {
            let stack = &mut new_state.stacks[inst.from - 1];
            let mut moving_crates = stack.split_off(stack.len() - inst.crates);

            // This crate picks up one by one, so the result will be flipped
            if crane == Crane::CrateMover9000 {
                moving_crates.reverse();
            }

            for moving in moving_crates.drain(..) {
                new_state.stacks[inst.to - 1].push(moving);
            }
        }
        new_state
    }

    pub fn topmost(&self) -> String {
        let mut top = String::new();
        for s in self.stacks.iter() {
            if let Some(c) = s.last() {
                top.push(*c)
            }
        }
        top
    }
}
