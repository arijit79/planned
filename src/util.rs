// This file contains miscellaneous but important functions used in multiple
// places throughout the program
use rand;
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
use chrono;
use chrono::prelude::*;
use gtk::prelude::*;

// A note struct, abstractly describes a note
pub struct Note {
    pub title: String,
    pub date: String,
    pub content: String,
    pub filen: String
}

impl Note {
    // Read a new note from the given filename
    pub fn new(filen: &str) -> Result<Note, &str> {
        // Open the file
        let f = File::open(filen);
        if let Ok(mut f) = f {
            // Read the yaml data in the file
            let mut data = String::new();
            f.read_to_string(&mut data).expect("Error reading notes file");

            // Generate a new BTreeMap from the given yaml data
            let note: BTreeMap<String, String> = serde_yaml::from_str(&data)
                .expect("Cannot get valid data from the notes dir");
            // Get all fields
            let title = note.get("title").unwrap().to_string();
            let date = note.get("date").unwrap().to_string();
            let content =  note.get("content").unwrap().to_string();
            // Return the note instance
            Ok(Note {title, date, content, filen: filen.to_string()})
        } else {
            Err("The file does not exists")
        }
    }
    // Add a note to a ListStore
    pub fn on_list_store(&self, l: &gtk::ListStore, pos: usize) {
        l.insert_with_values(Some(pos as u32), &[0, 1, 2],
                                &[&self.title, &self.date, &self.filen]);
    }
}

// Generate a random number that is an u32
pub fn gen_fcode() -> u32 {
    let mut rng = rand::thread_rng();
    let rand_no: u32 = rng.gen();
    rand_no
}

// Create a note file from the given data
pub fn save(text: &str, title: &str, path: String) {
    // Create the file
    let mut file = File::create(std::path::Path::new(&path))
                    .expect("Cannot open file");
    // Generate today's date in a nice format
    let time = Local::now().format("%Y-%m-%d %H:%M").to_string();
    // Generate the new BTreeMap and insert the provided data
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    map.insert("title", title);
    map.insert("content", text);
    map.insert("date", &time);
    // Generate a new yaml from the BTrreMap
    let yaml = serde_yaml::to_string(&map).unwrap();
    // Write the data yo thr file
    file.write_all(yaml.as_bytes()).expect("Cannot write to file");
}

// Get the information about the user
pub fn get_user<'a>(file: String) -> String {
    // Read the given file
    let mut f = File::open(file).expect("Can't open file userinfo.yaml");
    // Create a new String and insert the yaml data of the file in it
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Can't read file");
    // Generate the BTreeMap from the yaml data
    let userinfo: BTreeMap<String, String> = serde_yaml::from_str(&data)
                    .expect("No valid data found in userinfo,yaml");
    // Return the user key from the map
    userinfo.get("user").expect("Key 'user' not found in userinfo.yaml ").clone()
}
