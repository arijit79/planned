use gtk;
use gtk::prelude::*;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;
use crate::util::Note;

pub fn add_records(l: &gtk::ListStore, dir: &str) {
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

fn view_note(notes_tree: gtk::TreeView, selection: gtk::TreeSelection) ->
(String, String) {
    let selection = selection.get_selected().unwrap();
    let model = notes_tree.get_model().unwrap();
    let text = model.get_value(&selection.1, 2).get::<String>().unwrap();
    let note = Note::new(&text.unwrap());
    (note.title, note.date)
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
    let mut userinfo_file = String::new();
    userinfo_file.push_str(&dir);
    userinfo_file.push_str("/userinfo.yaml");
    let user = get_user(userinfo_file);
    b.get_object::<gtk::HeaderBar>("titlebar").unwrap().set_subtitle(Some(&user));
    let notes: gtk::ListStore = b.get_object("notes_list").unwrap();
    add_records(&notes, &dir);
    crate::add_window::config_add_button(&b, (dir.clone(), notes.clone()));
    let notes_tree: gtk::TreeView = b.get_object("notes_tree").unwrap();
    let notes_selection: gtk::TreeSelection = b.get_object("notes_tree_selection").unwrap();

    let notes_view: gtk::Box = b.get_object("notes_view").unwrap();

    notes_tree.set_activate_on_single_click(true);
    win.connect_destroy(|_| std::process::exit(0));
    notes_tree.connect_row_activated(move |tree, _, _| {
        let delete_button = crate::config_delete::config_delete_buttons(&b, notes.clone(),
                                                            tree.clone(),
                                                            notes_selection.clone());
        delete_button.set_sensitive(true);
        notes_view.set_visible(true);
        let data = view_note(tree.clone(), notes_selection.clone());
        b.get_object::<gtk::Label>("note_title").unwrap()
                        .set_text(&format!("Note Title:\t{}", data.0));
        b.get_object::<gtk::Label>("creation_date").unwrap()
                        .set_text(&format!("Creation Date:\t{}", data.1));
    });
    win.show_all();
}
