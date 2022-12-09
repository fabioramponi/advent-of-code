use utils::{read_input, Purpose};

use utils::arena_tree::ArenaTree;

use crate::DayChallenge;

trait WithSize {
    fn size(&self) -> u32;
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
}

impl Default for Directory {
    fn default() -> Self {
        Directory { name: String::from("/"), files: vec![] }
    }
}
impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Directory {
    fn with_name(dir_name: String) -> Directory {
        Directory{name: dir_name, files: vec![]}
    }
}


#[derive(Debug)]
struct File {
    size: u32,
    name: String,
    dir: usize,
}

impl WithSize for File {
    fn size(&self) -> u32 {
        return self.size;
    }
}

#[derive(Debug)]
struct FileSystem {
    tree :ArenaTree<Directory>,
    current_dir: usize
}

impl FileSystem {

    fn cd(&mut self, new_dir: &str) {
        self.current_dir = match new_dir {
            "/" => self.tree.node(Directory{name: String::from("/"), files: vec![]}),
            ".." => self.tree.arena[self.current_dir].parent.unwrap(),
            a => {
                let dir_full_name = format!("{}/{}",self.tree.arena[self.current_dir].val.name, a); 
                self.tree.insert_or_get(Directory::with_name(dir_full_name), self.current_dir)
            }
    };}


    fn ls(&mut self, output: &[String]) {
        for f in output {
            let mut file_info = f.split_whitespace();
            let sz_or_dir = file_info.next().unwrap();
            let fname = file_info.next().unwrap();
            if sz_or_dir == "dir" {
                let dir_full_name = format!("{}/{}", self.tree.arena[self.current_dir].val.name, fname);
                let _ = self.tree.insert_or_get(Directory::with_name(dir_full_name), self.current_dir);
            } else {
                let sz: u32 = sz_or_dir.parse::<u32>().unwrap();
                self.add_file(String::from(fname), sz)
            }
        }
    }

    fn add_file(&mut self, name: String, sz: u32) {
        let files: &mut Vec<File> = &mut self.tree.arena[self.current_dir].val.files;
        let exists = files.iter().find(|f| f.name == name);
        if exists.is_none() {
            files.push(File {
                name: name,
                dir: self.current_dir,
                size: sz,
            })
        }
    }

    fn size(&self, idx: usize) -> u32 {
        let directory = &self.tree.arena[idx];
        let size = directory.val.files.iter().fold(0,|acc, f| acc + f.size);
        directory.children.iter().fold(size, |acc, &c| acc +  self.size(c))
    }

    fn new() -> FileSystem {

        FileSystem {
            tree: ArenaTree::default(),
            current_dir: 0,
        }
    }
}

pub struct Day07 {
    fs: FileSystem,
}

impl Day07 {
    pub fn init(purp: Purpose) -> Self {
        Day07 {
            fs: parse_input(purp),
        }
    }
}

impl DayChallenge for Day07 {
    fn part_1(&mut self) -> String {
        let sizes = self.fs.tree.arena.iter().map(|d| self.fs.size(d.idx)).collect::<Vec<u32>>();
        let res = sizes.iter().filter(|&&sz| sz < 100000).sum::<u32>();
        res.to_string()
    }

    fn part_2(&mut self) -> String {
        let total_disk_space :u32 = 70000000;
        let min_unused_space :u32 = 30000000;
        let total_size = self.fs.size(0);
        let mut sizes = self.fs.tree.arena.iter().map(|d| self.fs.size(d.idx)).collect::<Vec<u32>>();
        sizes.sort();
        sizes.iter().find(|&&sz| (total_disk_space-(total_size-sz)) >= min_unused_space).unwrap().to_string()
    }
}

fn parse_input(purp: Purpose) -> FileSystem {
    let input = read_input(7, purp);
    let mut fs = FileSystem::new();
    let mut idx: usize = 0;
    while idx < input.len() {
        let line = &input[idx];
        if line.starts_with("$") {
            let cmd_start_idx = idx;
            idx += 1;
            while idx < input.len() && !input[idx].starts_with("$") {
                idx += 1;
            }
            execCommand(&input[cmd_start_idx..idx], &mut fs);
        }
    }
    fs
}

fn execCommand(cmd_and_output: &[String], fs: &mut FileSystem) {
    let cmd = &cmd_and_output[0];
    if (&cmd[2..]).starts_with("cd") {
        fs.cd(&cmd[5..]);
    } else {
        fs.ls(&cmd_and_output[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day07 {
        Day07::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_07 = init_test();
        assert_eq!(day_07.part_1(), "95437");
    }

    #[test]
    fn part_2_works() {
        let mut day_07 = init_test();
        assert_eq!(day_07.part_2(), "24933642");
    }
}
