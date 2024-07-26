use std::{env, fs};
use std::error::Error as StdError;


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please provide enough arguments representing query and file-path");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: ignore_case,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    pub row: u32,
    pub col: usize,
    pub text: &'a str,
}

pub fn run(config: Config) -> Result<(), Box<dyn StdError>> {
    let contents = fs::read_to_string(&config.file_path)?;

    // println!("Arguments: \nSearch = {} \nFile = {} \nContent = {}", config.query, config.file_path, contents);

    let found = match config.ignore_case {
        true => search(&config.query, &contents, |s: &str| {
            s.to_lowercase()
        }),
        false => search(&config.query, &contents, |s: &str| {
            s.to_string()
        }),
    };

    for line in found {
        println!("{}-{} | {}", pad(line.row.to_string(), 2), pad(line.col.to_string(), 2), line.text);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, formatter: impl Fn(&str) -> String) -> Vec<Line<'a>> {
    let query_formatted = formatter(query);
    let query_len = query_formatted.len();

    let mut result: Vec<Line<'a>> = vec![];
    let mut row: u32 = 1;

    for line in contents.lines() {
        let mut column = query_len;

        loop {
            if column > line.len() {
                break;
            }

            let start = column - query_len;
            let sliced = formatter(&line[start..column]);
            let is_matched = sliced.contains(&query_formatted);

            column += 1;

            if !is_matched {
                continue;
            }

            result.push(Line {
                row: row,
                col: start + 1,
                text: &line,
            });

            break;
        }

        row += 1;
    }

    result
}

fn pad(n: String, l: usize) -> String {
    let padded = l - n.len();
    let mut zeros: Vec<&str> = vec![];

    for _ in 1..=padded {
        zeros.push("0");
    }

    format!("{}{}", zeros.join(""), n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_case_sensitive() {
        let found = Line {
            row: 2,
            col: 16,
            text: "safe, fast, productive.",
        };
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec![found],
            search(query, contents, |s: &str| {
                s.to_string()
            })
        );
    }

    #[test]
    fn should_find_case_insensitve() {
        let found1 = Line {
            row: 1,
            col: 1,
            text: "Rust:",
        };
        let found2 = Line {
            row: 4,
            col: 2,
            text: "Trust me.",
        };
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![found1, found2],
            search(query, contents, |s: &str| {
                s.to_lowercase()
            })
        );
    }
}