mod input_reader;
mod day_01_calorie_counting;
mod day_02_rock_paper_scissors;
mod day_03_rusksack_reorg;
mod day_04_camp_cleanup;
mod day_05_supply_stacks;
mod day_06_tuning_trouble;

fn main() {
    let day: String = std::env::args().nth(1).expect(
        "No day given. Possible options are: 01-25."
    );
    let day_slice: &str = day.as_str();

    match day_slice {
        "01" => day_01_calorie_counting::main(),
        "02" => day_02_rock_paper_scissors::main(),
        "03" => day_03_rusksack_reorg::main(),
        "04" => day_04_camp_cleanup::main(),
        "05" => day_05_supply_stacks::main(),
        "06" => day_06_tuning_trouble::main(),
        _ => println!("No valid day given. Possible options are: 01-25."),
    };
}


