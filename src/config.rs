use toml;
use toml::value::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub theme: Option<String>,
    pub bars: Vec<BarConfig>,
    pub components: Vec<ComponentConfig>,
}

#[derive(Debug)]
pub enum Position {
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct BarConfig {
    pub name: String,
    pub monitor_index: usize,
    pub position: Position,
    pub layout: Property,
}

#[derive(Debug)]
pub struct ComponentConfig {
    pub name: String,
    pub properties: HashMap<String, Property>,
}

impl ComponentConfig {
    pub fn get_int_or(&self, prop: &str, or: i64) -> i64 {
        let value_option = self.properties.get(prop);
        if let Some(&Property::Integer(ref val)) = value_option {
            *val
        }
        else {
            or
        }
    }
    pub fn get_bool_or(&self, prop: &str, or: bool) -> bool {
        let value_option = self.properties.get(prop);
        if let Some(&Property::Boolean(ref val)) = value_option {
            *val
        }
        else {
            or
        }
    }
    pub fn get_str_or<'a>(&'a self, prop: &str, or: &'a str) -> &'a str {
        let value_option = self.properties.get(prop);
        if let Some(&Property::String(ref val)) = value_option {
            val.as_str()
        }
        else {
            or
        }
    }
    pub fn get_vec_or(&self, prop: &str, or: Vec<Property>) -> Vec<Property> {
        let value_option = self.properties.get(prop);
        if let Some(&Property::Array(ref val)) = value_option {
            val.clone()
        }
        else {
            or
        }
    }
}

#[derive(Debug, Clone)]
pub enum Property {
    String(String),
    Integer(i64),
    Array(Vec<Property>),
    Boolean(bool),
    Object(HashMap<String, Property>),
    Null,
}

// TODO: return Result<Config, Err> instead
pub fn parse_config(filename: &str) -> Config {
    let config_dir = Path::new(filename).parent().unwrap();

    // get file
    let file_result = File::open(filename);
    if let Err(e) = file_result {
        eprintln!("{}: {}", filename, e);
        exit(2i32);
    }

    let mut contents = String::new();
    file_result.unwrap()
        .read_to_string(&mut contents)
        .expect("something went wrong reading the config");

    // parse file
    let parsed_result = contents.parse::<toml::Value>();

    if let Err(e) = parsed_result {
        eprintln!("{}: {}", filename, e);
        exit(1i32);
    }

    let parsed = parsed_result.unwrap();

    // theme

    let theme = match parsed.get("theme") {
        Some(theme) => theme.as_str().map(String::from),
        None => None,
    };
    let theme_str = theme.map(|x| get_path(x, config_dir));

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

        // layout
        let layout = value.get("layout").map(value_to_property)
            .unwrap_or(Property::Array(Vec::new()));

        BarConfig {
            name: key.to_string(),
            monitor_index,
            position,
            layout,
        }
    }).collect();

    // components

    let component_option = parsed.get("component");

    let components = if let Some(Some(component_table)) = component_option.map(|d| d.as_table()) {
        // get all component tables
        let components: Vec<(&String, &Value)> = component_table
            .iter()
            .filter(|&(_k, v)| v.is_table())
            .collect();
        let component_configs: Vec<ComponentConfig> = components
            .iter()
            .map(|&(key, value)| {
                // get properties
                let mut properties: HashMap<String, Property> = HashMap::new();
                value.as_table().unwrap().iter().for_each(|(key, value)| {
                    let key_str = key.to_string();
                    properties.insert(key_str, value_to_property(value));
                });

                // convert src prop to real path
                if let Some(&mut Property::String(ref mut src)) = properties.get_mut("src") {
                    *src = get_path(src.to_string(), config_dir);
                }

                ComponentConfig {
                    name: key.to_string(),
                    properties,
                }
            })
            .collect();

        component_configs
    } else {
        Vec::new()
    };

    // root

    let config = Config {
        theme: theme_str,
        bars: bar_configs,
        components,
    };

    // #[cfg(debug_assertions)]
    // println!("{:#?}", config);

    config
}

fn get_path(file: String, directory: &Path) -> String {
    let file_path = Path::new(&file);
    let file_path = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        directory.join(&file_path)
    }.canonicalize();
    if file_path.is_err() {
        eprintln!("{}: {:?}", &file, file_path.err().unwrap());
        exit(2i32);
    }
    file_path.unwrap().as_path().to_str().unwrap_or("").to_string()
}

fn value_to_property(value: &Value) -> Property {
    match value {
        &Value::String(ref str_) => Property::String(
            str_.to_string()
        ),
        &Value::Integer(ref int) => Property::Integer(
            *int
        ),
        &Value::Array(ref arr) => Property::Array(
            arr.iter().map(value_to_property).collect()
        ),
        &Value::Boolean(ref boolean) => Property::Boolean(
            *boolean
        ),
        &Value::Table(ref table) => {
            let mut properties: HashMap<String, Property> = HashMap::new();
            table.iter().for_each(|(k, v)| {
                properties.insert(k.to_string(), value_to_property(v));
            });
            Property::Object(properties)
        },
        _ => Property::Null,
    }
}
