use clap::Parser;
use days;
use utils::Purpose;

#[derive(Parser, Debug)]
struct Arguments {
    #[arg(short, long, default_value_t = 0)]
    day: u8,
    #[arg(short, long)]
    resources_path: Option<String>,
}

fn main() {
    let arguments = Arguments::parse();
    if arguments.day == 0 {
        for i in 1..=25 {
            run_challenge(i);
        }
    } else {
        run_challenge(arguments.day);
    }
}

fn run_challenge(day: u8) {
    let day_challenge = days::get_day(day, Purpose::Challenge);
    match day_challenge {
        Some(b) => {
            println!("Day {:02} - Part 1 : {}", day, (*b).part_1());
            println!("Day {:02} - Part 2 : {}", day, (*b).part_2());
        }
        None => return,
    }
}
