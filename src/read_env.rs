use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

pub fn get_apikey (env_key: String) -> String {
  let env_file = File::open(".env")
      .expect("You should provide en .env file with your api key");

  let reader = BufReader::new(env_file);
  let formatted_regex = format!(r"^{}", env_key);
  let regex = Regex::new(&formatted_regex)
      .expect("Regex could not be generated");

  let lines = reader
      .lines()
      .map(|l| l.unwrap());

  lines
      // .find(|line| regex.is_match(&line)).unwrap();
      .fold(String::new(), |acc, line| {
          if regex.is_match(&line) {
              let splitted: Vec<&str> = line.split('=').collect();

              return splitted[1].to_string();
          }

          acc
      })
}