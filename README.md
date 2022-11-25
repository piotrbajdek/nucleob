# OVERVIEW OF NUCLEOB

[nucleob](https://github.com/piotrbajdek/nucleob) performs a simple statistical analysis of .fasta files with either nucleotide or amino acid sequences. It is designed to give (1) highly reliable and (2) clearly formatted results, being also (3) supremely minimalist and (4) cross-platform.

[keywords: amino acids, bioinformatics, biology, DNA, fasta, genes, genetics, molecular, nucleic acids, nucleobases, nucleotides, proteins, RNA, scientific computing, statistics, stats]

# EXAMPLES

![example-image-1](https://github.com/piotrbajdek/nucleob/blob/main/docs/images/example-image-1.png?raw=true)

![example-image-2](https://github.com/piotrbajdek/nucleob/blob/main/docs/images/example-image-2.png?raw=true)

# CITATION AND REUSE

You can modify and fork nucleob under terms of the [MIT License](https://github.com/piotrbajdek/nucleob/blob/main/LICENSE.md). Citing nucleob in a research article, always refer to a specific program version, e.g.:

Bajdek, P., 2022. nucleob (version 1.1.0). [computer software] https://github.com/piotrbajdek/nucleob

# INSTALLATION ON LINUX

[nucleob](https://github.com/piotrbajdek/nucleob) should run smoothly on **Windows** and **macOS**, and can be installed by the use of [cargo](https://www.rust-lang.org/tools/install). Yet, it is being developed and primarily tested on **Fedora Linux**.

nucleob v1.1.0:

– Was successfully tested on Arch Linux, Fedora Linux 37, openSUSE Tumbleweed, Ubuntu 22.04 and Ubuntu 22.10.

– Failed to run on Mageia 8 due to an old glibc version (required ≥2.34).

## METHOD 1 – BY THE USE OF CARGO

**[recommended for programmers]**

**1.** Install from crates.io by the use of cargo:

_cargo install nucleob_

By default, the file will be downloaded to `.cargo/bin/`, a hidden folder in your home directory.

**2a.** For convenience, you will probably want to copy nucleob to `/usr/bin/` as in Method 2 (3a, 3b).

**2b.** Alternatively, add `~/.cargo/bin` directory to your PATH variable (can be set up by [rustup](https://www.rust-lang.org/tools/install)).

## METHOD 2 – LINUX UNIVERSAL BINARIES

**1.** Download the distro-independent [binary](https://github.com/piotrbajdek/nucleob/releases/download/v1.1.0/nucleob) of nucleob from GitHub.

**2.** Make the file executable:

_sudo chmod +x ./nucleob_

**3a.** On most Linux distros, install nucleob via copying the binary to `/usr/bin/`:

_sudo cp nucleob /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp nucleob /var/usrlocal/bin/_

## METHOD 3 – DISTRO-SPECIFIC PACKAGES

**[recommended for most users]**

Distro-specific packages are also available for download for [.rpm](https://github.com/piotrbajdek/nucleob/releases/download/v1.1.0/nucleob-1.1.0-1.x86_64.rpm)- and [.deb](https://github.com/piotrbajdek/nucleob/releases/download/v1.1.0/nucleob_1.1.0_amd64.deb)-based Linux distros. Installation instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i nucleob-1.1.0-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install nucleob-1.1.0-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i nucleob_1.1.0_amd64.deb_

## METHOD 4 – MANUAL COMPILATION

Download and unpack the nucleob [source](https://github.com/piotrbajdek/nucleob/archive/refs/tags/v1.1.0.zip) from GitHub. Then, build and install the program:

_cargo build \--release && sudo cp target/release/nucleob /usr/bin/_
