extern crate chrono;
extern crate clap;

mod unit;

use std::io::{self, Read, Write, stderr};
use std::fs::{OpenOptions};

use clap::{Arg, App};
use chrono::prelude::*;

use unit::{Unit, Amount};

const CAPACITY: usize = 4096;

fn print_rate(bytes: usize, unit: &SelectedUnit, stream: &mut Write) {
    let bytes = bytes as u64 as f64;

    let data = match unit {
        SelectedUnit::Unit(unit) => Amount::new(bytes, *unit),
        SelectedUnit::Auto => Amount::auto_detect(bytes)
    };

    writeln!(stream, "{}/s", data);
}

enum SelectedUnit {
    Unit(Unit),
    Auto
}

fn main() {
    let unit_values = ["b", "k", "m", "g", "t"];

    let matches = App::new("measure")
        .version("1.0")
        .author("Mota")
        .about("Measures data transfer given in standard input")
        .arg(Arg::with_name("unit")
             .short("u")
             .long("unit")
             .value_name("UNIT")
             .required(false)
             .help("Display the result in a different unit format")
             .takes_value(true)
             .possible_values(&unit_values)
             )
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .required(false)
             .help("File to output the transfer rate to instead of stderr")
             .takes_value(true)
             )
        .get_matches();

    let handle = stderr();

    let mut stream: Box<dyn Write> = match matches.value_of("file") {
        Some(path) => {
            let file = OpenOptions::new().write(true).create(true).open(path).unwrap();
            Box::new(file)
        },
        None => Box::new(handle.lock())
    };

    let unit = match matches.value_of("unit") {
        Some(value) => {
            match value {
                "b" => SelectedUnit::Unit(Unit::Byte),
                "k" => SelectedUnit::Unit(Unit::Kilo),
                "m" => SelectedUnit::Unit(Unit::Mega),
                "g" => SelectedUnit::Unit(Unit::Giga),
                "t" => SelectedUnit::Unit(Unit::Tera),
                _ => SelectedUnit::Auto
            }
        },
        None => SelectedUnit::Auto
    };

    let mut buffer: [u8; CAPACITY] = [0; 4096];
    let mut bytes_read = 0;
    let mut dt = Local::now();

    loop {
        bytes_read += match io::stdin().read(&mut buffer) {
            Ok(count) => count,
            Err(e) => {
                panic!("An error occured: {:?}", e);
            }
        };

        match io::stdout().write(&buffer) {
            Ok(_) => (),
            Err(e) => {
                panic!("An error occured: {:?}", e);
            }
        }

        let now = Local::now();
        let diff = (now - dt).to_std().unwrap().as_secs();

        if diff >= 1 {
            dt = now;

            print_rate(bytes_read, &unit, &mut stream);
            bytes_read = 0;
        }
    }
}
