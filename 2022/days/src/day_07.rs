use std::{cell::RefCell, rc::Rc};

use utils::{read_input, Purpose};

use crate::DayChallenge;

trait WithSize {
    fn size(&self) -> u32;
}

struct Directory2 {
    name: String,
    parent: Option<Rc<RefCell<Directory2>>>,
    files: Vec<File>,
    children: Vec<Rc<RefCell<Directory2>>>,
}

impl WithSize for Directory2 {
    fn size(&self) -> u32 {
        self.files.iter().map(|f| f.size()).sum::<u32>()
            + self.children.iter().map(|c| c.borrow().size()).sum::<u32>()
    }
}

struct File {
    size: u32,
    name: String,
    dir: Rc<RefCell<Directory2>>,
}

impl WithSize for File {
    fn size(&self) -> u32 {
        return self.size;
    }
}

struct FileSystem2 {
    root: Rc<RefCell<Directory2>>,
    current_dir: Rc<RefCell<Directory2>>,
}

impl FileSystem2 {
    fn cd(&mut self, new_dir: &str) {
        self.current_dir = match new_dir {
            "/" => Rc::clone(&(self.root)),
            ".." => Rc::clone(&(self.current_dir)).borrow().parent.unwrap(),
            &_ => todo!(),
            /*a => {
                match self
                    .current_dir
                    .borrow()
                    .children
                    .iter()
                    .find(|b| b.borrow().name == a)
                {
                    None => {
                        let dir = Directory2 {
                            name: String::from(a),
                            parent: None, //Some(curdir),
                            files: vec![],
                            children: vec![],
                        };
                        self.current_dir
                            .borrow_mut()
                            .children
                            .push(Rc::new(RefCell::new(dir)));
                        Rc::new(RefCell::new(dir))
                    }
                    Some(&d) => d,
                }
            }*/
        };
    }

    fn ls(&mut self, output: &[String]) {
        for f in output {
            let mut file_info = f.split_whitespace();
            let sz_or_dir = file_info.next().unwrap();
            let fname = file_info.next().unwrap();
            if sz_or_dir == "dir" {
                if self
                    .current_dir
                    .borrow()
                    .children
                    .iter()
                    .find(|b| b.borrow().name == sz_or_dir)
                    .is_none()
                {
                    let dir = Directory2 {
                        name: String::from(fname),
                        parent: Some(Rc::clone(&self.current_dir)),
                        files: vec![],
                        children: vec![],
                    };
                    self.current_dir
                        .borrow_mut()
                        .children
                        .push(Rc::new(RefCell::new(dir)));
                }
            } else {
                let sz: u32 = sz_or_dir.parse::<u32>().unwrap();
                self.add_file(String::from(fname), sz)
            }
        }
    }

    fn add_file(&mut self, name: String, sz: u32) {
        let files: &mut Vec<File> = &mut self.current_dir.borrow_mut().files;
        let exists = files.iter().find(|f| f.name == name);
        if exists.is_none() {
            files.push(File {
                name: name,
                dir: Rc::clone(&self.current_dir),
                size: sz,
            })
        }
    }

    fn new() -> FileSystem2 {
        let root = Rc::new(RefCell::new(Directory2 {
            name: String::from("/"),
            parent: Option::None,
            files: vec![],
            children: vec![],
        }));

        let ref_1 = Rc::clone(&root);

        FileSystem2 {
            root: ref_1,
            current_dir: Rc::clone(&root),
        }
    }
}

pub struct Day07 {
    fs: FileSystem2,
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
        String::from("")
    }

    fn part_2(&mut self) -> String {
        String::from("")
    }
}

fn parse_input(purp: Purpose) -> FileSystem2 {
    let input = read_input(7, purp);
    let mut fs = FileSystem2::new();
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

fn execCommand(cmd_and_output: &[String], fs: &mut FileSystem2) {
    let cmd = &cmd_and_output[0];
    println!("{}", cmd);
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
        assert_eq!(day_07.part_1(), "CMZ");
    }

    #[test]
    fn part_2_works() {
        let mut day_07 = init_test();
        assert_eq!(day_07.part_2(), "MCD");
    }
}
