

### Overview
The goal is to enhance the `minigrep` Rust application by adding a case-insensitive search option that users can enable via an environment variable (`IGNORE_CASE`). This feature allows users to set the environment variable once and have all their searches be case-insensitive within that terminal session.

### Step 1: Writing a Failing Test
A new test is written for the `search_case_insensitive` function. The test checks if a query like "rUsT" can match lines such as "Rust:" or "Trust me." despite the different cases.

#### Code Sample: Test for `search_case_insensitive`
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

### Step 2: Implementing `search_case_insensitive`
The `search_case_insensitive` function is implemented to convert both the query and each line of the content to lowercase before comparing them. This ensures case-insensitivity during the search.

#### Code Sample: Implementation of `search_case_insensitive`
```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

### Step 3: Modifying the `Config` Struct and `run` Function
A new boolean field `ignore_case` is added to the `Config` struct. The `run` function checks the value of `ignore_case` and decides whether to call `search` or `search_case_insensitive`.

#### Code Sample: Modified `Config` Struct and `run` Function
```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

### Step 4: Checking the Environment Variable
The `env::var` function is used to check if the `IGNORE_CASE` environment variable is set. If it is, the `ignore_case` field in `Config` is set to `true`.

#### Code Sample: Checking Environment Variable
```rust
use std::env;

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

### Conclusion
With the implementation complete, the `minigrep` program can now perform case-insensitive searches when the `IGNORE_CASE` environment variable is set. The program allows the user to control the case sensitivity of their searches via environment variables, providing a flexible and user-friendly experience.