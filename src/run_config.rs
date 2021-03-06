use std::{fmt, path::PathBuf};

use serde::{
    de,
    de::{DeserializeOwned, Unexpected},
    Deserialize, Deserializer,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Part {:?}", self)
    }
}

impl<'de> Deserialize<'de> for Part {
    fn deserialize<D>(deserializer: D) -> Result<Part, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input_value = String::deserialize(deserializer)?;

        match input_value.to_lowercase().as_ref() {
            "one" => Ok(Part::One),
            "two" => Ok(Part::Two),
            unknown => Err(de::Error::invalid_value(
                Unexpected::Str(unknown),
                &"one of [\"one\", \"two\"]",
            )),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub part: Part,
    #[serde(default = "Config::default_input_file")]
    pub input_file: PathBuf,
}

impl Config {
    fn default_input_file() -> PathBuf {
        "full.txt".into()
    }
}

pub fn get_config() -> Result<Config, envy::Error> {
    get_custom_config()
}

pub fn get_custom_config<T>() -> Result<T, envy::Error>
where
    T: DeserializeOwned,
{
    envy::prefixed("APP_").from_env::<T>()
}
