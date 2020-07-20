use rand;
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
use chrono;
use chrono::prelude::*;
use gtk::prelude::*;

pub struct Note {
    pub title: String,
    pub date: String,
    pub content: String,
    pub filen: String
}

impl Note {
    pub fn new(filen: &str) -> Note {
        let mut f = File::open(filen).expect("Can't open file");
        let mut data = String::new();
        f.read_to_string(&mut data).expect("Error reading notes file");

        let note: BTreeMap<String, String> = serde_yaml::from_str(&data)
            .expect("Cannot get valid data from the notes dir");
        let title = note.get("title").unwrap().to_string();
        let date = note.get("date").unwrap().to_string();
        let content =  note.get("content").unwrap().to_string();

        Note {title, date, content, filen: filen.to_string()}
    }
    pub fn on_list_store(&self, l: &gtk::ListStore, pos: usize) {
        l.insert_with_values(Some(pos as u32), &[0, 1, 2],
                                &[&self.title, &self.date, &self.filen]);
    }
}

pub fn gen_fcode() -> u32 {
    let mut rng = rand::thread_rng();
    let rand_no: u32 = rng.gen();
    rand_no
}

pub fn save(text: &str, title: &str, path: String, rand_no: u32) {
    let filen = format!("{}/notes/note{}.yaml", path, rand_no);
    let mut file = File::create(std::path::Path::new(&filen))
                    .expect("Cannot open file");
    let time = Local::now().format("%Y-%m-%d %H:%M").to_string();

    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    map.insert("title", title);
    map.insert("content", text);
    map.insert("date", &time);
    let yaml = serde_yaml::to_string(&map).unwrap();
    file.write_all(yaml.as_bytes()).expect("Cannot write to file");
}


pub fn get_user<'a>(file: String) -> String{
    let mut f = File::open(file).expect("Can't open file userinfo.yaml");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Can't read file");
    let userinfo: BTreeMap<String, String> = serde_yaml::from_str(&data)
                    .expect("No valid data found in userinfo,yaml");
    userinfo.get("user").expect("Key 'user' not found in userinfo.yaml ").clone()
}
