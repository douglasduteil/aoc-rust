/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
// use aoc_cli;
use std::process;

use advent_of_code::helpers::LATEST_AOC_YEAR;

struct Args {
    day: u8,
    year: Option<u16>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    Ok(Args {
        day: args.free_from_str()?,
        year: args.opt_value_from_str(["-y", "--year"])?,
    })
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Failed to process arguments: {}", e);
            process::exit(1);
        }
    };

    if advent_of_code::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    let year = args.year.unwrap_or(LATEST_AOC_YEAR);

    match advent_of_code::read(args.day, year) {
        Ok(cmd_output) => {
            if !cmd_output.status.success() {
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("failed to spawn aoc-cli: {}", e);
            process::exit(1);
        }
    }
}
