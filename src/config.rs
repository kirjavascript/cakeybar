extern crate toml;

use self::toml::value::*;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    pub theme: Option<String>,
    pub bars: Vec<BarConfig>,
}

#[derive(Debug, Clone)]
pub enum Position {
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct BarConfig {
    pub name: String,
    pub monitor_index: usize,
    pub position: Position,
    pub layout: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ComponentConfig {
    pub name: String,
    pub type_: String,
    pub properties: HashMap<String, Property>, //Value ?
}

#[derive(Debug, Clone)]
pub enum Property {
    // String(String),
    // Number(u64),
    // Array(Vec<Property>)
}

pub fn parse_config(filename: &str) -> Config {
    let file_result = File::open(filename);
    if let Err(e) = file_result {
        eprintln!("{}: {}", filename, e);
        exit(2i32);
    }

    let mut contents = String::new();
    file_result.unwrap()
        .read_to_string(&mut contents)
        .expect("something went wrong reading the config");

    let parsed_result = contents.parse::<toml::Value>();

    if let Err(e) = parsed_result {
        eprintln!("{}: {}", filename, e);
        exit(1i32);
    }

    let parsed = parsed_result.unwrap();

    // get bars

    let bar_option = parsed.get("bar");

    if bar_option.is_none() {
        eprintln!("{}: no bars specified", filename);
        exit(1i32);
    }

    let bar_table_option = parsed.get("bar").unwrap().as_table();

    if bar_table_option.is_none() {
        eprintln!("{}: bar needs to be a table like [bar.name]", filename);
        exit(1i32);
    }

    let bar_table = bar_table_option.unwrap();

    let bars: Vec<(&String, &Value)> = bar_table.iter().filter(|&(_k, v)| v.is_table()).collect();

    if bars.len() == 0 {
        eprintln!("{}: no bars defined (bars need a name like [bar.name])", filename);
        exit(1i32);
    }

    let bar_configs: Vec<BarConfig> = bars.iter().map(|&(key, value)| {
        // position
        let position = value.get("position").map(|x| x.as_str().unwrap_or(""));
        let position = match position.unwrap_or("") {
            "bottom" => Position::Bottom,
            _ => Position::Top,
        };

        // monitor
        let monitor_index = value.get("monitor").map(|x| x.as_integer().unwrap_or(0)).unwrap_or(0) as usize;

        BarConfig {
            name: key.to_string(),
            monitor_index,
            position,
            layout: get_layout(&value),
        }
    }).collect();

    // components

    // theme

    let theme = parsed.get("theme").map(|x| x.as_str().unwrap_or(""));
    let theme = theme.map(|x| x.to_string());

    // root

    let config = Config {
        theme: theme,
        bars: bar_configs,
    };

    println!("{:#?}", config);

    config
}

fn get_layout(value: &Value) -> Vec<String> {
    let empty_layout = Vec::new();
    let layout = value.get("layout").map(|x| x.as_array().unwrap_or(&empty_layout));
    let layout = layout.unwrap_or(&empty_layout).iter().filter(|x| x.is_str());
    let layout: Vec<String> = layout.map(|x| String::from(x.as_str().unwrap())).collect();
    layout
}
