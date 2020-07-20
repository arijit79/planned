use gtk::prelude::*;

// Initialize the delete note window
pub fn init_delete(notes: gtk::ListStore, notes_selection: gtk::TreeSelection) {
    // Make the Builder from delete.glade file
    let b = gtk::Builder::new_from_string(include_str!("../ui/delete.glade"));
    // Get and show the delete Window
    let delete_confirm: gtk::Window = b.get_object("confirm_delete").unwrap();
    // Get the cancel
    let cancel_button: gtk::Button = b.get_object("cancel_delete").unwrap();
    // Clone the window for later use
    let delete_confirm_clone = delete_confirm.clone();
    // Get the delete_button
    let confirm_delete_button: gtk::Button = b.get_object("delete_button").unwrap();
    // Initialize the window
    b.get_object::<gtk::Window>("confirm_delete").unwrap().show_all();

    // If cancel is pressed, destroy the window
    cancel_button.connect_clicked(move |_| {
        delete_confirm.destroy();
    });

    // If delete is pressed, delete the file
    confirm_delete_button.connect_clicked(move |_| {
        // Get the model and iter using the notes_selection TreeSelection
        let (model, iter) = notes_selection.get_selected().unwrap();
        // Get the value from the model and get the String out of it
        let text = model.get_value(&iter, 2).get::<String>().unwrap();
        // Remove it from the notes ListStore
        notes.remove(&iter);
        // Remove the note file
        let r = std::fs::remove_file(text.unwrap());
        // Handle the error properly
        match r {
            Ok(_) => {},
            Err(_) => eprintln!("Error deleting file")
        }
        // Destroy the window after deleting
        delete_confirm_clone.destroy();
    });
}
