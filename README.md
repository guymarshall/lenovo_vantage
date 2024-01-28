# Lenovo Vantage

This is a Rust command-line utility for managing certain settings on certain Lenovo laptops. Lenovo Vantage is an application that is only installable on Windows via the Microsoft Store, this project aims to replicate the features of that software but on Linux instead.

## Features

- Toggle Fn Lock between 0 and 1
- Toggle Conservation Mode between 0 and 1.
- More settings to come...

## Prerequisites

- Rust programming language and Cargo: [Install Rust](https://www.rust-lang.org/tools/install)
- Lenovo laptop running Linux

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/guymarshall/lenovo_vantage.git
   ```

2. Change into the project directory:
    ```bash
    cd lenovo_vantage
   ```

3. Run the project:
    ```bash
    cargo run --release
   ```

## Usage
- Use up/down arrow keys to change selection.
- Use left/right arrow keys to change selected value.
- Press q or Esc to quit the application.

## Troubleshooting
- If you encounter anything, please open a new issue!