use std::{collections::HashMap, hash::Hash};

struct FileSystem {
    root: HashMap<String, Node>,
}

enum Node {
    File(String),
    Directory(HashMap<String, Node>),
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            root: HashMap::new(),
        }
    }

    fn create_file(&mut self, path: &str, content: &str) {
        let parts: Vec<&str> = path.split("/").collect();
        let mut current = &mut self.root;

        for part in parts[0..(parts.len() - 1)].iter() {
            if !current.get(*part).is_none() {
                current.insert(part.to_string(), Node::Directory(HashMap::new()));
            }

            if let Some(Node::Directory(dir)) = current.get_mut(*part) {
                current = dir;
            } else {
                unreachable!()
            }
        }

        let filename = parts.last().unwrap();
        current.insert(filename.to_string(), Node::File(content.to_string()));
    }

    fn read_file(&self, path: &str) -> Option<&str> {
        todo!()
    }

    fn list_files_and_directories(&self) -> Vec<&str> {
        todo!()
    }

    fn rename_file(&mut self, old_path: &str, new_path: &str) {
        todo!()
    }

    fn delete_file(&mut self, path: &str) {
        todo!()
    }
}
