use failure::Error;
use std::env;
use std::path::PathBuf;

pub fn root_dir() -> Result<PathBuf, Error> {
    let p = env::var("CARGO_MANIFEST_DIR")
        .map(From::from)
        .or_else(|_| env::current_dir())?;
    Ok(p.parent().unwrap().to_path_buf())
}

pub fn corpora_dir() -> Result<PathBuf, Error> {
    let p = root_dir()?.join("corpora");
    Ok(p)
}
