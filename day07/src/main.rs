use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::rc::Rc;

use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// Input
    #[arg(value_name = "FILE")]
    #[arg(default_value = "data/sample.txt")]
    input: PathBuf,
}

enum Entry {
    Dir(Rc<RefCell<Directory>>),
    File { name: String, size: usize },
}

impl Entry {
    fn name(&self) -> String {
        match self {
            Entry::Dir(dir) => dir.borrow().name.clone(),
            Entry::File { name, .. } => name.clone(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Entry::Dir(dir) => dir.borrow().size(),
            Entry::File { size, .. } => *size,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Dir(dir) => write!(
                f,
                "Dir({:?}: [{}])",
                dir.borrow().name,
                dir.borrow()
                    .children
                    .values()
                    .map(|e| e.to_string())
                    .join(", ")
            ),
            Entry::File { name, size } => write!(f, "File({:?}: {})", name, size),
        }
    }
}

struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    children: HashMap<String, Entry>, // {name: entry}
}

impl Directory {
    fn new(name: impl Into<String>, parent: Option<Rc<RefCell<Directory>>>) -> Self {
        Self {
            name: name.into(),
            parent,
            children: HashMap::new(),
        }
    }

    fn add_child(&mut self, child: Entry) {
        self.children.insert(child.name(), child);
    }

    fn size(&self) -> usize {
        self.children.values().map(|e| e.size()).sum()
    }

    fn dir_sizes(&self) -> Vec<usize> {
        let mut sizes = Vec::new();
        sizes.push(self.size());
        for child in self.children.values() {
            match child {
                Entry::Dir(dir) => {
                    sizes.extend(dir.borrow().dir_sizes());
                }
                Entry::File {..}=>{}
            }
        }
        sizes
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Directory(name = {:?}, parent = {:?}, children = [{}])",
            self.name,
            self.parent.as_ref().map(|p| p.borrow().name.clone()),
            self.children.values().map(|e| e.to_string()).join(", ")
        )
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let lines = BufReader::new(File::open(args.input)?)
        .lines()
        .filter_map(|x| x.ok())
        .collect_vec();

    let mut root = Rc::new(RefCell::new(Directory::new("/", None)));
    let mut cwd = Rc::clone(&root);
    for line in lines {
        match line.as_str() {
            "$ cd /" => {
                cwd = Rc::clone(&root);
            }
            "$ cd .." => {
                let parent = Rc::clone(cwd.borrow().parent.as_ref().unwrap());
                cwd = parent;
            }
            "$ ls" => {}
            _ if line.starts_with("$ cd ") => {
                let name = line[5..].to_string();
                let child = match &cwd.borrow().children[&name] {
                    Entry::Dir(dir) => Rc::clone(dir),
                    Entry::File { .. } => panic!("Cannot 'cd' to a file"),
                };
                cwd = child;
            }
            _ if line.starts_with("dir ") => {
                let name = line[4..].to_string();
                let dir = Rc::new(RefCell::new(Directory::new(name, Some(Rc::clone(&cwd)))));
                let entry = Entry::Dir(dir);
                cwd.borrow_mut().add_child(entry);
            }
            _ => {
                let parts = line.split_whitespace().collect_vec();
                assert_eq!(parts.len(), 2, "parts: {:?}", parts);
                let size = parts[0].parse::<usize>().unwrap();
                let name = parts[1].to_string();
                let entry = Entry::File { name, size };
                cwd.borrow_mut().add_child(entry);
            }
        }
    }
    println!("root = {}", root.borrow());

    let dir_sizes = root.borrow().dir_sizes();

    println!("==> Solving part one...");
    let ans1: usize = dir_sizes.iter().filter(|&&s| s <= 100000).sum();
    println!("Total size under 100000: {}", ans1);

    println!("==> Solving part two...");
    let need_to_free = dir_sizes[0] - 40000000;
    let ans2 = *dir_sizes.iter().filter(|&&s| s >= need_to_free).min().unwrap();
    println!("Total size of removed dir: {}", ans2);

    Ok(())
}
