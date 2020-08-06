// This file contains miscellaneous but important functions used in multiple
// places throughout the program
use chrono::prelude::*;
use gtk::prelude::*;
use rand::prelude::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

// A note struct, abstractly describes a note
#[derive(Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub date: String,
    pub content: String,
    pub tags: Vec<String>,
    pub filen: PathBuf,
}

impl Note {
    // Read a new note from the given filename
    pub fn new(filen: PathBuf) -> Result<Note, &'static str> {
        // Open the file
        let f = File::open(filen);
        if let Ok(mut f) = f {
            // Read the yaml data in the file
            let mut data = String::new();
            f.read_to_string(&mut data)
                .expect("Error reading notes file");

            // Generate a new BTreeMap from the given yaml data
            let note: Note =
                serde_yaml::from_str(&data).expect("Cannot get valid data from the notes dir");
            Ok(note)
        } else {
            Err("The file does not exists")
        }
    }
    // Add a note to a ListStore
    pub fn on_list_store(&self, l: &gtk::ListStore, pos: usize) {
        let mut tags_str = String::new();
        for tag in &self.tags {
            tags_str.push_str(&tag);
        }
        let file_str = self.filen.to_str().unwrap();
        l.insert_with_values(
            Some(pos as u32),
            &[0, 1, 2, 3],
            &[&self.title, &self.date, &tags_str, &file_str]);
    }
}

// Defines the contents of the add window
#[derive(Clone)]
pub struct Content {
    pub title: String,
    pub body: String
}

// Overload the eq method for correctly checking if other content is equal
impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.body == other.body
    }
}

// Generate a random number that is an u32
pub fn gen_fcode() -> u32 {
    let mut rng = rand::thread_rng();
    let rand_no: u32 = rng.gen();
    rand_no
}

// Create a note file from the given data
pub fn save(content: Content, path: PathBuf) {
    println!("{}", path.display());
    // Create the file
    let mut file = File::create(std::path::Path::new(&path)).expect("Cannot open file");
    // Generate today's date in a nice format
    let time = Local::now().format("%Y-%m-%d %H:%M").to_string();
    // Generate the new Note and insert the provided data
    let note = Note {
        title: content.title,
        date: time,
        content: content.body,
        filen: path,
        tags: vec!["important".to_string()]
    };
    // Generate a new yaml from the Note
    let yaml = serde_yaml::to_string(&note).unwrap();
    // Write the data yo thr file
    file.write_all(yaml.as_bytes())
        .expect("Cannot write to file");
}

// Get the information about the user
pub fn get_user(file: std::path::PathBuf) -> String {
    // Read the given file
    let mut f = File::open(file).expect("Can't open file userinfo.yaml");
    // Create a new String and insert the yaml data of the file in it
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Can't read file");
    // Generate the BTreeMap from the yaml data
    let userinfo: BTreeMap<String, String> =
        serde_yaml::from_str(&data).expect("No valid data found in userinfo,yaml");
    // Return the user key from the map
    userinfo
        .get("user")
        .expect("Key 'user' not found in userinfo.yaml ")
        .clone()
}
