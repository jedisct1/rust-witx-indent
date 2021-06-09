#[macro_use]
extern crate clap;

use clap::Arg;
use std::fs::File;
use std::io::{self, prelude::*};

fn main() -> Result<(), io::Error> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("tab")
                .value_name("tab string")
                .short("-t")
                .long("--tab-string")
                .multiple(false)
                .help("Tabulation string")
                .default_value("    "),
        )
        .arg(
            Arg::with_name("in")
                .value_name("input file")
                .multiple(false)
                .help("Input file")
                .required(true),
        )
        .get_matches();

    let path = matches.value_of("in").unwrap();
    let tab = matches.value_of("tab").unwrap();

    let fp = File::open(path)?;
    let reader = io::BufReader::new(fp);
    let mut indent_level = 0;
    let mut previous_line_was_blank = true;
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            if previous_line_was_blank {
                continue;
            }
            previous_line_was_blank = true;
        } else {
            previous_line_was_blank = false;
        }
        let opening_braces = line.chars().filter(|&c| c == '(').count();
        let closing_braces = line.chars().filter(|&c| c == ')').count();
        let new_indent_level = (indent_level as isize)
            .checked_add(opening_braces as isize - closing_braces as isize)
            .expect("Unexpected closing brace") as usize;
        if line.is_empty() {
            println!();
        } else if line.starts_with(")") {
            println!("{}{}", str::repeat(tab, new_indent_level), line);
        } else {
            println!("{}{}", str::repeat(tab, indent_level), line);
        }
        indent_level = new_indent_level;
    }
    if indent_level != 0 {
        panic!("Missing closing brace");
    }
    Ok(())
}
