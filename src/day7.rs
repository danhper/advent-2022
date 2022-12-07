use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::utils::{self, Day};

#[derive(Debug)]
struct Folder {
    path: PathBuf,
    children: HashMap<String, Node>,
}

impl Folder {
    fn new(path: PathBuf) -> Folder {
        Folder {
            path,
            children: HashMap::new(),
        }
    }

    fn from(lines: Vec<String>) -> Folder {
        let mut folder = Folder::new(PathBuf::from("/"));
        folder.construct(lines);
        folder
    }

    fn construct(&mut self, lines: Vec<String>) -> Vec<String> {
        if lines.is_empty() {
            return lines;
        }
        let (line, rest_) = lines.split_first().unwrap();
        let mut rest = rest_.to_vec();

        if line.starts_with("$ cd") {
            let arg = &line[5..];
            if arg == ".." {
                return rest; // to parent dir
            }
            if let Some(folder) = self.children.get_mut(arg).map(|n| n.as_folder_mut()) {
                rest = folder.construct(rest);
            } else {
                let mut folder = Folder::new(self.path.join(arg));
                rest = folder.construct(rest);
                self.children.insert(arg.to_string(), Node::Folder(folder));
            }
        } else if line.starts_with("$ ls") {
            // nothing to do
        } else if line.starts_with("dir") {
            let dir_name = &line[4..];
            if !self.children.contains_key(dir_name) {
                self.children.insert(
                    dir_name.to_string(),
                    Node::Folder(Folder::new(self.path.join(dir_name))),
                );
            }
        } else {
            let (size, file) = utils::split2::<u64, String>(line, " ");
            self.children
                .insert(file.to_string(), Node::File(file, size));
        }

        self.construct(rest)
    }

    fn compute_sizes(&self) -> (u64, HashMap<PathBuf, u64>) {
        let mut size = 0;
        let mut sizes = HashMap::new();
        for node in self.children.values() {
            match node {
                Node::Folder(f) => {
                    let (node_size, node_sizes) = f.compute_sizes();
                    size += node_size;
                    sizes.extend(node_sizes);
                }
                Node::File(_, s) => size += s,
            }
        }
        sizes.insert(self.path.clone(), size);
        (size, sizes)
    }
}

#[derive(Debug)]
enum Node {
    Folder(Folder),
    File(String, u64),
}

impl Node {
    fn as_folder_mut(&mut self) -> &mut Folder {
        match self {
            Node::Folder(f) => f,
            _ => panic!("Not a folder"),
        }
    }
}

pub struct Day7 {
    root_size: u64,
    folder_sizes: HashMap<PathBuf, u64>,
}

impl Day7 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let lines = utils::read_lines(filepath);
        let root = Folder::from(lines[1..].to_vec());
        let (root_size, folder_sizes) = root.compute_sizes();
        Box::new(Day7 {
            root_size,
            folder_sizes,
        })
    }
}

impl Day for Day7 {
    fn solve_a(&self) -> u64 {
        self.folder_sizes.values().filter(|s| **s < 100_000).sum()
    }

    fn solve_b(&self) -> u64 {
        let unused_space = 70_000_000 - self.root_size;
        let to_free = 30_000_000 - unused_space;
        *self
            .folder_sizes
            .values()
            .filter(|s| **s >= to_free)
            .min()
            .unwrap()
    }
}
