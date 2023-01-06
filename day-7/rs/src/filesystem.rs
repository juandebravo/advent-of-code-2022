pub struct File<'a> {
    pub name: &'a str,
    pub size: u32,
}

pub struct Dir<'a> {
    pub name: &'a str,
    pub dirs: Vec<Dir<'a>>,
    pub files: Vec<File<'a>>,
}

impl<'a> Dir<'a> {
    pub fn size(&self) -> u32 {
        let mut total_size = 0;
        for file in &self.files {
            total_size += file.size;
        }
        for dir in &self.dirs {
            total_size += dir.size();
        }
        total_size
    }

    pub fn add_file(&mut self, file: File<'a>) -> () {
        self.files.push(file);
    }

    pub fn add_dir(&mut self, dir: Dir<'a>) -> () {
        self.dirs.push(dir);
    }
}

#[test]
fn test_can_create_file() {
    let file = File {
        name: "foo",
        size: 10,
    };

    assert_eq!(file.name, "foo");
    assert_eq!(file.size, 10);
}

#[test]
fn test_can_create_dir() {
    let mut root = Dir {
        name: "root",
        dirs: Vec::default(),
        files: Vec::default(),
    };

    assert_eq!(root.name, "root");
    assert_eq!(root.dirs.len(), 0);
    assert_eq!(root.files.len(), 0);

    root.dirs.push(Dir {
        name: "foo",
        dirs: Vec::default(),
        files: Vec::default(),
    });

    root.files.push(File {
        name: "foo",
        size: 10,
    });
    root.files.push(File {
        name: "bar",
        size: 20,
    });

    assert_eq!(root.size(), 30);
}

#[test]
fn test_dir_size() {
    let mut root = Dir {
        name: "root",
        dirs: Vec::default(),
        files: Vec::default(),
    };

    let mut subfolder = Dir {
        name: "foo",
        dirs: vec![Dir {
            name: "foo2",
            dirs: Vec::default(),
            files: vec![
                File {
                    name: "one",
                    size: 50,
                },
                File {
                    name: "two",
                    size: 60,
                },
            ],
        }],
        files: Vec::default(),
    };

    subfolder.files.push(File {
        name: "bazz",
        size: 40,
    });

    root.dirs.push(subfolder);

    root.files.push(File {
        name: "foo",
        size: 10,
    });
    root.files.push(File {
        name: "bar",
        size: 20,
    });

    assert_eq!(root.size(), 180);
    assert_eq!(root.dirs[0].size(), 150);
    assert_eq!(root.dirs[0].dirs[0].size(), 110);
}
