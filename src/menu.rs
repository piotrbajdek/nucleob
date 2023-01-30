// NUCLEOB VERSION 1.1.1 / MIT LICENSE © 2022–2023 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]

// IMPORTS

use std::env;
use std::process::exit;

// DOCUMENTATION

pub fn documentation(reset: &str, blue_underlined: &str, grey: &str, violet: &str, yellow: &str) {
    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // CHANGES

        if argument == "-c" || argument == "--changes" {
            println!("{}", yellow.to_owned() + "30.01.2023 v1.1.1 – Enhanced source code and layout" + reset);
            println!("25.11.2022 {}", yellow.to_owned() + "v1.1.0" + reset + " – nucleob can now read all files from a directory");
            println!("14.10.2022 {}", yellow.to_owned() + "v1.0.0" + reset + " – Implemented " + violet + "-a" + reset + ", " + violet + "-n" + reset);
            exit(0);
        }

        // CITATION

        if argument == "-C" || argument == "--citation" {
            println!("Bajdek, P., 2023. nucleob (version 1.1.1). [computer software] https://github.com/piotrbajdek/nucleob");
            exit(0);
        }

        // HELP

        if argument == "-h" || argument == "--help" {
            println!("{}", grey.to_owned() + "Usage" + reset + ":" + yellow + "    nucleob [option / file/dir] [file/dir / option]" + reset);
            println!();
            println!("{}", grey.to_owned() + "Options" + reset + ":  " + violet + "-a" + reset + "  amino acids");
            println!("          {}", violet.to_owned() + "-n" + reset + "  nucleobases");
            println!();
            println!("{}", grey.to_owned() + "Examples" + reset + ":" + yellow + " nucleob -n rRNA.fasta");
            println!("          nucleob RNA_folder -n");
            println!("{reset}");
            println!("{}", grey.to_owned() + "See also" + reset + ": " + violet + "-c" + reset + ", " + violet + "--changes" + reset + "     Show simplified change notes");
            println!("          {}", violet.to_owned() + "-C" + reset + ", " + violet + "--citation" + reset + "    Show how to cite this program");
            println!("          {}", violet.to_owned() + "-h" + reset + ", " + violet + "--help" + reset + "        Show the help menu");
            println!("          {}", violet.to_owned() + "-i" + reset + ", " + violet + "--info" + reset + "        Show contact and program info");
            println!("          {}", violet.to_owned() + "-l" + reset + ", " + violet + "--license" + reset + "     Show licensing information");
            println!("          {}", violet.to_owned() + "-v" + reset + ", " + violet + "--version" + reset + "     Show the program version");
            exit(0);
        }

        // INFO

        if argument == "-i" || argument == "--info" {
            println!("{}", grey.to_owned() + "Program" + reset + ":  " + yellow + "nucleob" + reset);
            println!("{}", grey.to_owned() + "Version" + reset + ":  1.1.1");
            println!("{}", grey.to_owned() + "Date" + reset + ":     January 30, 2023");
            println!("{}", grey.to_owned() + "Author" + reset + ":   Piotr Bajdek");
            println!("{}", grey.to_owned() + "Contact" + reset + ":  " + blue_underlined + "piotr.bajdek@proton.me" + reset);
            println!("{}", grey.to_owned() + "ORCID" + reset + ":    " + blue_underlined + "https://orcid.org/0000-0003-2678-3122" + reset);
            println!("{}", grey.to_owned() + "Source" + reset + ":   " + blue_underlined + "https://github.com/piotrbajdek/nucleob" + reset);
            println!("{}", grey.to_owned() + "License" + reset + ":  MIT License © 2022–2023 Piotr Bajdek");
            exit(0);
        }

        // LICENSE

        if argument == "-l" || argument == "--license" {
            println!("{}", yellow.to_owned() + "MIT License" + reset);
            println!();
            println!("Copyright © 2022–2023 Piotr Bajdek");
            println!();
            println!("Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:");
            println!();
            println!("The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.");
            println!();
            println!("THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");
            exit(0);
        }

        // VERSION

        if argument == "-v" || argument == "--version" {
            println!("{}", grey.to_owned() + "Version" + reset + ": " + yellow + "1.1.1" + reset);
            println!("January 30, 2023");
            exit(0);
        }
    }
}
