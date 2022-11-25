// NUCLEOB VERSION 1.1.0 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]

// IMPORTS

use std::env;
use std::process::exit;

// DOCUMENTATION

pub fn documentation(reset: &str, yellow: &str, blue_underlined: &str, cyan: &str) {
    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // CHANGES

        if argument == "-c" || argument == "--changes" {
            println!("25.11.2022 v1.1.0 – nucleob can now read all files from a directory");
            println!("14.10.2022 v1.0.0 – Implemented{}", cyan.to_owned() + " -a" + reset + ", " + cyan + "-n" + reset);
            exit(0);
        }

        // CITATION

        if argument == "-C" || argument == "--citation" {
            println!("Bajdek, P., 2022. nucleob (version 1.1.0). [computer software] https://github.com/piotrbajdek/nucleob");
            exit(0);
        }

        // HELP

        if argument == "-h" || argument == "--help" {
            println!("Usage:{}", yellow.to_owned() + "    nucleob [OPTION / FILE/DIR] [FILE/DIR / OPTION]" + reset);
            println!();
            println!("Options:  {}", cyan.to_owned() + "-a" + reset + "  amino acids");
            println!("          {}", cyan.to_owned() + "-n" + reset + "  nucleobases");
            println!();
            println!("Examples:{}", yellow.to_owned() + " nucleob -n rRNA.fasta");
            println!("          nucleob RNA_folder -n");
            println!("{}", reset);
            println!("See also: {}", cyan.to_owned() + "-c" + reset + ", " + cyan + "--changes" + reset + "     Show simplified change notes");
            println!("          {}", cyan.to_owned() + "-C" + reset + ", " + cyan + "--citation" + reset + "    Show how to cite this program");
            println!("          {}", cyan.to_owned() + "-h" + reset + ", " + cyan + "--help" + reset + "        Show this help");
            println!("          {}", cyan.to_owned() + "-i" + reset + ", " + cyan + "--info" + reset + "        Show contact and program info");
            println!("          {}", cyan.to_owned() + "-l" + reset + ", " + cyan + "--license" + reset + "     Show licensing information");
            println!("          {}", cyan.to_owned() + "-v" + reset + ", " + cyan + "--version" + reset + "     Show the program version");
            exit(0);
        }

        // INFO

        if argument == "-i" || argument == "--info" {
            println!("Program:  {}", yellow.to_owned() + "nucleob" + reset);
            println!("Version:  1.1.0");
            println!("Date:     November 25, 2022");
            println!("Author:   Piotr Bajdek (Poland)");
            println!("Contact:  {}", blue_underlined.to_owned() + "piotr.bajdek@proton.me" + reset);
            println!("ORCID:    {}", blue_underlined.to_owned() + "https://orcid.org/0000-0003-2678-3122" + reset);
            println!("Source:   {}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/nucleob" + reset);
            println!("License:  MIT License © 2022 Piotr Bajdek");
            exit(0);
        }

        // LICENSE

        if argument == "-l" || argument == "--license" {
            println!("{}", yellow.to_owned() + "MIT License" + reset);
            println!();
            println!("Copyright © 2022 Piotr Bajdek");
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
            println!("Version: {}", yellow.to_owned() + "1.1.0" + reset);
            println!("November 25, 2022");
            exit(0);
        }
    }
}
