use std::{collections::HashMap, hash::Hash, io::Cursor, thread::current};

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
        let parts: Vec<&str> = path.split("/").collect();
        let mut current = &self.root;
        for part in parts[0..(parts.len() - 1)].iter() {
            if let Some(Node::Directory(dir)) = current.get(*part) {
                current = dir;
            } else {
                return None;
            }
        }

        let file_name = parts.last().unwrap();
        if let Some(Node::File(content)) = current.get(*file_name) {
            Some(content)
        } else {
            None
        }
    }

    fn list_files_and_directories(&self, path: &str) -> Vec<&str> {
        let parts: Vec<&str> = path.split("/").collect();
        let mut current: &HashMap<String, Node> = &self.root;
        for part in parts[0..(parts.len() - 1)].iter() {
            if let Some(Node::Directory(dir)) = current.get(*part) {
                current = dir;
            } else {
                return vec![];
            }
        }

        current.keys().map(|s| s.as_str()).collect::<Vec<&str>>()
    }

    fn rename_file(&mut self, old_path: &str, new_path: &str) {
        todo!()
    }

    fn delete_file(&mut self, path: &str) {
        todo!()
    }
}
