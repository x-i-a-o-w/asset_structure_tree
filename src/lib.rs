use std::fs;
use std::path::{Path, PathBuf};

type Dir = fs::ReadDir;

type AssetEntry = AssetStructure;

pub struct AssetStructure {
    local: AssetBranch,
    children: Vec<AssetEntry>,
}

impl AssetStructure {
    pub fn new<P: AsRef<Path>>(path: P) -> AssetStructure {
        AssetStructure {
            local: AssetBranch::new(path),
            children: Vec::new(),
        }
    }
    pub fn get_local<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, Error> {
        self.local.get(path)
    }
    pub fn get_global<P: AsRef<Path>>(&self, path: P) -> Result<Vec<PathBuf>, Error> {
        let mut matches = Vec::new();
        if let Ok(r) = self.local.get(path) {
            matches.push(r);
        }
        for i in &self.children {
            self.get_global_recursive(i.get_path(), &mut matches)?;
        }
        Ok(matches)
    }
    fn get_global_recursive<P: AsRef<Path>>(
        &self,
        path: P,
        buffer: &mut Vec<PathBuf>,
    ) -> Result<(), Error> {
        if let Ok(r) = self.local.get(path) {
            buffer.push(r);
        }
        for i in &self.children {
            self.get_global_recursive(i.get_path(), buffer)?;
        }
        Ok(())
    }
}

impl Asset for AssetStructure {
    fn get_path(&self) -> &Path {
        self.local.get_path()
    }
    fn is_alive(&self) -> bool {
        self.local.is_alive()
    }
}

impl AsRef<Path> for AssetStructure {
    fn as_ref(&self) -> &Path {
        self.local.as_ref()
    }
}

pub struct AssetBranch {
    path: Box<Path>,
    dir: Option<Dir>,
    alive: bool,
}
impl AssetBranch {
    pub fn new<P: AsRef<Path>>(path: P) -> AssetBranch {
        let dir = if let Ok(r) = fs::read_dir(&path) {
            Some(r)
        } else {
            None
        };
        AssetBranch {
            path: Box::from(path.as_ref()),
            dir,
            alive: path.as_ref().exists(),
        }
    }
    pub fn get<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, Error> {
        if !self.is_alive() {
            return Err(Error::new(
                ErrorKind::NotAlive,
                &format!("This branch is not alive: {:?}", self.path),
            ));
        }
        let get_path = self.path.join(path);
        if !get_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                &format!("This branch was not found: {:?}", get_path),
            ));
        }
        Ok(get_path)
    }
}

impl AsRef<Path> for AssetBranch {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl Asset for AssetBranch {
    fn get_path(&self) -> &Path {
        &self.path
    }
    fn is_alive(&self) -> bool {
        self.alive
    }
}
#[derive(Debug)]
pub struct Error {
    msg: Box<str>,
    kind: ErrorKind,
}
impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Error {
        Error {
            kind,
            msg: Box::from(msg),
        }
    }
}
#[derive(Debug)]
pub enum ErrorKind {
    NotFound,
    NotAlive,
}

pub trait Asset {
    fn get_path(&self) -> &Path;
    fn is_alive(&self) -> bool;
}
