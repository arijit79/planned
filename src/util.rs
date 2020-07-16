use rand;
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
use chrono;
use chrono::prelude::*;

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
