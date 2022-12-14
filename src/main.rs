use std::fmt::Display;

use text_io::try_read;

use advent_of_code_2022::advent_task::{
    camp_cleaning::CampCleaning, device_cleanup::DeviceCleanup, elven_calories::ElvenCalories,
    elven_tournament::ElvenTournament, rucksack_troubles::RucksackTroubles,
    supply_stacks::SupplyStacks, treehouse_lookup::TreeHouseLookup, tuning_trouble::TuningTrouble,
    AdventTask,
};

fn main() {
    println!("Hello, Advent of Code!");
    println!("Choose, which days code you want to run! (1-24)");
    let day = try_read!().expect("I can't understand you, you had one too many eggnogs friend!");
    match day {
        1 => print_solution(ElvenCalories),
        2 => print_solution(ElvenTournament),
        3 => print_solution(RucksackTroubles),
        4 => print_solution(CampCleaning),
        5 => print_solution(SupplyStacks),
        6 => print_solution(TuningTrouble),
        7 => print_solution(DeviceCleanup),
        8 => print_solution(TreeHouseLookup),
        _ => panic!("No such day, but happy holidays nonetheless!"),
    };
}

fn print_solution<R: Display, T: AdventTask<Solution = R>>(task: T) {
    let font = neofiglet::FIGfont::standard().unwrap();
    let task_name = font.convert(task.get_name()).unwrap();
    let (first_result, second_result) = task.solve();
    println!("================================================================================");
    println!("{task_name}");
    println!("================================================================================");
    println!("First solution - {}", first_result);
    println!("Second solution - {}", second_result);
    println!("================================================================================");
}
