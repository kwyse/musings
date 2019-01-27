use failure::{Fail, Error};

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

struct FileOnDisk;

impl FileOnDisk {
    fn write(contents: &[u8], path: impl AsRef<Path>) -> Result<(), Error> {
        create_parent_dirs_if_needed(&path);
        File::create(path)?.write_all(contents)?;

        Ok(())
    }
}

#[derive(Debug, Fail)]
enum WriteError {
    #[fail(display = "parent directory does not exist for path: {}", path)]
    ParentDoesNotExist {
        path: String,
    }
}

fn create_parent_dirs_if_needed(path: impl AsRef<Path>) -> Result<(), Error> {
    let displayable_path = || path.as_ref().to_string_lossy().to_string();
    let mk_err = || WriteError::ParentDoesNotExist { path: displayable_path() };

    let parent_dir = path.as_ref().parent().ok_or(mk_err())?;

    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::io::Read;

    #[test]
    fn contents_is_written_to_file_on_disk() {
        let contents = "file contents";
        let mut path = env::temp_dir();
        path.push("file.txt");

        FileOnDisk::write(contents.as_bytes(), &path).unwrap();

        let mut written_contents = String::new();
        let mut written_file = File::open(path).unwrap();
        written_file.read_to_string(&mut written_contents);
        assert_eq!(written_contents, contents);
    }

    #[test]
    fn parent_dirs_are_created_if_they_do_not_exist() {
        let mut path = env::temp_dir();
        path.push("dir");
        path.push("file.txt");

        FileOnDisk::write(&[0_u8], &path).unwrap();

        assert!(File::open(path).is_ok());
    }
}
