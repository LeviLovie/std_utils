use crate::errors::*;
use anyhow::{Error, Result};
use std::path::PathBuf;

pub fn rel_path(path: &str) -> Result<PathBuf, Error> {
    let path_parts: Vec<&str> = path.split('/').collect();
    let mut path: PathBuf = std::env::current_exe()?.parent().anyhow()?.to_path_buf();
    for part in path_parts {
        path.push(part);
    }
    Ok(path)
}

pub fn path(path: PathBuf) -> Result<PathBuf, Error> {
    let exec_dir = std::env::current_exe()?.parent().anyhow()?.to_path_buf();
    let path = exec_dir.join(path);
    Ok(path)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rel_path() {
        let path = rel_path("test").unwrap();
        assert_eq!(
            path,
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("test")
        );
    }

    #[test]
    fn test_path() {
        let path = path(PathBuf::from("test")).unwrap();
        assert_eq!(
            path,
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("test")
        );
    }
}
