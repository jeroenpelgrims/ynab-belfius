mod csv;
mod input;
use clap::{Arg, Command};
use csv::parse_csv;
use input::read_input;
use std::fs;

fn get_args() -> (String, String) {
    let args = Command::new("YNAB-Belfius")
        .arg(Arg::new("in_file"))
        .get_matches();
    let in_path = args.get_one::<String>("in_file").unwrap();
    let out_path = in_path.replace(".csv", ".ynab.csv");
    (in_path.clone(), out_path)
}

fn main() {
    let (in_path, out_path) = get_args();
    let input = read_input(in_path);
    let original = parse_csv(input);
    let new_csv = csv::to_ynab_format(original);
    let _ = fs::write(out_path, new_csv);
}
