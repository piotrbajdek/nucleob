// NUCLEOB VERSION 1.1.0 / MIT LICENSE © 2022 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod menu;
pub mod stat;

// MAIN

fn main() {
    let reset = "\x1b[0m";
    let yellow = "\x1b[93m";
    let red = "\x1b[31m";
    let blue_underlined = "\x1b[34;4m";
    let cyan = "\x1b[36m";

    menu::documentation(reset, yellow, blue_underlined, cyan);
    stat::slct(reset, yellow, red, cyan);
}
