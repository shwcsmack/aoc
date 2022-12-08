use std::collections::{BTreeMap, VecDeque};

use super::computer::Path;

#[derive(Debug, Default)]
pub struct FileSystem {
    root: Directory,
}

impl FileSystem {
    pub fn build(data: BTreeMap<String, String>) -> Self {
        dbg!(&data);
        let mut root = Directory {
            path: Path::new("/"),
            name: "/".to_string(),
            items: Vec::default(),
        };
        let file_items: Vec<FileItem> = data
            .iter()
            .map(|(key, value)| FileItem::parse_from_btree(key, value))
            .collect();

        root.add_items(file_items);
        Self { root }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum FileItem {
    Dir(Directory),
    File(File),
}

impl FileItem {
    fn parse_from_btree(file: &str, path: &str) -> FileItem {
        let path = path.parse::<Path>().unwrap();
        let file_parts: Vec<&str> = file.split(' ').collect();
        if file_parts[0] == "dir" {
            let dir = Directory {
                path,
                name: file_parts[1].to_string(),
                items: Vec::default(),
            };
            FileItem::Dir(dir)
        } else {
            let file = File {
                path,
                name: file_parts[1].to_string(),
                size: file_parts[0].parse().unwrap(),
            };
            FileItem::File(file)
        }
    }

    fn dir(&self) -> Result<Directory, String> {
        match self {
            FileItem::Dir(dir) => Ok(dir.to_owned()),
            FileItem::File(_) => Err("Not a directory".to_string()),
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            FileItem::Dir(_) => true,
            FileItem::File(_) => false,
        }
    }

    fn path(&self) -> Path {
        match self {
            FileItem::Dir(dir) => dir.path.clone(),
            FileItem::File(file) => file.path.clone(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Directory {
    path: Path,
    name: String,
    items: Vec<FileItem>,
}

impl Directory {
    fn add_items(&mut self, items: Vec<FileItem>) {
        let mut items: VecDeque<FileItem> = items.into();
        while items.len() > 0 {
            let current_item = items.pop_front().unwrap();

            if self.path == current_item.path() {
                self.items.push(current_item);
            } else {
                self.items.iter().fla
                for item in &self.items {
                    if item.is_dir() && item.path() == current_item.path() {
                        item.dir().unwrap().items.push(current_item.clone());
                    } else {

                    }
                }
            }
        }
        // let mut dirs: Vec<FileItem> = Vec::default();
        // for (idx, item) in items.iter().enumerate() {
        //     match item {
        //         FileItem::Dir(dir) => {
        //             let back_half = items.split_off(idx + 1);
        //         }
        //     }
        // }
        // let dirs: Vec<FileItem> = items
        //     .clone()
        //     .into_iter()
        //     .filter(|item| match item {
        //         FileItem::Dir(_) => true,
        //         FileItem::File(_) => false,
        //     })
        //     .filter(|item| match item {
        //         FileItem::Dir(dir) => dir.path == self.path,
        //         _ => false,
        //     })
        //     .collect();
        // dbg!(&dirs);

        // dirs.iter()
        //     .for_each(|dir| dir.dir().unwrap().add_items(items.clone()));
        // dirs.into_iter().for_each(|dir| self.items.push(dir));

        // let files: Vec<FileItem> = items
        //     .into_iter()
        //     .filter(|item| match item {
        //         FileItem::File(_) => true,
        //         FileItem::Dir(_) => false,
        //     })
        //     .filter(|item| match item {
        //         FileItem::File(file) => file.path == self.path,
        //         _ => false,
        //     })
        //     .collect();
        // dbg!(&files);
        // files.into_iter().for_each(|item| self.items.push(item));
    }
}

#[derive(Debug, Clone, PartialEq)]
struct File {
    path: Path,
    name: String,
    size: u64,
}
