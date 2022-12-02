mod input_reader;
mod day_01_calorie_counting;

fn main() {
    let day: String = std::env::args().nth(1).expect(
        "No day given. Possible options are: 01-25."
    );
    let day_slice: &str = day.as_str();

    match day_slice {
        "01" => day_01_calorie_counting::run(),
        _ => println!("No valid day given. Possible options are: 01-25."),
    };
}
