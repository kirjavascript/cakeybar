use toml;
use toml::value::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
    pub global: ConfigGroup,
    pub bars: Vec<ConfigGroup>,
    pub components: Vec<ConfigGroup>,
    pub config_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ConfigGroup {
    pub name: String,
    pub properties: HashMap<String, Property>,
}

#[derive(Debug, Clone)]
pub enum Property {
    String(String),
    Integer(i64),
    Float(f64),
    Array(Vec<Property>),
    Boolean(bool),
    Object(HashMap<String, Property>),
    Null,
}

impl Config {
    pub fn get_path(&self, filename: &str) -> String {
        get_path(filename.to_string(), &self.config_dir)
    }
    pub fn get_theme(&self) -> String {
        self.get_path(self.global.get_str_or("theme", "theme.css"))
    }
    pub fn set_theme(&mut self, new_path: String) {
        self.global.properties.insert("theme".to_string(), Property::String(new_path));
    }
    pub fn get_filename(&self) -> String {
        self.get_path(self.global.get_str_or("filename", "config.toml"))
    }
    pub fn set_filename(&mut self, new_path: String) {
        self.global.properties.insert("filename".to_string(), Property::String(new_path));
    }
}

impl ConfigGroup {
    pub fn get_int_or(&self, prop: &str, or: i64) -> i64 {
        let value_option = self.properties.get(prop);
        if let Some(&Property::Integer(ref val)) = value_option {
            *val
        } else {
            or
        }
    }
    pub fn get_bool_or(&self, prop: &str, or: bool) -> bool {
        let value_option = self.properties.get(prop);
        if let Some(&Property::Boolean(ref val)) = value_option {
            *val
        } else {
            or
        }
    }
    pub fn get_str_or<'a>(&'a self, prop: &str, or: &'a str) -> &'a str {
        let value_option = self.properties.get(prop);
        if let Some(&Property::String(ref val)) = value_option {
            val.as_str()
        } else {
            or
        }
    }
    pub fn get_vec_or(&self, prop: &str, or: Vec<Property>) -> Vec<Property> {
        let value_option = self.properties.get(prop);
        if let Some(&Property::Array(ref val)) = value_option {
            val.clone()
        } else {
            or
        }
    }
    pub fn get_string(&self, prop: &str) -> Option<String> {
        let value_option = self.properties.get(prop);
        if let Some(&Property::String(ref val)) = value_option {
            Some(val.to_string())
        } else {
            None
        }
    }
    pub fn get_string_vec(&self, prop: &str) -> Vec<String> {
        self.get_vec_or(prop, vec![])
            .iter()
            .fold(vec![], |mut acc, cur| {
                if let Property::String(s) = cur {
                    acc.push(s.to_string());
                }
                acc
            })
    }
}

pub fn parse_config(filename: &str) -> Result<Config, String> {
    let file_path = Path::new(filename);
    let config_dir = file_path.parent().ok_or("getting config directory")?;
    let config_file = file_path.file_name().ok_or("getting config filename")?;

    // get file
    let mut file_result = File::open(filename).map_err(|x| x.to_string())?;

    let mut contents = String::new();
    file_result
        .read_to_string(&mut contents)
        .map_err(|x| x.to_string())?;

    // parse file
    let parsed = contents.parse::<toml::Value>().map_err(|x| x.to_string())?;

    // bar assertions

    let bar_table = parsed
        .get("bar")
        .ok_or(format!("{}: no bars specified", filename))?
        .as_table()
        .ok_or(format!(
            "{}: bar needs to be a table like [bar.name]",
            filename
        ))?;

    let bars: Vec<(&String, &Value)> = bar_table.iter().filter(|&(_k, v)| v.is_table()).collect();

    if bars.len() == 0 {
        return Err(format!(
            "{}: no bars defined (bars need a name like [bar.name])",
            filename,
        ));
    }

    // getters

    let get_table_config = |&(key, value): &(&String, &Value)| {
        // get properties
        let mut properties: HashMap<String, Property> = HashMap::new();
        value.as_table().unwrap().iter().for_each(|(key, value)| {
            let key_str = key.to_string();
            properties.insert(key_str, value_to_property(value));
        });

        ConfigGroup {
            name: key.to_string(),
            properties,
        }
    };

    let get_table_config_list = |name| -> Vec<ConfigGroup> {
        let component_option = parsed.get(name);

        if let Some(Some(component_table)) = component_option.map(|d| d.as_table()) {
            // get all component tables
            let components: Vec<(&String, &Value)> = component_table
                .iter()
                .filter(|&(_k, v)| v.is_table())
                .collect();
            let component_configs: Vec<ConfigGroup> =
                components.iter().map(get_table_config).collect();

            component_configs
        } else {
            Vec::new()
        }
    };

    // get global properties (ignore tables)

    let global = {
        let mut properties: HashMap<String, Property> = HashMap::new();
        parsed.as_table().unwrap().iter().for_each(|(key, value)| {
            if !value.is_table() {
                let key_str = key.to_string();
                properties.insert(key_str, value_to_property(value));
            }
        });

        // add filename
        properties.insert("filename".to_string(), Property::String(
            config_file.to_string_lossy().to_string()
        ));

        ConfigGroup {
            name: "global".to_string(),
            properties,
        }
    };

    // root

    let config = Config {
        global,
        bars: get_table_config_list("bar"),
        components: get_table_config_list("component"),
        config_dir: config_dir.to_path_buf(),
    };

    // #[cfg(debug_assertions)]
    // println!("{:#?}", config);

    Ok(config)
}

fn get_path(file: String, directory: &Path) -> String {
    let file_path = Path::new(&file);
    let file_path_res = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        directory.join(&file_path)
    }.canonicalize();

    if let Err(err) = file_path_res {
        error!("{}: {:?}", &file, err.to_string());
        String::from("")
    } else if let Ok(file_path) = file_path_res {
        file_path.as_path().to_str().unwrap_or("").to_string()
    } else {
        unreachable!();
    }
}

fn value_to_property(value: &Value) -> Property {
    match value {
        &Value::String(ref str_) => Property::String(str_.to_string()),
        &Value::Integer(ref int) => Property::Integer(*int),
        &Value::Float(ref float) => Property::Float(*float),
        &Value::Array(ref arr) => Property::Array(arr.iter().map(value_to_property).collect()),
        &Value::Boolean(ref boolean) => Property::Boolean(*boolean),
        &Value::Table(ref table) => {
            let mut properties: HashMap<String, Property> = HashMap::new();
            table.iter().for_each(|(k, v)| {
                properties.insert(k.to_string(), value_to_property(v));
            });
            Property::Object(properties)
        }
        _ => Property::Null,
    }
}
