
###  TDD Process for Implementing the `search` Function in Rust

**Test-Driven Development (TDD) Steps:**
1. **Write a Failing Test:** Start by writing a test for the functionality you want to implement. In this case, the `search` function is tested to return lines containing a query string from a given text.
2. **Implement Minimal Code:** Write just enough code to make the test compile and run. Initially, this might involve returning a placeholder value.
3. **Run the Test and Ensure It Fails:** Run the test to verify that it fails as expected.
4. **Develop the Functionality:** Gradually build the functionality to make the test pass. This involves implementing the actual logic in the function.
5. **Refactor and Improve:** Refactor the code while keeping the tests green (i.e., all tests passing).

**Example Implementation:**

1. **Writing the Failing Test:**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn one_result() {
           let query = "duct";
           let contents = "\
Rust:
safe, fast, productive.
Pick three.";

           assert_eq!(vec!["safe, fast, productive."], search(query, contents));
       }
   }
   ```

2. **Initial Implementation to Compile the Test:**
   ```rust
   pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
       vec![]
   }
   ```

3. **Iterating Through Lines and Searching:**
   ```rust
   pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
       for line in contents.lines() {
           if line.contains(query) {
               // do something with line
           }
       }
   }
   ```

4. **Final Implementation of the `search` Function:**
   ```rust
   pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
       let mut results = Vec::new();

       for line in contents.lines() {
           if line.contains(query) {
               results.push(line);
           }
       }

       results
   }
   ```

5. **Using the `search` Function in the `run` Function:**
   ```rust
   pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
       let contents = fs::read_to_string(config.file_path)?;

       for line in search(&config.query, &contents) {
           println!("{line}");
       }

       Ok(())
   }
   ```

**Outcome:**
- After implementing the `search` function, the tests should pass, confirming that the function behaves as expected. This implementation can then be used in the main program to search through files for specific strings. The approach allows for easy refactoring and extension of the code while maintaining confidence in its correctness.