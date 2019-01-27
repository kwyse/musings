use failure::Error;

use std::fs::File;
use std::io::Write;
use std::path::Path;

struct FileOnDisk;

impl FileOnDisk {
    fn write(contents: &[u8], path: impl AsRef<Path>) -> Result<(), Error> {
        File::create(path)?
            .write_all(contents)?;

        Ok(())
    }
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
}
