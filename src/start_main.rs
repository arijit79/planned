use gtk;
use gtk::prelude::*;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;

pub fn add_records(l: gtk::ListStore, dir: String) {
    let notes_dir = dir + "/notes/";
    let path = std::path::Path::new(&notes_dir);
    if ! path.exists() {
        std::fs::create_dir(path).expect("Unable to initialize notes directory");
    }
    for (count, file) in std::fs::read_dir(path).unwrap().enumerate() {
        let filename = file.unwrap().file_name();
        let mut fn_str = String::from(&notes_dir);
        fn_str.push_str(filename.to_str().unwrap());
        let mut f = File::open(fn_str).expect("Can't open file");
        let mut data = String::new();
        f.read_to_string(&mut data);
        let note: BTreeMap<String, String> = serde_yaml::from_str(&data)
            .expect("Cannot get valid data from the notes dir");
        let title = note.get("title").unwrap();
        let date = note.get("date").unwrap();
        l.insert_with_values(Some(count as u32), &[0, 1], &[title, date]);
    }
}

fn get_user<'a>(file: String) -> String{
    let mut f = File::open(file).expect("Can't open file userinfo.yaml");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Can't read file");
    let userinfo: BTreeMap<String, String> = serde_yaml::from_str(&data)
                    .expect("No valid data found in userinfo,yaml");
    userinfo.get("user").expect("Key 'user' not found in userinfo.yaml ").clone()
}

pub fn start_main(b: gtk::Builder, dir: String) {
    let win: gtk::Window = b.get_object("window1").unwrap();
    let titlebar: gtk::HeaderBar = b.get_object("titlebar").unwrap();
    let mut userinfo_file = String::new();
    userinfo_file.push_str(&dir);
    userinfo_file.push_str("/userinfo.yaml");
    let user = get_user(userinfo_file);
    titlebar.set_subtitle(Some(&user));
    let notes: gtk::ListStore = b.get_object("notes_list").unwrap();
    // notes.insert_with_values(Some(1), &[0, 1], &[&"Note 1", &"2020-07-06"]);
    add_records(notes, dir);
    win.connect_destroy(|_| std::process::exit(0));
    win.show_all();
}
