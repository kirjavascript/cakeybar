use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};

type BoxErr = Box<dyn std::error::Error>;

pub fn read_file(path: &str) -> Result<String, Error> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

pub fn write_raw(path: &str, data: &[u8]) -> Result<(), Error> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

pub fn _write_file(path: &str, data: &str) -> Result<(), Error> {
    write_raw(path, data.as_bytes())?;
    Ok(())
}

pub fn write_data<T>(path: &str, data: T) -> Result<(), BoxErr>
    where T: Serialize
{
    let encoded: Vec<u8> = serialize(&data)?;
    write_raw(path, &encoded)?;
    Ok(())
}

pub fn read_data<T>(path: &str) -> Result<T, BoxErr>
    where for<'a> T: Deserialize<'a>
{
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let data = deserialize(&contents)?;
    Ok(data)
}
