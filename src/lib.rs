use std::collections::HashMap;

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
}
