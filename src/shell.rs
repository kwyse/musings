use failure::Error;

use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
use std::path::{Component, Path};

const TILDE: &str = "~";

pub struct ShellExpander {
    home_dir_var: String
}

impl ShellExpander {
    pub fn new(home_dir_var: &str) -> Self {
        Self {
            home_dir_var: home_dir_var.to_string(),
        }
    }

    pub fn tilde<'a, P>(&self, path: &'a P) -> Result<Cow<'a, Path>, Error>
    where
        P: AsRef<Path> + 'a,
    {
        let mut components = path.as_ref().components();

        match components.next() {
            Some(Component::Normal(first)) if first == OsStr::new(TILDE) => {
                let home_dir = env::var(&self.home_dir_var)?;
                let mut expanded_path = Path::new(&home_dir).to_path_buf();
                expanded_path.push(components.as_path());

                Ok(Cow::Owned(expanded_path))
            },
            _ => Ok(Cow::Borrowed(path.as_ref()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn tildes_at_start_of_path_are_expanded() {
        let expander = ShellExpander::new("TILDE");
        env::set_var("TILDE", "home");
        let path = "~/some/path";

        let expanded_path = expander.tilde(&path).unwrap();

        let expected_path = Path::new("home/some/path").to_path_buf();
        assert_eq!(expanded_path, Cow::from(expected_path));
    }

    #[test]
    fn tildes_not_at_start_of_path_are_ignored() {
        let expander = ShellExpander::new("");
        let path = "some/~/path";

        let expanded_path = expander.tilde(&path).unwrap();

        let expected_path = Path::new("some/~/path");
        assert_eq!(expanded_path, Cow::from(expected_path));
    }
}
