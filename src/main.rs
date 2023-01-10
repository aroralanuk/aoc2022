mod input_reader;
mod day_01_calorie_counting;
mod day_02_rock_paper_scissors;
mod day_03_rusksack_reorg;
mod day_04_camp_cleanup;
mod day_05_supply_stacks;
mod day_06_tuning_trouble;
mod day_07_no_space_left_on_device;
mod day_08_treetop_tree_house;
mod day_09_rope_bridge;
mod day_10_cathod_ray_tube;
mod day_11_monkey_business;
mod day_12_hill_climb_algorithm;
mod day_13_distress_signal;
mod day_14_regolith_reservoir;

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
        "07" => day_07_no_space_left_on_device::main(),
        "08" => day_08_treetop_tree_house::main(),
        "09" => day_09_rope_bridge::main(),
        "10" => day_10_cathod_ray_tube::main(),
        "11" => day_11_monkey_business::main(),
        "12" => day_12_hill_climb_algorithm::main(),
        "13" => day_13_distress_signal::main(),
        "14" => day_14_regolith_reservoir::main(),
        _ => println!("No valid day given. Possible options are: 01-25."),
    };
}


