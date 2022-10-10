# OVERVIEW OF NUCLEOB

[nucleob](https://github.com/piotrbajdek/nucleob) performs a statistical analysis of .fasta files with either nucleotide or amino acid sequences. It is designed to give (1) highly reliable and (2) clearly formatted results, being also (3) supremely minimalist and (4) cross-platform with no system dependencies.

[keywords: amino acids, bioinformatics, biology, DNA, fasta, genes, genetics, nucleic acids, nucleobases, nucleotides, proteins, RNA, scientific computing, statistics, stats]

**As of v1.0.0-alpha.1, nucleob remains in an unstable fast-development phase. This program version is not intended for scientific research but for presentation of the technology and testing!**

# EXAMPLES

[Static link to a changeable image of the _most recent_ version of nucleob! This may include pre-releases!]

![example-image-1](https://github.com/piotrbajdek/nucleob/blob/main/docs/images/example-image-1.png?raw=true)

# CITATION AND REUSE

You can modify and fork nucleob under terms of the [MIT License](https://github.com/piotrbajdek/nucleob/blob/main/LICENSE.md). Citing nucleob in a research article, always refer to a specific program version, e.g.:

Bajdek, P., 2022. nucleob (version 1.0.0-alpha.1). [computer software] https://github.com/piotrbajdek/nucleob

# INSTALLATION ON LINUX

[nucleob](https://github.com/piotrbajdek/nucleob) should run smoothly on Windows and macOS, and can be installed by the use of [cargo](https://www.rust-lang.org/tools/install). Yet, it is being developed and tested on Fedora Linux.

## METHOD 1

**1.** Install from crates.io by the use of cargo:

_cargo install nucleob \--version 1.0.0-alpha.1_

By default, the file will be downloaded to `.cargo/bin/`, a hidden folder in your home directory.

**2a.** For convenience, you will probably want to copy nucleob to `/usr/bin/` as in Method 2 (3a, 3b).

**2b.** Alternatively, add `~/.cargo/bin` directory to your PATH variable (see documentation of your shell).

## METHOD 2

**1.** Download the binary 'nucleob' for Linux x86_64 from GitHub:

https://github.com/piotrbajdek/nucleob/releases/tag/v1.0.0-alpha.1

**2.** Make the file executable:

_sudo chmod +x ./nucleob_

**3a.** On most Linux distros, install nucleob via copying the binary to `/usr/bin/`:

_sudo cp nucleob /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp nucleob /var/usrlocal/bin/_

## METHOD 3

Download and unpack the nucleob source from GitHub. Then, build and install the program:

https://github.com/piotrbajdek/nucleob/releases/tag/v1.0.0-alpha.1

_cargo build \--release && sudo cp target/release/nucleob /usr/bin/_

# NUCLEOB CRATE ON CRATES.IO

The Rust community’s crate registry

https://crates.io/crates/nucleob
