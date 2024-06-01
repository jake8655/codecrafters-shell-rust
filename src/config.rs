use std::{path::PathBuf, str::FromStr};

pub struct Config {
    pub path: Vec<PathBuf>,
}

impl Config {
    pub fn new(path: Vec<PathBuf>) -> Self {
        Config { path }
    }
}

impl FromStr for Config {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(':');
        let path = split.map(PathBuf::from).collect();

        Ok(Config::new(path))
    }
}
