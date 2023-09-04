# Port Sniffer (Rust)

Port Sniffer is a simple command-line tool written in Rust that allows you to scan a range of ports on a specified IP address to check for open ports. It uses multithreading to speed up the scanning process and provides a clear output of open ports.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Options](#options)
- [Examples](#examples)
- [License](#license)
- [Contributions](#contributions)
- [Author](#author)

## Installation

Before using the Port Sniffer, make sure you have Rust installed on your system. You can install Rust by following the instructions on the official [Rust website](https://www.rust-lang.org/).

Once Rust is installed, you can build the Port Sniffer from the source code:

1. Clone the repository:

   ```bash
   git clone https://github.com/Sangam-ghimire/port-sniffer-rust.git

2. Change to the project directory
    cd port-sniffer-rust

3. Build the Port Sniffer using Cargo:
    cargo build --release
This will create an executable in the target/release directory.

Usage

To use the Port Sniffer, run the executable from the command line with the desired options and arguments. The basic syntax is as follows:

./port-sniffer [OPTIONS] IP_ADDRESS

Options

    -j NUM_THREADS: Specify the number of threads to use for scanning (default is 4).
    -h, --help: Display the help message and usage instructions.

Examples

    Scan common ports on a local IP address (IPv4):
    ./port-sniffer 192.168.0.1

    Scan a specific range of ports on a remote IP address (IPv6) using 8 threads:

bash

./port-sniffer -j 8 2001:0db8:85a3:0000:0000:8a2e:0370:7334

Display the help message:

bash

./port-sniffer -h

