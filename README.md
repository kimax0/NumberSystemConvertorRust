# Number System Convertor

## About

This is a CLI program that converts numbers between any two number system with the bases 2 through 62 (base 1 is just a tally, and bases larger than 36 require characters other than 0-9 and A-Z which are non-standard and non-intuitive). It uses the standard characters 0-9 for bases 2-10, for bases 11-36 it adds capital letters A-Z to the existing digits, and then for bases 37-62 it implements lowercase letters a-z. It may seem more intuitive to use lowercase letters before uppercase letters but since hexadecimal uses A-F by default it has to follow that standard.
It's written in [Rust](https://www.rust-lang.org/).

## Usage

### [rustup](https://www.rustup.rs)

You need rustup to compile and run this program. The installation and setup process are covered on the rustup website.

### Compiling

After cloning this repository open it in a terminal and cd into the ./nsco folder. From there run

```shell
cargo build
```

### Running

You can either run the program from the same place where you compiled it. Just run

```shell
cargo run <args>
```

If you would like to run the compiled file itself, after compiling cd into ./target/debug and run

```shell
./nsco <args>
```

#### Arguements

The program takes in three arguements ```nsco <number> <base_from> <base_to>```. You provide the number you want to convert, the base in which the number currently is, and the base you want to convert it into.
