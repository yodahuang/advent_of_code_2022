use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::fs;

#[derive(Debug)]
enum Node {
    File {
        name: String,
        _size: i64,
    },
    Dir {
        name: String,
        contents: Vec<Node>,
        // This has to be a RefCell so that functions modifying this can still be not mut.
        // Cached size.
        _size: RefCell<Option<i64>>,
    },
}

impl Node {
    fn size(&self) -> i64 {
        match self {
            Node::File { _size, .. } => *_size,
            Node::Dir {
                contents, _size, ..
            } => {
                // Avoiding borrowing the size directly.
                let size_cache: Option<i64> = _size.borrow().to_owned();
                match size_cache {
                    Some(s) => s,
                    None => {
                        *_size.borrow_mut() = Some(contents.iter().map(|n| n.size()).sum());
                        return (*_size.borrow()).unwrap();
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    // We can make it a key-value store, but for fun let's pretend there's inode and all the posix stuff.
    tree: Vec<Node>,
    // Store the index instead of a bunch of Rcs. Faster.
    current_dir_idxs: Vec<usize>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            tree: vec![Node::Dir {
                name: String::from("/"),
                contents: Vec::new(),
                _size: RefCell::new(None),
            }],
            current_dir_idxs: vec![0],
        }
    }

    fn current_dir_mut(&mut self) -> &mut Node {
        assert!(!self.current_dir_idxs.is_empty());
        let mut current_dir: &mut Node = &mut self.tree[self.current_dir_idxs[0]];
        for idx in &self.current_dir_idxs[1..] {
            current_dir = match current_dir {
                Node::Dir { contents, .. } => &mut contents[*idx],
                Node::File { .. } => panic!("The idx somehow point to a file."),
            };
        }
        current_dir
    }

    // This seems wrong. In C++ we've got const_cast but now here.
    // https://stackoverflow.com/questions/41436525/how-to-avoid-writing-duplicate-accessor-functions-for-mutable-and-immutable-refe
    fn current_dir(&self) -> &Node {
        assert!(!self.current_dir_idxs.is_empty());
        let mut current_dir: &Node = &self.tree[self.current_dir_idxs[0]];
        for idx in &self.current_dir_idxs[1..] {
            current_dir = match current_dir {
                Node::Dir { contents, .. } => &contents[*idx],
                Node::File { .. } => panic!("The idx somehow point to a file."),
            };
        }
        current_dir
    }

    fn cd(&mut self, destination: &str) {
        let current_dir = self.current_dir();
        match destination {
            ".." => {
                self.current_dir_idxs.pop();
            }
            "/" => self.current_dir_idxs = vec![0],
            dir_name => {
                self.current_dir_idxs.push(match current_dir {
                    Node::Dir { name, contents, .. } => {
                        contents.iter().position(|node| {match node {
                            Node::File { .. } => false,
                            Node::Dir { name, .. } => name == dir_name,
                        }}).unwrap_or_else(|| panic!("Trying to cd into non exist directory {} , currently in {} with contents {:?}", dir_name, name, contents))
                    },
                    Node::File { .. } => panic!("Current dir is a file. Something's seriously wrong."),
                });
            }
        }
    }

    fn discover(&mut self, mut items: Vec<Node>) {
        if let Node::Dir { contents, .. } = self.current_dir_mut() {
            contents.append(&mut items);
        }
    }

    // Implementing tree traversal requires recording the location of the stack. Surprisingly hard.
    // This is so easy in Python!
    fn visit(&self, mut f: impl FnMut(&Node)) {
        fn visit_node(node: &Node, f: &mut impl FnMut(&Node)) {
            f(node);
            match node {
                Node::Dir { contents, .. } => {
                    for child in contents {
                        visit_node(child, f);
                    }
                }
                Node::File { .. } => {}
            };
        }
        for node in &self.tree {
            visit_node(node, &mut f);
        }
    }
}

fn parse_ls(output: &[&str]) -> Vec<Node> {
    output
        .iter()
        .map(|line| {
            lazy_static! {
                static ref DIR_RE: Regex = Regex::new(r"^dir (\w+)$").unwrap();
                static ref FILE_RE: Regex = Regex::new(r"^(\d+) ([\w.]+)$").unwrap();
            }
            match DIR_RE.captures(line) {
                Some(caps) => Node::Dir {
                    name: caps.get(1).unwrap().as_str().to_string(),
                    contents: Vec::new(),
                    _size: RefCell::new(None),
                },
                None => {
                    let caps = FILE_RE
                        .captures(line)
                        .unwrap_or_else(|| panic!("The line does not look like a file: {}", line));
                    Node::File {
                        name: caps.get(2).unwrap().as_str().to_string(),
                        _size: caps.get(1).unwrap().as_str().parse().unwrap_or_else(|_| {
                            panic!("The sized cannot be parsed as int: {}", line)
                        }),
                    }
                }
            }
        })
        .collect()
}

fn build_file_system(contents: &str) -> FileSystem {
    let lines: Vec<&str> = contents.lines().collect();
    let mut file_system = FileSystem::new();
    let mut current_ls_output_start: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("$ cd") {
            // This cannot be written as a closure because we'll need a mutable capture of start there.
            if let Some(start) = current_ls_output_start {
                file_system.discover(parse_ls(&lines[start..i]));
                current_ls_output_start = None;
            }
            file_system.cd(line.trim_start_matches("$ cd "));
        } else if *line == "$ ls" {
            if let Some(start) = current_ls_output_start {
                file_system.discover(parse_ls(&lines[start..i]));
            }
            current_ls_output_start = Some(i + 1);
        }
    }

    if let Some(start) = current_ls_output_start {
        file_system.discover(parse_ls(&lines[start..lines.len()]));
    }
    file_system
}

fn main() {
    let contents = fs::read_to_string("inputs/input").unwrap();
    let file_system = build_file_system(&contents);

    let mut problem1_sum: i64 = 0;
    file_system.visit(|node: &Node| {
        if let Node::Dir { .. } = node {
            if node.size() < 100000 {
                problem1_sum += node.size();
            }
        }
    });
    println!("Problem 1: {}", problem1_sum);

    let current_size = file_system.tree[0].size();
    let needed_size = (30000000 - (70000000 - current_size)).max(0);
    let mut candidate_dir_sizes: Vec<i64> = vec![];
    file_system.visit(|node: &Node| {
        if let Node::Dir { .. } = node {
            if node.size() >= needed_size {
                candidate_dir_sizes.push(node.size());
            }
        }
    });
    println!("Problem 2: {}", candidate_dir_sizes.iter().min().unwrap());
}
