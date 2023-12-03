use crate::aoc2023::day_to_solve_functions::DAY_TO_SOLVE_FUNCTIONS;
use crate::ui::run_cli;

mod aoc2023;
mod ui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let day = &args[1];
        DAY_TO_SOLVE_FUNCTIONS
            .get(day)
            .unwrap_or_else(|| panic!("Could not find {}", day))
            .iter()
            .for_each(|f| {
                let result = f();
                match result {
                    Ok(result) => println!("{}", result),
                    Err(error) => println!("{}", error),
                }
            });
    } else {
        run_cli(&DAY_TO_SOLVE_FUNCTIONS);
    }
}
