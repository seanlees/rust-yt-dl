use std::{fs, io, path::PathBuf};
use std::path::Path;



pub fn dir_size(path: impl Into<PathBuf>) -> io::Result<u64> {
    fn dir_size(mut dir: fs::ReadDir) -> io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(fs::read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }

    dir_size(fs::read_dir(path.into())?)
}

pub fn get_or_create(path_str: &str) -> Result<String, String> {
    let path = Path::new(path_str);
    if !path.exists() {
        //dir::create_all(path, false);
        let r = fs::create_dir_all(path);
        if !r.is_ok() {
            return Err(r.unwrap_err().to_string());
        }
    }

    Ok(path_str.to_string())
}
