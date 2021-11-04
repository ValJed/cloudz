use std::{fs, collections::HashMap};
use std::io::{BufReader, BufRead};
use fs::{File};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get env file content
    let api_key = read_env_file("OPEN_WEATHER".to_string());

    println!("apiKey {:?}", api_key);

    // let body = reqwest::get("https://httpbin.org/ip").await?
    //     .json::<HashMap<String, String>>().await?;
        // .text().await?;
        // .json::<HashMap<String, String>>()
        // .await?;

    Ok(())
}


fn read_env_file (env_key: String) -> String {
    let env_file = File::open(".env")
        .expect("Error when opening .env file");

    let reader = BufReader::new(env_file);
    let formatted_regex = format!(r"^{}", env_key);
    let regex = Regex::new(&formatted_regex)
        .expect("Regex could not be generated");

    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let api_key = lines
        .into_iter()
        // .find(|line| regex.is_match(&line)).unwrap();
        .fold(String::new(), |acc, line| {
            if regex.is_match(&line) {
                let splitted: Vec<&str> = line.split("=").collect();

                return splitted[1].to_string();
            }

            acc
        });

    api_key

    // for line in reader.lines() {
    //     let l = line.expect("Could not get line");
    //     println!("l {:?}", l);

    //     if regex.is_match(&l) {
    //         let splitted: Vec<&str> = l.split("=").collect();

    //         println!("splitted[1] {:?}", splitted[1]);
    //         key = splitted[1].to_string();
    //     }
    //     // let key = match line {
    //     //     Ok(l) => {
    //     //         if regex.is_match(&l) {
    //     //             let splitted: Vec<&str> = l.split("=").collect();

    //     //             splitted[1];
    //     //         }
    //     //     },
    //     //     Err(_) => println!("Error when reading line")
    //     // };
    // };
}