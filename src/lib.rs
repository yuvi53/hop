use std::io::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::env;
use std::error::Error;
use dotenvy::dotenv;
use regex::Regex;
use std::iter;

#[cfg(test)]
mod tests;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Config {
    pub data_path: PathBuf,
    pub backup_path: PathBuf,
}

#[derive(Clone)]
pub struct Data {
    pub weight: f64,
    pub path: PathBuf,
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

pub fn get_data() -> Result<Vec<Data>, Box<dyn Error>> {
    let re = Regex::new(r"(?m)^([0-9]+\.?[0-9]*) ([^:\n\r]+)$").unwrap();
    let data_path = set_defaults()?.data_path;
    if !data_path.exists() {
        create_data_dir()?;
        File::create(&data_path)?;
        return Ok(Vec::new());
    }
    let hay = fs::read_to_string(&data_path)?;
    let results: Vec<Data> = re.captures_iter(&hay).map(|c| {
        let (_, [weight, path]) = c.extract();
        let path = path
            .trim();
        let weight = weight
            .parse::<f64>()
            .expect("couldn't convert &str to f64(while parsing)");
        Data {weight, path: PathBuf::from(path)}
    }).collect();
    Ok(results)
}

fn exist_in_database(data: &Vec<Data>, queried_path: &str) -> Result<bool, Box<dyn Error>> {
    let mut exist = false;
    for &Data {weight: _, ref path} in data.iter() {
        if path.to_str().unwrap() == queried_path {
            exist = true;
        }
    }
    Ok(exist)
}

pub fn add_path(data: &Vec<Data>, path: String, weight: Option<f64>) -> Result<(), Box<dyn Error>> {
    let weight = match weight {
        Some(num) => num,
        None => 10.0,
    };
    let data_path = set_defaults()?.data_path;
    if !data_path.exists() {
        create_data_dir()?;
        File::create(&data_path)?;
    }
    if &path == &env::var("HOME").unwrap() {
        return Ok(());
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&data_path)?;
    let mut buffer = String::new();
    match exist_in_database(&data, &path)? {
        false => {
            for &Data {weight, ref path} in data.iter() {
                buffer.push_str(& format!("{} {}\n", weight, path.to_str().unwrap()));
            }
            buffer.push_str(& format!("{} {}\n", weight, &path));
        },
        true => {
            for Data {weight: lweight, path: lpath} in data.iter() {
                let lpath = lpath.to_str().unwrap();
                if lpath == &path {
                    let lweight = ((lweight * lweight) + (weight * weight)).sqrt();
                    buffer.push_str(& format!("{} {}\n", lweight, lpath));
                } 
                else {
                    buffer.push_str(& format!("{} {}\n", lweight, lpath));
                }
            }
        },
    }
    write!(file, "{}", buffer)?;
    Ok(())
}

pub fn find_matches(entries: Vec<Data>, needle: String) -> Result<Vec<Data>, Box<dyn Error>> {
    let is_cwd = |entry: &Data| {
        let pwd = std::env::current_dir()
            .expect("couldn't get the working directory");
        let pwd = pwd.to_str().expect("couldn't convert pwd to &str");
        let entry_path = entry.path.to_str().unwrap();
        pwd == entry_path
    };
    let matches: Vec<Data> = ifilter(
        |entry: &Data| !is_cwd(entry) && entry.path.exists(),
        iter::chain(
            match_consecutive(needle.clone(), entries.clone()),
            match_fuzzy(needle.clone(), entries.clone(), None),
        )
    ); 
    Ok(matches)
}

fn match_consecutive(needle: String, entries: Vec<Data>) -> Vec<Data> {
    let closure = |r: &Data| r.path.ends_with(&needle);
    let results = ifilter(closure, entries);
    results
}

fn ifilter<F, I, T,>(f: F, entries: I) -> Vec<T>
    where F: Fn(&T) -> bool,
    I: IntoIterator<Item = T>,
{
    let mut results: Vec<T> = Vec::new();
    for entry in entries.into_iter() {
        if f(&entry) {
            results.push(entry)
        }
    }
    results
}

fn match_fuzzy(needle: String, entries: Vec<Data>, threshold: Option<f64>) -> Vec<Data> {
    let threshold = match threshold {
        Some(num) => num,
        None => 0.6,
    };
    let v: Vec<Data> = vec![];
    v
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
