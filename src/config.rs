use crate::shell::{ShellExpander, UnixExpander};

use failure::Error;
use serde::{Deserialize, Deserializer, Serialize};

use std::borrow::Cow;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

mod defaults {
    use std::path::{Path, PathBuf};

    pub fn data_dir() -> PathBuf { Path::new("~/.local/share/muse").to_path_buf() }
    pub fn weight_csv_file() -> PathBuf { Path::new("weight.csv").to_path_buf() }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "defaults::data_dir")]
    data_dir: PathBuf,

    #[serde(default = "defaults::weight_csv_file")]
    weight_csv_file: PathBuf,
}

impl Config {
    pub fn load(reader: impl Read) -> Result<Self, Error> {
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn expand_paths(mut self, expander: &impl ShellExpander) -> Result<Self, Error> {
        if let Cow::Owned(path) = expander.expand(&self.data_dir)? {
            self.data_dir = path;
        }

        if let Cow::Owned(path) = expander.expand(&self.weight_csv_file)? {
            self.weight_csv_file = path;
        }

        Ok(self)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: defaults::data_dir(),
            weight_csv_file: defaults::weight_csv_file(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn config_is_loaded_from_reader() {
        let reader = BufReader::new("data_dir: some/dir".as_bytes());

        let config = Config::load(reader).unwrap();

        assert_eq!(config.data_dir, Path::new("some/dir").to_path_buf());
    }

    #[test]
    fn config_paths_are_left_unaltered_if_borrowed_cow_is_returned_when_expanding() {
        let reader = BufReader::new("data_dir: some/dir".as_bytes());
        struct StubExpander;
        impl ShellExpander for StubExpander {
            fn expand<'a, P>(&self, path: &'a P) -> Result<Cow<'a, Path>, Error>
            where P: AsRef<Path> + 'a {
                Ok(Cow::Borrowed(Path::new("new/path")))
            }
        }

        let config = Config::load(reader).unwrap()
            .expand_paths(&StubExpander {}).unwrap();

        assert_eq!(config.data_dir, Path::new("some/dir").to_path_buf());
    }

    #[test]
    fn config_paths_are_updated_if_owned_cow_is_returned_when_expanding() {
        let reader = BufReader::new("data_dir: some/dir".as_bytes());
        struct StubExpander;
        impl ShellExpander for StubExpander {
            fn expand<'a, P>(&self, path: &'a P) -> Result<Cow<'a, Path>, Error>
            where P: AsRef<Path> + 'a {
                Ok(Cow::Owned(Path::new("new/path").to_path_buf()))
            }
        }

        let config = Config::load(reader).unwrap()
            .expand_paths(&StubExpander {}).unwrap();

        assert_eq!(config.data_dir, Path::new("new/path").to_path_buf());
    }
}
