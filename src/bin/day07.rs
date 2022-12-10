//! # Day 7: No Space Left On Device
//!
//! In this one I decided to use some external crates that I've been curious about. The crates are:
//! slotmap for acting like a virtual memory system of sorts, and smallvec/smallstr for the sake of
//! keeping more data on the stack and reducing indirection when I expect to have many small strings
//! and lists.
//!
//! I may come back and clean this code up a bit later if I have the time, but, this is enough to
//! demonstrate the concept of circular references and other such things that are hard to do in
//! safe rust "natively" but not too hard to do with a data oriented approach.
//!
use aoc::Parser;

mod fs {
    use std::path::Path;

    use slotmap::{DefaultKey, SlotMap};
    use smallstr::SmallString;
    use smallvec::SmallVec;

    pub type Result<T> = std::result::Result<T, Error>;

    #[allow(clippy::enum_variant_names)]
    #[derive(Debug)]
    pub enum Error {
        NotFound,
        NotFoundDest,
        NotADirectory,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::NotFound => write!(f, "NotFound"),
                Self::NotFoundDest => write!(f, "NotFoundDest"),
                Self::NotADirectory => write!(f, "NotADirectory"),
            }
        }
    }

    impl std::error::Error for Error {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Handle(DefaultKey);

    #[derive(Debug)]
    pub struct INode {
        parent_handle: Option<Handle>,
        handle: Handle,
        name: SmallString<[u8; 14]>,
        size: u64,
        children: Option<SmallVec<[Handle; 10]>>,
    }

    impl INode {
        pub fn is_dir(&self) -> bool {
            self.children.is_some()
        }

        pub fn size(&self) -> u64 {
            self.size
        }
    }

    impl std::fmt::Display for INode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.children.is_none() {
                write!(f, "{} {}", self.size, self.name)
            } else {
                write!(f, "dir {}", self.name)
            }
        }
    }

    pub struct Fs {
        data: SlotMap<DefaultKey, INode>,
        root: Handle,
    }

    impl Fs {
        pub fn new() -> Self {
            let mut fs = Self {
                data: SlotMap::with_key(),
                root: Handle(DefaultKey::default()),
            };
            fs.root = fs.create_dir("/").unwrap();
            fs
        }

        pub fn root(&self) -> Handle {
            self.root
        }

        pub fn create_file(&mut self, name: &str, size: u64) -> Result<Handle> {
            Ok(Handle(self.data.insert_with_key(|key| INode {
                parent_handle: None,
                handle: Handle(key),
                name: name.into(),
                size,
                children: None,
            })))
        }

        pub fn create_dir(&mut self, name: &str) -> Result<Handle> {
            Ok(Handle(self.data.insert_with_key(|key| INode {
                parent_handle: None,
                handle: Handle(key),
                name: name.into(),
                size: 0,
                children: Some(SmallVec::new()),
            })))
        }

        pub fn move_to(&mut self, handle: Handle, parent: Handle) -> Result<()> {
            // Get any old parent ahead of time
            let old_parent = {
                self.data
                    .get_mut(handle.0)
                    .ok_or(Error::NotFound)?
                    .parent_handle
            };

            // Establish the new parent handle is real and get it
            let new_parent = {
                let parent = self.data.get_mut(parent.0).ok_or(Error::NotFoundDest)?;
                if !parent.is_dir() {
                    return Err(Error::NotADirectory);
                }
                parent.handle
            };

            // Update the parent handle on the inode being moved
            {
                let node = self.data.get_mut(handle.0).ok_or(Error::NotFound)?;
                node.parent_handle = Some(new_parent);
            }

            // Remove the inode from the old parent's children
            if let Some(old_parent) = old_parent {
                let old_parent_node = self.data.get_mut(old_parent.0);
                if let Some(old_parent_node) = old_parent_node {
                    if let Some(children) = old_parent_node.children.as_mut() {
                        children.retain(|h| h.0 != handle.0);
                    }
                }
            }

            // Add the inode to the new parent's children
            let children = self
                .data
                .get_mut(new_parent.0)
                .ok_or(Error::NotFoundDest)?
                .children
                .as_mut()
                .expect("By all accounts this was a directory, and yet...");

            // Obviously could be more efficient with a map
            if !children.contains(&handle) {
                children.push(handle);
            }

            Ok(())
        }

        pub fn get(&self, handle: Handle) -> Option<&INode> {
            self.data.get(handle.0)
        }

        pub fn get_child(&self, handle: Handle, child: &str) -> Result<Handle> {
            let inode = self.data.get(handle.0).ok_or(Error::NotFound)?;
            if !inode.is_dir() {
                return Err(Error::NotADirectory);
            }
            if let Some(children) = inode.children.as_ref() {
                for child_handle in children {
                    let child_node = self.data.get(child_handle.0).ok_or(Error::NotFound)?;
                    if child_node.name == child {
                        return Ok(child_node.handle);
                    }
                }
            }
            Err(Error::NotFound)
        }

        pub fn cd<P: AsRef<Path>>(&self, cwd: Handle, path: P) -> Result<Handle> {
            let mut handle = cwd;
            for component in path.as_ref().components() {
                match component {
                    std::path::Component::RootDir => {
                        handle = self.root;
                    }
                    std::path::Component::Normal(name) => {
                        let name = name.to_str().ok_or(Error::NotFound)?;
                        let inode = self.data.get(handle.0).ok_or(Error::NotFound)?;
                        if let Some(children) = inode.children.as_ref() {
                            for child in children {
                                let child = self.data.get(child.0).ok_or(Error::NotFound)?;
                                if child.name == name {
                                    handle = child.handle;
                                    break;
                                }
                            }
                        }
                    }
                    std::path::Component::ParentDir => {
                        let inode = self.data.get(handle.0).ok_or(Error::NotFound)?;
                        if let Some(parent) = inode.parent_handle {
                            handle = parent;
                        }
                    }
                    _ => {}
                }
            }
            Ok(handle)
        }

        pub fn ls(&self, handle: Handle) -> Result<SmallVec<[Handle; 10]>> {
            let inode = self.data.get(handle.0).ok_or(Error::NotFound)?;
            if let Some(children) = inode.children.as_ref() {
                Ok(children.clone())
            } else {
                Err(Error::NotADirectory)
            }
        }
    }
}

const DIR_PREFIX: &str = "dir ";
const CD_PREFIX: &str = "$ cd ";
const TOTAL_SPACE: u64 = 70000000;
const MIN_FREE_SPACE: u64 = 30000000;

fn walk_n_count(
    fs: &fs::Fs,
    handle: fs::Handle,
    cache: &mut std::collections::HashMap<fs::Handle, u64>,
) -> u64 {
    let inode = fs.get(handle).unwrap();
    if let Ok(children) = fs.ls(handle) {
        let mut count = 0;
        for child in children {
            count += walk_n_count(fs, child, cache);
        }
        cache.insert(handle, count);
        count
    } else {
        inode.size()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = aoc::Args::parse();
    let path = if cli.real {
        "input/07/real.txt"
    } else {
        "input/07/example.txt"
    };

    let mut fs = fs::Fs::new();
    let input = std::fs::read_to_string(path)?;
    let mut cwd = fs.root();
    for line in input.lines().filter(|line| !line.is_empty()) {
        if line.starts_with(CD_PREFIX) {
            let path = line.strip_prefix(CD_PREFIX).unwrap();
            if path == "/" {
                cwd = fs.root();
            } else if path == ".." {
                cwd = fs.cd(cwd, "..")?;
            } else {
                let child = fs.get_child(cwd, path);
                cwd = match child {
                    Ok(new_cwd) => new_cwd,
                    Err(fs::Error::NotFound) => {
                        let new_dir = fs.create_dir(path)?;
                        fs.move_to(new_dir, cwd)?;
                        new_dir
                    }
                    err @ Err(_) => err?,
                };
            }
        } else if line.starts_with(DIR_PREFIX) {
            let path = line.strip_prefix(DIR_PREFIX).unwrap();
            let new_dir = fs.create_dir(path)?;
            fs.move_to(new_dir, cwd)?;
        } else if line
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            let (size, name) = line.split_once(' ').unwrap();
            let new_file = fs.create_file(name, size.parse()?)?;
            fs.move_to(new_file, cwd)?;
        }
    }

    let mut cache = std::collections::HashMap::new();
    walk_n_count(&fs, fs.root(), &mut cache);

    if !cli.part_two {
        let total: u64 = cache
            .iter()
            .filter_map(|(_, size)| if *size <= 100000 { Some(*size) } else { None })
            .sum();
        println!("{total}");
    } else {
        let free_space = TOTAL_SPACE - cache[&fs.root()];
        let needed_space = MIN_FREE_SPACE - free_space;
        let mut candidates = cache
            .iter()
            .filter(|(_, &size)| size >= needed_space)
            .collect::<Vec<_>>();
        candidates.sort_by_key(|(_, &size)| size);
        let (_, size) = candidates[0];
        println!("{size}");
    }
    Ok(())
}
