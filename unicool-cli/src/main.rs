extern crate clap;
extern crate unicool_lib;

use clap::{Arg, App};
use std::fs::File;
use std::io::{Read, Write};

const ESCAPE_ERROR_MESSAGE: &str = "Error while trying to escape non ascii character";
const ARG_OUTPUT: &str = "out";
const ARG_RAW: &str = "raw";
const ARG_FILE: &str = "file";

fn main() {
    let matches = App::new("Unicool cli")
        .version("1.0")
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("Escape all non ascii unicode characters in a file or raw input")
        .arg(Arg::with_name(ARG_RAW)
            .short("r")
            .takes_value(true)
            .value_name("STR")
            .help("The raw input to parse"))
        .arg(Arg::with_name(ARG_FILE)
            .short("f")
            .conflicts_with(ARG_RAW)
            .takes_value(true)
            .value_name("INPUT_FILE")
            .help("Sets the input file to use"))
        .arg(Arg::with_name(ARG_OUTPUT)
            .short("o")
            .takes_value(true)
            .value_name("OUTPUT_FILE")
            .help("The output file to write"))
        .get_matches();


    let output = matches.value_of(ARG_OUTPUT);
    if let Some(path) = matches.value_of(ARG_FILE) {
        let content = get_file(path).expect("File not found");
        let escaped = unicool_lib::convert_non_ascii_to_unicode(&content).expect(ESCAPE_ERROR_MESSAGE);

        print_or_write(output, escaped);
    } else if let Some(raw) = matches.value_of(ARG_RAW) {
        let escaped = unicool_lib::convert_non_ascii_to_unicode(&raw).expect(ESCAPE_ERROR_MESSAGE);
        print_or_write(output, escaped)
    }
}

fn print_or_write(output: Option<&str>, escaped: String) {
    match output {
        Some(path) => write_output(path, escaped).expect("Unable to write to output file"),
        None => println!("{}", escaped)
    }
}

fn get_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn write_output(path: &str, contents: String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

