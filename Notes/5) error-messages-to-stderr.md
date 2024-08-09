### Overview

The section explains how to correctly handle error messages in Rust by printing them to standard error (stderr) instead of standard output (stdout). This distinction is important because it allows users to redirect normal output to a file while keeping error messages visible on the screen.

#### Key Points:
1. **Current Behavior**: By default, the `println!` macro prints both regular output and error messages to stdout. This can cause error messages to be written to a file when stdout is redirected, which is not ideal.

2. **Problem Demonstration**: Running the command `$ cargo run > output.txt` without arguments causes an error. The error message is incorrectly saved in `output.txt` instead of being displayed on the screen.

3. **Solution**: Use the `eprintln!` macro to print error messages to stderr. This way, error messages remain visible even when stdout is redirected to a file.

4. **Implementation**: The error printing logic is centralized in the `main` function, where `println!` is replaced with `eprintln!`.

#### Code Sample: Using `eprintln!` for Errors
```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

5. **Verification**: After updating to `eprintln!`, running the program with `> output.txt` now correctly displays errors on the screen while writing only successful output to the file.

This change ensures that your command-line application behaves correctly by using stdout for successful output and stderr for error messages.