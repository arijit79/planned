use gtk;
use gtk::prelude::*;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;

struct Note {
    title: String,
    date: String,
    filen: String
}

impl Note {
    fn new(filen: &str) -> Note {
        let mut f = File::open(filen).expect("Can't open file");
        let mut data = String::new();
        f.read_to_string(&mut data).expect("Error reading notes file");

        let note: BTreeMap<String, String> = serde_yaml::from_str(&data)
            .expect("Cannot get valid data from the notes dir");
        let title = note.get("title").unwrap().to_string();
        let date = note.get("date").unwrap().to_string();

        Note {title, date, filen: filen.to_string()}
    }
    fn on_list_store(&self, l: &gtk::ListStore, pos: usize) {
        l.insert_with_values(Some(pos as u32), &[0, 1, 2],
                                &[&self.title, &self.date, &self.filen]);
    }
}

pub fn add_records(l: gtk::ListStore, dir: &str) {
    l.clear();
    let notes_dir = dir.to_string() + "/notes/";
    let path = std::path::Path::new(&notes_dir);
    if ! path.exists() {
        std::fs::create_dir(path).expect("Unable to initialize notes directory");
    }
    for (count, file) in std::fs::read_dir(path).unwrap().enumerate() {
        let filename = file.unwrap().file_name();
        let mut fn_str = String::from(&notes_dir);
        fn_str.push_str(filename.to_str().unwrap());
        let note = Note::new(&fn_str);
        note.on_list_store(&l, count);
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

pub fn start_main(glade: String, dir: String) {
    let b = gtk::Builder::new_from_string(&glade);
    let win: gtk::Window = b.get_object("main_window").unwrap();
    let titlebar: gtk::HeaderBar = b.get_object("titlebar").unwrap();
    let mut userinfo_file = String::new();
    userinfo_file.push_str(&dir);
    userinfo_file.push_str("/userinfo.yaml");
    let user = get_user(userinfo_file);
    titlebar.set_subtitle(Some(&user));
    let notes: gtk::ListStore = b.get_object("notes_list").unwrap();
    let add_button: gtk::Button = b.get_object("add_note").unwrap();
    add_records(notes, &dir);
    add_button.connect_clicked(move |_| {
        let b = gtk::Builder::new_from_string(&glade);
        crate::add_window::init_add(b, dir.clone());
    });
    let notes_tree: gtk::TreeView = b.get_object("notes_tree")
                                            .unwrap();
    win.connect_destroy(|_| std::process::exit(0));
    win.show_all();
    notes_tree.connect_row_activated(move |treeview, path, _| {
        let model = treeview.get_model().unwrap();
        let iter = model.get_iter(path).unwrap();
        let text = model.get_value(&iter, 2).get::<String>().unwrap();
        println!("{}", text.unwrap());
    });
}
