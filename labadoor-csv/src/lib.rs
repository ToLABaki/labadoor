use serde::Deserialize;

pub mod acl;
pub mod logging;

pub struct CSVArgs {
    pub path: String,
}

impl CSVArgs {
    pub fn new(&self) -> CSV {
        CSV {
            path: self.path.clone(),
        }
    }
}

pub struct CSV {
    pub path: String,
}

impl CSV {
    fn find<T: for<'a> Deserialize<'a> + PartialEq>(&self, needle: T, path: &str) -> Option<T> {
        let p = format!("{}/{}", self.path, path);
        let file = std::fs::File::open(p).unwrap();
        let mut reader = csv::Reader::from_reader(file);
        reader
            .deserialize()
            .map(|x: Result<T, csv::Error>| x.unwrap())
            .collect::<Vec<T>>()
            .into_iter()
            .find(|x: &T| x == &needle)
    }
}
