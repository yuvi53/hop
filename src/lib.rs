use std::io::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::env;
use std::error::Error;
use dotenvy::dotenv;
use std::iter;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub data_path: PathBuf,
    pub backup_path: PathBuf,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Data {
    pub weight: f64,
    pub path: PathBuf,
}

pub fn set_defaults() -> Result<Config, Box<dyn Error>> {
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

pub fn load(config: Config) -> Result<Vec<Data>, Box<dyn Error>> {
    let data_path = config.data_path;
    if !data_path.exists() {
        return Ok(Vec::new());
    }
    let file = fs::read_to_string(&data_path)?;
    let results: Vec<Data> = file.lines().map(|line| {
        let v: Vec<&str> = line.split("\t").collect();
        let weight = v[0]
            .parse::<f64>()
            .expect("couldn't convert &str to f64(while parsing)");
        let path = v[1].trim();
        Data {weight, path: PathBuf::from(path)}
    }).collect();
    Ok(results)
}

pub fn save(config: Config, data: Vec<Data>) -> Result<(), Box<dyn Error>> {
    let data_path = config.data_path;
    if !data_path.exists() {
        fs::create_dir_all(
            data_path.parent().unwrap()
        )?;
        File::create(&data_path)?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&data_path)?;
    let mut buffer = String::new();
    for Data {weight, path} in data {
        let path = path.to_str().unwrap();
        buffer.push_str(& format!("{}\t{}\n", weight, path));
    }
    write!(file, "{}", buffer)?;
    Ok(())
}

fn exist_in_database(queried_path: PathBuf, data: Vec<Data>) -> bool {
    let mut exist = false;
    for Data {weight: _, path} in data {
        if path == queried_path {
            exist = true;
        }
    }
    exist
}

pub fn add_path(path: PathBuf, mut data: Vec<Data>, weight: Option<f64>) -> Vec<Data> {
    let weight = match weight {
        Some(num) => num,
        None => 10.0,
    };

    if path == PathBuf::from(env::var("HOME").unwrap()) {
        return data;
    }
    match exist_in_database(path.clone(), data.clone()) {
        false => {
            data.push(Data {weight, path});
        },
        true => {
            for Data {weight: lweight, path: lpath} in data.iter_mut() {
                if path == *lpath {
                    *lweight = ((*lweight * *lweight) + (weight * weight)).sqrt();
                } 
            }
        },
    }
    data
}

pub fn find_matches(needle: String, entries: Vec<Data>, threshold: Option<f64>) -> Vec<Data> {
    let threshold = match threshold {
        Some(num) => num,
        None => 0.6,
    };
    let is_cwd = |entry: &Data| {
        let pwd = std::env::current_dir()
            .expect("couldn't get the working directory");
        let pwd = pwd.to_str().expect("couldn't convert pwd to &str");
        let entry_path = entry.path.to_str().unwrap();
        pwd == entry_path
    };
    let meets_threshold = |entry: &Data| {
        let entry = entry.path
            .file_name()
            .expect("couldn't get the dir name")
            .to_str()
            .expect("couldn't convert OsStr into &str");
        match_percent(&entry, &needle) >= threshold
    };
    let entries: Vec<Data> = entries
        .into_iter()
        .filter(|entry| !is_cwd(entry) && entry.path.exists())
        .collect();

    iter::chain(
        entries.clone()
            .into_iter()
            .filter(|entry| entry.path.ends_with(&needle))
            .collect::<Vec<Data>>(),
        entries.clone()
            .into_iter()
            .filter(meets_threshold)
            .collect::<Vec<Data>>(),
    ).collect::<Vec<Data>>()
}

fn match_percent(s1: &str, s2: &str) -> f64 {
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
    (result as f64 * 2.0) / (m as f64 + n as f64)
}
