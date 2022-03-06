use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;

pub fn get_config (keys: [String; 2]) -> HashMap<String, String> {
    let env_file = File::open(".env")
        .expect("You should provide en .env file with your config");

    let reader = BufReader::new(env_file);

    let regexes: HashMap<String, Regex> = keys.iter()
        .fold(HashMap::new(), |mut acc, key| {
            let formatted_regex = format!(r"^{}", key);
            let regex = Regex::new(&formatted_regex)
                    .expect("Regex could not be generated");

            acc.insert(key.to_string(), regex);

            acc
        });

    let lines = reader
        .lines()
        .map(|l| l.unwrap());

    lines
        .fold(HashMap::new(), |mut acc, line| {
            let splitted: Vec<&str> = line.split('=').collect();

            if regexes.contains_key(splitted[0]) {
                acc.insert(
                    splitted[0].to_string(),
                    splitted[1].to_string()
                );
            }

            acc
        })
}