use crate::{
    calories::CalorieList,
    filesystem::Filesystem,
    rps::Strategy,
    rucksack::RucksackInventory,
    sections::Sections,
    signal::Signal,
    warehouse::{Crane, Crates},
};

pub mod calories;
pub mod filesystem;
pub mod rps;
pub mod rucksack;
pub mod sections;
pub mod signal;
pub mod warehouse;

fn main() -> Result<(), anyhow::Error> {
    // Day 1
    let calories = CalorieList::load("./inputs/calories.txt")?;
    dbg!(calories.top_n_calories(1).0);
    dbg!(calories.top_n_calories(3).0);

    // Day 2
    dbg!(Strategy::load_incorrect("./inputs/rps_strategy.txt")?.run_and_score());
    dbg!(Strategy::load("./inputs/rps_strategy.txt")?.run_and_score());

    // Day 3
    let inventory = RucksackInventory::load("./inputs/rucksack_list.txt")?;
    let analysis = inventory.analyze_rucksack();
    dbg!(inventory.sum_priorities(analysis.errors));
    dbg!(inventory.sum_priorities(analysis.badges));

    // Day 4
    let sections = Sections::load("./inputs/sections.txt")?;
    dbg!(sections.fully_contained().count());
    dbg!(sections.overlapped().count());

    // Day 5
    let crates = Crates::load("./inputs/crates.txt")?;
    dbg!(crates.execute_moves(Crane::CrateMover9000).topmost());
    dbg!(crates.execute_moves(Crane::CrateMover9001).topmost());

    // Day 6
    let signal = Signal::load("./inputs/signal.txt")?;
    dbg!(signal.markers(4).next().unwrap());
    dbg!(signal.markers(14).next().unwrap());

    // Day 7
    let filesystem = Filesystem::load("./inputs/filesystem.txt")?;
    dbg!(filesystem.dirs_up_to(100_000).sum::<usize>());
    dbg!(filesystem.dir_to_delete(70_000_000, 30_000_000).size);

    Ok(())
}
