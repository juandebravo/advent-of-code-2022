use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: u32,
}

impl File {
    pub fn new(name: &str, size: u32) -> Self {
        File {
            name: name.to_string(),
            size,
        }
    }
}

#[test]
fn test_can_create_file() {
    let file = File::new("foo", 10);
    assert_eq!(file.name, "foo");
    assert_eq!(file.size, 10);
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    parent: Option<Rc<RefCell<Folder>>>,
    pub folders: HashMap<String, Rc<RefCell<Folder>>>,
    pub files: Vec<File>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            parent: None,
            folders: HashMap::new(),
            files: Vec::new(),
        }
    }

    pub fn size(&self) -> u32 {
        let mut total_size = 0;
        for file in &self.files {
            total_size += file.size;
        }
        for (_name, folder) in &self.folders {
            let f = folder.try_borrow().unwrap();
            total_size += f.size();
        }
        total_size
    }

    pub fn add_file(&mut self, file: File) -> () {
        self.files.push(file);
    }

    pub fn add_folder(&mut self, name: String, folder: Rc<RefCell<Folder>>) -> () {
        self.folders.insert(name, folder);
    }

    pub fn path(&self) -> String {
        let path = self.name.clone();
        match &self.parent {
            None => format!("/{path}"),
            Some(p) => {
                let parent = p.borrow();
                let mut _path = parent.path();
                _path.push_str("/");
                _path.push_str(&path);

                // Make sure we avoid double slash
                if _path[0..2].to_string() == "//" {
                    _path[1..].to_string()
                } else {
                    _path
                }
            }
        }
    }
}

#[test]
fn test_can_create_folder() {
    let root = Folder::new("root");

    assert_eq!(root.name, "root");
    assert_eq!(root.files.len(), 0);
    assert_eq!(root.folders.len(), 0);
}

#[test]
fn test_can_add_files_to_folder() {
    let mut root = Folder::new("root");
    root.add_file(File::new("foo", 10));
    root.add_file(File::new("bar", 20));
    assert_eq!(root.files.len(), 2);
    assert_eq!(root.size(), 30);
}

#[test]
fn test_check_total_folder_size() {
    let root = Rc::new(RefCell::new(Folder::new("")));
    let current = Rc::clone(&root);

    current.borrow_mut().add_file(File::new("foo", 10));
    current.borrow_mut().add_file(File::new("bar", 20));

    for (folder, files) in [
        ("foo", vec![("bazz", 40)]),
        ("bar", vec![("one", 50), ("two", 60)]),
    ] {
        let child = Rc::new(RefCell::new(Folder::new(folder)));

        current
            .borrow_mut()
            .add_folder(folder.to_string(), Rc::clone(&child));

        let mut mut_child = child.borrow_mut();
        mut_child.parent = Some(Rc::clone(&current));

        for (name, size) in files {
            mut_child.add_file(File::new(name, size));
        }
    }

    assert_eq!(root.borrow().size(), 180);

    assert_eq!(
        root.borrow()
            .folders
            .get("foo")
            .expect("Folder not found")
            .borrow()
            .size(),
        40
    );
    assert_eq!(
        root.borrow()
            .folders
            .get("bar")
            .expect("Folder not found")
            .borrow()
            .size(),
        110
    );
}

#[test]
fn test_folder_path() {
    let root = Rc::new(RefCell::new(Folder::new("")));

    let mut current = Rc::clone(&root);

    for name in ["foo", "bar", "bazz"] {
        let child = Rc::new(RefCell::new(Folder::new(name)));

        current
            .borrow_mut()
            .add_folder(name.to_string(), Rc::clone(&child));
        {
            let mut mut_child = child.borrow_mut();
            mut_child.parent = Some(Rc::clone(&current));
        }
        current = child;
    }

    assert_eq!(root.borrow().path(), "/");

    assert_eq!(
        root.borrow()
            .folders
            .get("foo")
            .expect("Folder not found")
            .borrow()
            .path(),
        "/foo"
    );

    assert_eq!(
        root.borrow()
            .folders
            .get("foo")
            .expect("Folder not found")
            .borrow()
            .folders
            .get("bar")
            .expect("Folder not found")
            .borrow()
            .path(),
        "/foo/bar"
    );

    assert_eq!(
        root.borrow()
            .folders
            .get("foo")
            .expect("Folder not found")
            .borrow()
            .folders
            .get("bar")
            .expect("Folder not found")
            .borrow()
            .folders
            .get("bazz")
            .expect("Folder not found")
            .borrow()
            .path(),
        "/foo/bar/bazz"
    );
}
