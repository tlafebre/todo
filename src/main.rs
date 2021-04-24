// add timestamps
// sort on alpha and timestamp
// avoid double inputs
// Tabularize

use std::collections::HashMap;
use std::io::Error;

struct ToDo {
    map: HashMap<String, bool>,
}

impl ToDo {
    fn new() -> Result<ToDo, Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("todo.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(ToDo { map }),
            Err(e) if e.is_eof() => Ok(ToDo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("todo.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }
}
fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = ToDo::new().expect("failed to initialize DB");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("task saved."),
            Err(reason) => println!("failed to save task: {}", reason),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' not found", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("task saved."),
                Err(reason) => println!("failed to save task: {}", reason),
            },
        }
    }
}
