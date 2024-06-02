use std::path::PathBuf;

pub struct Config {
    pub path: Vec<PathBuf>,
    pub home: PathBuf,
}

impl Config {
    pub fn new(path: Vec<PathBuf>, home: PathBuf) -> Self {
        Config { path, home }
    }

    pub fn from_str(path: &str, home: &str) -> Self {
        let path_splits = path.split(':');
        let path = path_splits.map(PathBuf::from).collect();

        let home = PathBuf::from(home);

        Config::new(path, home)
    }
}
