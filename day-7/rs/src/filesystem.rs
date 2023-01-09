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
    parent: Option<Rc<Folder>>,
    pub folders: HashMap<String, Folder>,
    pub files: HashMap<String, File>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            parent: None,
            folders: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn size(&self) -> u32 {
        let mut total_size = 0;
        for (_name, file) in &self.files {
            total_size += file.size;
        }
        for (_name, folder) in &self.folders {
            total_size += folder.size();
        }
        total_size
    }

    pub fn add_file(&mut self, file: File) -> () {
        self.files.insert(file.name.clone(), file);
    }

    pub fn add_folder(&mut self, mut folder: Folder) -> () {
        folder.parent = Some(Rc::new(self.clone()));
        self.folders.insert(folder.name.clone(), folder);
    }

    pub fn path(&self) -> String {
        let mut path = Vec::new();
        let mut current = self;
        while let Some(parent) = &current.parent {
            path.push(current.name.as_str());
            current = parent;
        }
        if current.name.as_str() != "" {
            path.push(current.name.as_str());
        }
        path.reverse();
        let mut value = "/".to_string();
        value.push_str(path.join("/").as_str());
        value
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
fn test_can_add_subfolder_to_folder() {
    let mut root = Folder::new("root");
    root.add_folder(Folder::new("foo"));
    assert_eq!(root.folders.len(), 1);
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
    let mut root = Folder::new("root");

    let mut subfolder = Folder::new("foo");
    subfolder.add_file(File::new("bazz", 40));

    let mut subsubfolder = Folder::new("foo");
    subsubfolder.add_file(File::new("one", 50));
    subsubfolder.add_file(File::new("two", 60));

    subfolder.add_folder(subsubfolder);
    root.add_folder(subfolder);

    root.add_file(File::new("foo", 10));
    root.add_file(File::new("bar", 20));

    assert_eq!(root.size(), 180);
    assert_eq!(root.folders.get("foo").unwrap().size(), 150);
    assert_eq!(
        root.folders
            .get("foo")
            .expect("Folder not found")
            .folders
            .get("foo")
            .expect("Folder not found")
            .size(),
        110
    );
}

#[test]
fn test_folder_path() {
    let bazz = Folder::new("bazz");

    let mut bar = Folder::new("bar");
    bar.add_folder(bazz);

    let mut foo = Folder::new("foo");
    foo.add_folder(bar);

    let mut root = Folder::new("");
    root.add_folder(foo);

    assert_eq!(root.path(), "/");

    assert_eq!(
        root.folders.get("foo").expect("Folder not found").path(),
        "/foo"
    );

    assert_eq!(
        root.folders
            .get("foo")
            .expect("Folder not found")
            .folders
            .get("bar")
            .expect("Folder not found")
            .path(),
        "/foo/bar"
    );

    assert_eq!(
        root.folders
            .get("foo")
            .expect("Folder not found")
            .folders
            .get("bar")
            .expect("Folder not found")
            .folders
            .get("bazz")
            .expect("Folder not found")
            .path(),
        "/foo/bar/bazz"
    );
}
