pub trait GetSize {
    fn size(& self) -> u64;
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: u64,
}

impl GetSize for File {
    fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub subdirectories: Vec<Directory>,
}

impl GetSize for Directory {
    fn size(&self) -> u64 {
        let subdirs_sum : u64 = self.subdirectories.iter().map(|child| child.size()).sum();
        let files_sum : u64 = self.files.iter().map(|child| child.size()).sum();
        subdirs_sum + files_sum
    }
}

impl Directory {
    pub fn get_directory_sizes(&self, base_name: Option<String>) -> Vec<(String, u64)> {
        let mut sizes = Vec::new();
        let name = match base_name {
            Some(name) => name + self.name.clone().as_str() + "/",
            None => self.name.clone(),
        };
        sizes.push((name.clone(), self.size()));

        for child in &self.subdirectories {
            let child_sizes = child.get_directory_sizes(Some(name.clone()));
            sizes.extend(child_sizes.iter().cloned());
        }
        sizes
    }
}