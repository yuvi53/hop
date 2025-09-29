use std::io::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::env;
use std::error::Error;
use dotenvy::dotenv;

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

fn if_exist(queried_path: &str) -> Result<bool, Box<dyn Error>> {
    let data_path = set_defaults()?.data_path;
    if !data_path.exists() {
        create_data_dir()?;
        File::create(&data_path)?;
        return Ok(false)
    }
    let data = fs::read_to_string(&data_path)?;
    let paths: Vec<&str> = data.lines().collect();
    let mut exist = false;
    for path in paths {
        let path = path.trim();
        if path == queried_path {
            exist = true;
        }
    }
    Ok(exist)
}

pub fn add_path(path: String) -> Result<(), Box<dyn Error>> {
    let data_path = set_defaults()?.data_path;
    if !data_path.exists() {
        create_data_dir()?;
        File::create(&data_path)?;
    }
    if let false = if_exist(&path)? {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&data_path)?;
        write!(file, "{}\n", &path)?;
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
