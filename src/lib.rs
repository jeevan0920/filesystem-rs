use std::collections::HashMap;

pub struct FileSystem {
    pub root: HashMap<String, Node>,
}

#[derive(Debug)]
pub enum Node {
    File(String),
    Directory(HashMap<String, Node>),
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            root: HashMap::new(),
        }
    }

    pub fn create_file(&mut self, path: &str, content: &str) {
        let parts: Vec<&str> = path.split("/").collect();
        let mut current = &mut self.root;

        for part in parts[1..(parts.len() - 1)].iter() {
            if current.get(*part).is_none() {
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

    pub fn read_file(&self, path: &str) -> Option<&str> {
        let parts: Vec<&str> = path.split("/").collect();
        let mut current = &self.root;
        for part in parts[1..(parts.len() - 1)].iter() {
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

    pub fn list_files_and_directories(&self, path: &str) -> Vec<&str> {
        let parts: Vec<&str> = path.split("/").collect();
        let mut current: &HashMap<String, Node> = &self.root;
        for part in parts[1..parts.len()].iter() {
            if let Some(Node::Directory(dir)) = current.get(*part) {
                current = dir;
            } else {
                return vec![];
            }
        }

        current.keys().map(|s| s.as_str()).collect::<Vec<&str>>()
    }

    pub fn delete_file(&mut self, path: &str) {
        let parts: Vec<&str> = path.split("/").collect();
        let mut current = &mut self.root;
        for part in parts[1..(parts.len() - 1)].iter() {
            if let Some(Node::Directory(dir)) = current.get_mut(*part) {
                current = dir;
            } else {
                return;
            }
        }

        let filename = parts.last().unwrap();
        current.remove(*filename);
    }

    pub fn rename_file(&mut self, old_path: &str, new_path: &str) {
        self.move_file(old_path, new_path)
    }

    pub fn copy_file(&mut self, path: &str, new_path: &str) {
        if let Some(content) = self.read_file(path) {
            self.create_file(new_path, &content.to_owned())
        }
    }

    fn move_file(&mut self, old_path: &str, new_path: &str) {
        if let Some(content) = self.read_file(old_path) {
            self.create_file(new_path, &content.to_string());
            self.delete_file(old_path);
        }
    }

    pub fn search_file(&self, file_name: &str) -> Vec<String> {
        let path = "";
        let mut res = Vec::new();
        self.search_helper(file_name, &self.root, path, &mut res);
        res
    }

    fn search_helper(
        &self,
        file_name: &str,
        current: &HashMap<String, Node>,
        old_path: &str,
        res: &mut Vec<String>,
    ) {
        for (name, node) in current.iter() {
            let new_path = if old_path.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", old_path, name)
            };
            match node {
                Node::File(_) => {
                    if file_name == name {
                        res.push(new_path);
                    }
                }
                Node::Directory(curr_map) => {
                    self.search_helper(file_name, curr_map, &new_path, res);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_read_file() {
        let mut fs = FileSystem::new();
        fs.create_file("/a.txt", "Hello");
        assert_eq!(fs.read_file("/a.txt"), Some("Hello"));
    }

    #[test]
    fn test_read_non_existent_file() {
        let fs = FileSystem::new();
        assert_eq!(fs.read_file("/b.txt"), None);
    }

    #[test]
    fn test_create_nested_file() {
        let mut fs = FileSystem::new();
        fs.create_file("/dir/a.txt", "Hello");
        assert_eq!(fs.read_file("/dir/a.txt"), Some("Hello"));
    }

    #[test]
    fn test_list_files_and_directories() {
        let mut fs = FileSystem::new();
        fs.create_file("/dir/a.txt", "Hello");
        fs.create_file("/dir/b.txt", "World");
        let entries = fs.list_files_and_directories("/dir");
        assert!(entries.contains(&"a.txt"));
        assert!(entries.contains(&"b.txt"));
    }

    #[test]
    fn test_delete_file() {
        let mut fs = FileSystem::new();
        fs.create_file("/a.txt", "Hello");
        fs.delete_file("/a.txt");
        assert_eq!(fs.read_file("/a.txt"), None);

        fs.create_file("/dir1/dir2/a.txt", "Hello");
        fs.delete_file("/dir1/dir2/a.txt");
        assert_eq!(fs.read_file("/dir1/dir2/a.txt"), None);
    }

    #[test]
    fn test_rename_file() {
        let mut fs = FileSystem::new();
        fs.create_file("/a.txt", "Hello");
        fs.rename_file("/a.txt", "/b.txt");
        assert_eq!(fs.read_file("/a.txt"), None);
        assert_eq!(fs.read_file("/b.txt"), Some("Hello"));
    }

    #[test]
    fn test_copy_file() {
        let mut fs = FileSystem::new();
        fs.create_file("/a.txt", "Hello");
        fs.copy_file("/a.txt", "/b.txt");
        assert_eq!(fs.read_file("/a.txt"), Some("Hello"));
        assert_eq!(fs.read_file("/b.txt"), Some("Hello"));
    }

    #[test]
    fn test_move_file() {
        let mut fs = FileSystem::new();
        fs.create_file("/a.txt", "Hello");
        fs.move_file("/a.txt", "/b.txt");
        assert_eq!(fs.read_file("/a.txt"), None);
        assert_eq!(fs.read_file("/b.txt"), Some("Hello"));
    }

    #[test]
    fn test_search_file() {
        let mut fs = FileSystem::new();
        fs.create_file("/dir/a.txt", "Hello");
        fs.create_file("/dir/subdir/b.txt", "World");
        let search_results = fs.search_file("a.txt");
        assert_eq!(search_results, vec!["dir/a.txt"]);
        let search_results = fs.search_file("b.txt");
        assert_eq!(search_results, vec!["dir/subdir/b.txt"]);
    }
}
