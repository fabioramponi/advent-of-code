use shellexpand::tilde;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub mod arena_tree;

pub enum Purpose {
    Test,
    Challenge,
}

pub fn read_input(day: u8, purp: &Purpose) -> Vec<String> {
    let resources_path = env::var("AOC2022_RESOURCES_PATH")
        .unwrap_or(String::from_str("~/dev/advent-of-code/2022/resources").unwrap());
    let folder = format!("day_{:02}", day);
    let filename = match purp {
        Purpose::Test => "test_input.txt",
        Purpose::Challenge => "challenge.txt",
    };
    let full_path = Path::new(tilde(&resources_path).trim())
        .join(folder)
        .join(filename);
    let r = fs::read_to_string(full_path).expect("File read");
    r.split("\n").map(str::to_string).collect()
}
