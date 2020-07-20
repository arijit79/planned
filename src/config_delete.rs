use gtk::prelude::*;

pub fn init_delete(notes: gtk::ListStore, notes_tree: gtk::TreeView,
    notes_selection: gtk::TreeSelection) {

    let src = include_str!("../ui/delete.glade");
    let b = gtk::Builder::new_from_string(src);
    let delete_confirm: gtk::Window = b.get_object("confirm_delete").unwrap();
    delete_confirm.show_all();
    let cancel_button: gtk::Button = b.get_object("cancel_delete").unwrap();
    let delete_confirm_clone = delete_confirm.clone();
    cancel_button.connect_clicked(move |_| {
        delete_confirm.destroy();
    });

    let confirm_delete_button: gtk::Button = b.get_object("delete_button").unwrap();
    confirm_delete_button.connect_clicked(move |_| {
        let selection = notes_selection.get_selected().unwrap();

        let model = notes_tree.get_model().unwrap();
        let iter = selection.1;
        let text = model.get_value(&iter, 2).get::<String>().unwrap();
        notes.remove(&iter);
        let r = std::fs::remove_file(text.unwrap());
        match r {
            Ok(_) => {},
            Err(_) => eprintln!("Error deleting file")
        }
        delete_confirm_clone.destroy();
    });
}
