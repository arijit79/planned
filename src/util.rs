use rand;
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn save(text: &str, path: String) {
    let mut rng = rand::thread_rng();
    let rand_no: u32 = rng.gen();
    let filen = format!("{}/notes/note{}.yaml", path, rand_no);
    let mut file = File::create(std::path::Path::new(&filen)).expect("Cannot open file");
    let systime = SystemTime::now();
    let time = &format!("{:?}", systime.duration_since(UNIX_EPOCH).unwrap());

    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    map.insert("title", "Ttile");
    map.insert("content", text);
    map.insert("date", time);
    let yaml = serde_yaml::to_string(&map).unwrap();
    file.write_all(yaml.as_bytes()).expect("Cannot write to file");
}
