// NUCLEOB VERSION 1.1.1 / MIT LICENSE © 2022–2023 PIOTR BAJDEK

// MODULE STAT

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::too_many_lines)]

// IMPORTS

use std::env;
use std::fs;
use std::path::Path;

pub mod amino_acids;
pub mod nucleobases;

// OPTIONS FOR CALCULATIONS

pub fn slct(reset: &str, grey: &str, red: &str, violet: &str, yellow: &str) {
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
                for file in fs::read_dir(input2).unwrap() {
                    let data0 = file.unwrap().path();

                    let data_n = data0.clone();
                    let fl_name0 = data_n.file_name().unwrap().to_str();
                    let fl_name: &str = fl_name0.unwrap().trim();

                    let data = fs::read_to_string(data0).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                    amino_acids::a_count(&data, reset, grey, red, violet, yellow, fl_name);
                    println!();
                }
            } else if isfile {
                let fl_name: &str = input2.trim();
                let data = fs::read_to_string(input2).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                amino_acids::a_count(&data, reset, grey, red, violet, yellow, fl_name);
            } else {
                panic!("{}", red.to_owned() + "Something went wrong reading the file or directory!" + reset);
            }
        } else if input2 == "-a" {
            let isdir: bool = Path::new(input1).is_dir();
            let isfile: bool = Path::new(input1).is_file();
            if isdir {
                for file in fs::read_dir(input1).unwrap() {
                    let data0 = file.unwrap().path();

                    let data_n = data0.clone();
                    let fl_name0 = data_n.file_name().unwrap().to_str();
                    let fl_name: &str = fl_name0.unwrap().trim();

                    let data = fs::read_to_string(data0).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                    amino_acids::a_count(&data, reset, grey, red, violet, yellow, fl_name);
                    println!();
                }
            } else if isfile {
                let fl_name: &str = input1.trim();
                let data = fs::read_to_string(input1).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                amino_acids::a_count(&data, reset, grey, red, violet, yellow, fl_name);
            } else {
                panic!("{}", red.to_owned() + "Something went wrong reading the file or directory!" + reset);
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
                for file in fs::read_dir(input2).unwrap() {
                    let data0 = file.unwrap().path();

                    let data_n = data0.clone();
                    let fl_name0 = data_n.file_name().unwrap().to_str();
                    let fl_name: &str = fl_name0.unwrap().trim();

                    let data = fs::read_to_string(data0).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                    nucleobases::n_count(&data, reset, grey, red, violet, yellow, fl_name);
                    println!();
                }
            } else if isfile {
                let fl_name: &str = input2.trim();
                let data = fs::read_to_string(input2).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                nucleobases::n_count(&data, reset, grey, red, violet, yellow, fl_name);
            } else {
                panic!("{}", red.to_owned() + "Something went wrong reading the file or directory!" + reset);
            }
        } else if input2 == "-n" {
            let isdir: bool = Path::new(input1).is_dir();
            let isfile: bool = Path::new(input1).is_file();
            if isdir {
                for file in fs::read_dir(input1).unwrap() {
                    let data0 = file.unwrap().path();

                    let data_n = data0.clone();
                    let fl_name0 = data_n.file_name().unwrap().to_str();
                    let fl_name: &str = fl_name0.unwrap().trim();

                    let data = fs::read_to_string(data0).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                    nucleobases::n_count(&data, reset, grey, red, violet, yellow, fl_name);
                    println!();
                }
            } else if isfile {
                let fl_name: &str = input1.trim();
                let data = fs::read_to_string(input1).expect(&(red.to_owned() + "Something went wrong reading the file!" + reset));
                nucleobases::n_count(&data, reset, grey, red, violet, yellow, fl_name);
            } else {
                panic!("{}", red.to_owned() + "Something went wrong reading the file or directory!" + reset);
            }
        }
        return;
    }

    panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
}
