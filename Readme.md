### Project Description

**Minigrep** is a simple command-line search tool written in Rust, inspired by the classic `grep` utility. It allows users to search for lines containing a specified query string within a file. The tool supports both case-sensitive and case-insensitive searches, with the latter being controlled via an environment variable. Additionally, Minigrep adheres to proper command-line application practices by distinguishing between standard output and standard error, making it easier to handle and redirect outputs in terminal sessions.

### Usage

1. **Running a Case-Sensitive Search:**
   ```bash
   cargo run -- <query> <file_path>
   ```
   Example:
   ```bash
   cargo run -- to poem.txt
   ```

2. **Running a Case-Insensitive Search:**
   Set the `IGNORE_CASE` environment variable before running the command.
   ```bash
   IGNORE_CASE=1 cargo run -- <query> <file_path>
   ```
   Example:
   ```bash
   IGNORE_CASE=1 cargo run -- to poem.txt
   ```
   
   *In PowerShell:*
   ```powershell
   $Env:IGNORE_CASE=1; cargo run -- to poem.txt
   ```

3. **Redirecting Output to a File:**
   You can redirect the standard output to a file while keeping error messages visible in the terminal.
   ```bash
   cargo run -- <query> <file_path> > output.txt
   ```

This tool is a great way to practice Rust concepts such as handling command-line arguments, working with environment variables, performing file I/O, and managing errors effectively.

**For detailed notes refer to the [Notes section](https://github.com/vasanthgx/minigrep/tree/master/Notes)**