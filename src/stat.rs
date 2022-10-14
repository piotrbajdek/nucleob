// NUCLEOB VERSION 1.0.0 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE STAT

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

// IMPORTS

use std::env;
use std::fs;
use std::path::Path;

pub mod amino_acids;
pub mod nucleobases;

// OPTIONS FOR CALCULATIONS

pub fn slct() {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";

    let args: Vec<String> = env::args().collect();

    let input1 = args.get(1).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));
    let input2 = args.get(2).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));

    if input1 == "-a" && input2 == "-a" {
        panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
    }

    if input1 == "-n" && input2 == "-n" {
        panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
    }

    if input1 == "-a" && input2 == "-n" {
        panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
    }

    if input1 == "-n" && input2 == "-a" {
        panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
    }

    // AMINO ACIDS

    if input1 == "-a" || input2 == "-a" {
        if input1 == "-a" {
            let isdir: bool = Path::new(input2).is_dir();
            let isfile: bool = Path::new(input2).is_file();
            if isdir {
                panic!("{}", red.to_owned() + input2 + " is a directory! See: --help" + reset);
            // loading all files from a directory at once may be supported in future
            } else if isfile {
                let data = fs::read_to_string(input2).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                amino_acids::a_count(&data);
            } else {
                panic!("{}", red.to_owned() + "Something went wrong reading the file!" + reset);
            }
        } else if input2 == "-a" {
            let isdir: bool = Path::new(input1).is_dir();
            let isfile: bool = Path::new(input1).is_file();
            if isdir {
                panic!("{}", red.to_owned() + input1 + " is a directory! See: --help" + reset);
            // loading all files from a directory at once may be supported in future
            } else if isfile {
                let data = fs::read_to_string(input1).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                amino_acids::a_count(&data);
            } else {
                panic!("{}", red.to_owned() + "Something went wrong reading the file!" + reset);
            }
        }
        return;
    }

    // NUCLEOBASES

    if input1 == "-n" || input2 == "-n" {
        if input1 == "-n" {
            let isdir: bool = Path::new(input2).is_dir();
            let isfile: bool = Path::new(input2).is_file();
            if isdir {
                panic!("{}", red.to_owned() + input2 + " is a directory! See: --help" + reset);
            // loading all files from a directory at once may be supported in future
            } else if isfile {
                let data = fs::read_to_string(input2).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                nucleobases::n_count(&data);
            } else {
                panic!("{}", red.to_owned() + "Something went wrong reading the file!" + reset);
            }
        } else if input2 == "-n" {
            let isdir: bool = Path::new(input1).is_dir();
            let isfile: bool = Path::new(input1).is_file();
            if isdir {
                panic!("{}", red.to_owned() + input1 + " is a directory! See: --help" + reset);
            // loading all files from a directory at once may be supported in future
            } else if isfile {
                let data = fs::read_to_string(input1).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                nucleobases::n_count(&data);
            } else {
                panic!("{}", red.to_owned() + "Something went wrong reading the file!" + reset);
            }
        }
        return;
    }

    panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
}
