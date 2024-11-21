# Static web server

A multi-threaded web server in Rust that serves static files from a
specified directory. The server should handle multiple requests concurrently using a thread pool.

To see all the commits related to this project, visit: 

https://github.com/DanielaLopez777/HTTP_server.git

## Download Rust in a Linux environment

1. Update the package list and their versions:

    `sudo apt update`

2. Install essential development tools (curl, C/C++ compiler, and build utilities) needed for some Rust dependencies:

    `sudo apt install curl gcc make build-essential -y`


3. Download and install Rust via rustup, Rust’s official toolchain installer and choose the option 1 to proceed with standard installation:

    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

4. Set up the environment variables needed for Rust and cargo to work correctly in the current shell session, allowing to use Rust and Cargo commands in the terminal:

    `source $HOME/.cargo/env`


5. Verify rust and cargo varsions

    `rustc --version`

    `cargo --version`


6. Update all the associated Rust components:

    `rustup update`


7. Install git in Ubuntu

    `sudo apt-get install git`

## Running the code

For compiling the code use:

`cargo build`

For running the code:

`cargo run`

Then, go to a web server and type 127.0.0.1:8080/
following by any file present in the static/ directory or just leave only the IP address to visualize the home page.

## Running the tests

Repair some apt packages:

`sudo dpkg --configure -a`

`sudo apt --fix-broken install`

Clean old packages:

`sudo apt clean`

`sudo apt autoremove`

Update apt:

`sudo apt update`

Install openssl:

`sudo apt install libssl-dev`

Run the tests:

`cargo test`

## References a further information

For more documentation about the internal structure of the project type in a terminal:

`cargo doc --open`

This code was base on the three subchapters of the following rust project:

https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

The tests were generated with the help of chatgpt and for syntax support the following link was consulted:

https://www.rust-lang.org/learn