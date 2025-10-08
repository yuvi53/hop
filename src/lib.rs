use std::io::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::env;
use std::error::Error;
use dotenvy::dotenv;
use regex::Regex;

#[cfg(test)]
mod tests;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Config {
    pub data_path: PathBuf,
    pub backup_path: PathBuf,
}

fn set_defaults() -> Result<Config, Box<dyn Error>> {
    dotenv().ok();
    let data_home: PathBuf = match env::var("XDG_DATA_HOME") {
        Ok(path) => PathBuf::from(&path),
        Err(_) => {
            [
                &env::var("HOME").unwrap(),
                ".local",
                "share",
            ].iter().collect()
        },
    };
    let mut data_path = data_home.clone();
    data_path.push("hop/hop.txt");
    let mut backup_path = data_home.clone();
    backup_path.push("hop/hop.txt.bak");
    Ok(Config {data_path, backup_path})
}

fn create_data_dir() -> Result<(), Box<dyn Error>> {
    let data_path = set_defaults()?.data_path;
    fs::create_dir_all(data_path.parent().unwrap())?;
    Ok(())
}

fn get_data() -> Result<Vec<(f64, String)>, Box<dyn Error>> {
    let re = Regex::new(r"(?m)^([0-9]+\.?[0-9]*) ([^:\n\r]+)$").unwrap();
    let data_path = set_defaults()?.data_path;
    if !data_path.exists() {
        create_data_dir()?;
        File::create(&data_path)?;
        return Ok(Vec::new());
    }
    let hay = fs::read_to_string(&data_path)?;
    let results: Vec<(f64, String)> = re.captures_iter(&hay).map(|c| {
        let (_, [weight, path]) = c.extract();
        let path = path.trim();
        (weight.parse::<f64>().unwrap(), path.to_string())
    }).collect();
    Ok(results)
}

fn if_exist(queried_path: &str) -> Result<bool, Box<dyn Error>> {
    let mut exist = false;
    for (_weight, path) in get_data()? {
        if &path == queried_path {
            exist = true;
        }
    }
    Ok(exist)
}

pub fn add_path(path: String, weight: Option<f64>) -> Result<(), Box<dyn Error>> {
    let weight = match weight {
        Some(num) => num,
        None => 10.0,
    };
    let data_path = set_defaults()?.data_path;
    if !data_path.exists() {
        create_data_dir()?;
        File::create(&data_path)?;
    }
    match if_exist(&path)? {
        false => {
            let mut file = OpenOptions::new()
                .append(true)
                .open(&data_path)?;
            write!(file, "{weight} {}\n", &path)?;
        },
        true => {
            let mut file = OpenOptions::new()
                .write(true)
                .open(&data_path)?;
            let mut buffer = String::new();
            for (lweight, lpath) in get_data()? {
                if &lpath == &path {
                    let lweight = ((lweight * lweight) + (weight * weight)).sqrt();
                    buffer.push_str(& format!("{lweight} {path}\n"));
                    continue;
                }
                buffer.push_str(& format!("{lweight} {path}\n"));
            }
            write!(file, "{}", &buffer)?;
        },
    }
    Ok(())
}

pub fn search_path(query: String) -> Result<String, Box<dyn Error>> {
    let data_path = set_defaults()?.data_path;
    let paths = fs::read_to_string(data_path)?;
    let mut max_lcs: (usize, &str) = (0, "");
    for path in paths.lines() {
        let lcs = lcs(&query, &path);
        if lcs > max_lcs.0 {
            max_lcs.0 = lcs;
            max_lcs.1 = path;
        }
    }
    Ok(max_lcs.1.to_string())
} 

fn lcs(s1: &str, s2: &str) -> usize {
    let m = s1.chars().count();
    let n = s2.chars().count();
    let mut result = 0;

    for i in 0..m {
        for j in 0..n {
            let mut curr = 0;
            while (i + curr) < m 
            && (j + curr) < n 
            && s1.chars().nth(i + curr).unwrap() == s2.chars().nth(j + curr).unwrap() {
                curr+=1;
            }
            if curr > result {
                result = curr;
            }
        }
    }
    result
}
