use gtk;
use gtk::prelude::*;
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
(String, String, String) {
    let selection = selection.get_selected().unwrap();
    let model = notes_tree.get_model().unwrap();
    let text = model.get_value(&selection.1, 2).get::<String>().unwrap();
    let note = Note::new(&text.unwrap());
    (note.title, note.date, note.content)
}

pub fn config_delete_buttons(b: &gtk::Builder,
notes: gtk::ListStore, notes_tree: gtk::TreeView, notes_selection: gtk::TreeSelection)
-> gtk::ToolButton {

    let delete_button: gtk::ToolButton = b.get_object("delete_button").unwrap();
    delete_button.connect_clicked(move |_| {
        crate::config_delete::init_delete(notes.clone(), notes_tree.clone(),
                                                        notes_selection.clone());
    });
    delete_button
}

pub fn config_add_button(b: &gtk::Builder, data: (String, gtk::ListStore)) {
    let add_button: gtk::ToolButton = b.get_object("add_note").unwrap();
    add_button.connect_clicked(move |_| {
        crate::add_window::init_add(data.0.clone(), data.1.clone());
    });
}

pub fn start_main(glade: String, dir: String) {
    let b = gtk::Builder::new_from_string(&glade);
    let win: gtk::Window = b.get_object("main_window").unwrap();
    let mut userinfo_file = String::new();
    userinfo_file.push_str(&dir);
    userinfo_file.push_str("/userinfo.yaml");
    let user = crate::util::get_user(userinfo_file);
    b.get_object::<gtk::HeaderBar>("titlebar").unwrap().set_subtitle(Some(&user));
    let notes: gtk::ListStore = b.get_object("notes_list").unwrap();
    add_records(&notes, &dir);
    config_add_button(&b, (dir.clone(), notes.clone()));
    let notes_tree: gtk::TreeView = b.get_object("notes_tree").unwrap();
    let notes_selection: gtk::TreeSelection = b.get_object("notes_tree_selection").unwrap();

    let notes_view: gtk::Box = b.get_object("notes_view").unwrap();

    let delete_button = config_delete_buttons(&b, notes.clone(), notes_tree.clone(),
                                                notes_selection.clone());

    notes_tree.set_activate_on_single_click(true);
    win.connect_destroy(|_| std::process::exit(0));
    notes_tree.connect_row_activated(move |tree, _, _| {
        delete_button.set_sensitive(true);
        notes_view.set_visible(true);
        let data = view_note(tree.clone(), notes_selection.clone());
        b.get_object::<gtk::Label>("note_title").unwrap()
                        .set_text(&format!("Note Title:\t{}", data.0));
        b.get_object::<gtk::Label>("creation_date").unwrap()
                        .set_text(&format!("Creation Date:\t{}", data.1));
        b.get_object::<gtk::TextBuffer>("textbuffer1").unwrap()
                        .set_text(&format!("{}", data.2));
    });
    win.show_all();
}
