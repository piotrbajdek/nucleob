// NUCLEOB VERSION 1.0.0-ALPHA.1 / MIT LICENSE © 2022 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod menu;
pub mod stat;

// MAIN

fn main() {
    menu::documentation();
    stat::slct();
}
