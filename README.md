# monkey-rs 🐵
An interpreted, dynamically-typed, general purpose hobby programming language written in Rust.

Inspired by [this tutorial](https://monkeylang.org).

## Features:
1. Arithmetic and logical operators
2. Very basic IO

> I am still thinking about how I want to implement arrays and maps.
>
> I would like to be able to call functions from an array or map object like Python

## Use:
Building the language from source requires a working rust toolchain installed. Check out the tutorial [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) to setup Rust and Cargo.

Right now the only way to use the language is to build the project using Cargo. This requires a working rust toolchain installed. Check out the tutorial [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) to setup Rust and Cargo.

1. Clone the source code:
```
git clone https://github.com/wzid/monkey-rs.git
```

2. Run with file or REPL mode
```
cargo run <filename>

cargo run
```

## Language examples:
```rust
println("Salary calculator");

let name = input("Enter your name: ");

let hours_worked = input("Enter the amount of hours you work each day: ");

let days_worked = input("Enter the amount of days you work each week: ");

let hourly_wage = input("Enter your hourly pay: ");

let calculate_salary = fn(hours, days, wage) {
    hours * days * wage * 52
};

println(name, "your annual salary is", calculate_salary(hours_worked, days_worked, hourly_wage));
```