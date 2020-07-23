use gtk;
use gtk::prelude::*;
use crate::util::Note;

// Add records to a gtk ListStore
pub fn add_records(l: &gtk::ListStore, dir: &str) {
    // Clear the ListBox, just in case it isen't
    l.clear();
    // Check if the notes directory exists
    let notes_dir = dir.to_string() + "/notes/";
    let path = std::path::Path::new(&notes_dir);
    if ! path.exists() {
        std::fs::create_dir(path).expect("Unable to initialize notes directory");
    }
    // Read all files in the directory
    for (count, file) in std::fs::read_dir(path).unwrap().enumerate() {
        // Get the filename
        let filename = file.unwrap().file_name();
        // Convert the filename to a String
        let mut fn_str = String::from(&notes_dir);
        fn_str.push_str(filename.to_str().unwrap());
        // Create a new note instance from the generated filename
        let note = Note::new(&fn_str);
        // Add it to the ListStore at the count position, which is an enumeration
        note.on_list_store(&l, count);
    }
}

// Function to provide info to get information from a note
fn view_note(selection: gtk::TreeSelection) ->
(String, String, String) {
    // Get the model and iterator from the selection
    let (model, iter) = selection.get_selected().unwrap();
    // Extract the filename from the 2nd column of the ListStore
    let text = model.get_value(&iter, 2).get::<String>().unwrap();
    // Parse the note the note
    let note = Note::new(&text.unwrap());
    // Return the required data
    (note.title, note.date, note.content)
}

// Configure the delete button
pub fn config_delete_buttons(b: &gtk::Builder, notes: gtk::ListStore,
    notes_selection: gtk::TreeSelection, view: gtk::Box) -> gtk::ToolButton {

    // Get the delete button from the builder
    let delete_button: gtk::ToolButton = b.get_object("delete_button").unwrap();
    // configure is Functionality
    delete_button.connect_clicked(move |_| {
        // Initialize the delete window
        crate::delete_window::init_delete(notes.clone(), notes_selection.clone(),
                                            view.clone());
    });
    // Return the button
    delete_button
}

// Confugure the add note button
pub fn config_add_button(b: &gtk::Builder, data: (String, gtk::ListStore)) {
    // Get the button
    let add_button: gtk::ToolButton = b.get_object("add_note").unwrap();
    // Configure the windw
    add_button.connect_clicked(move |_| {
        // Initialize the add ote window
        crate::add_window::init_add(data.0.clone(), data.1.clone());
    });
}

// Start the main window
pub fn start_main(dir: String) {
    // Get the window using the builder and initialize the Window
    let b = gtk::Builder::new_from_string(include_str!("../ui/main.glade"));
    let win: gtk::Window = b.get_object("main_window").unwrap();
    // Generate path to the userinfo file
    let mut userinfo_file = String::new();
    userinfo_file.push_str(&dir);
    userinfo_file.push_str("/userinfo.yaml");
    // Get the user details from the userinfo file
    let user = crate::util::get_user(userinfo_file);
    // Set the subtitle of the window to the user's name
    b.get_object::<gtk::HeaderBar>("titlebar").unwrap().set_subtitle(Some(&user));
    // Get the ListStore which will contain all the notes
    let notes: gtk::ListStore = b.get_object("notes_list").unwrap();
    // Add data to the notes ListStore
    add_records(&notes, &dir);
    // Configure the add note button
    config_add_button(&b, (dir.clone(), notes.clone()));
    // Get the notes TreeView and TreeSelection
    let notes_tree: gtk::TreeView = b.get_object("notes_tree").unwrap();
    let notes_selection: gtk::TreeSelection = b.get_object("notes_tree_selection").unwrap();
    // Fetch the note viewer from the builder
    let notes_view: gtk::Box = b.get_object("notes_view").unwrap();
    // Get the configured delete button
    let delete_button = config_delete_buttons(&b, notes.clone(), notes_selection.clone(),
                                                        notes_view.clone());
    // Set the TreeView to be activated in a single click
    notes_tree.set_activate_on_single_click(true);
    // Destroy the entire process if this process is killed
    win.connect_destroy(|_| gtk::main_quit());
    // Configure notes_tree TreeView for selection action
    notes_tree.connect_row_activated(move |_, _, _| {
        // Set the delete button to be sensitive and display the notes content
        delete_button.set_sensitive(true);
        notes_view.set_visible(true);
        let data = view_note(notes_selection.clone());
        b.get_object::<gtk::Label>("note_title").unwrap()
                        .set_text(&format!("Note Title:\t{}", data.0));
        b.get_object::<gtk::Label>("creation_date").unwrap()
                        .set_text(&format!("Creation Date:\t{}", data.1));
        b.get_object::<gtk::TextBuffer>("textbuffer1").unwrap()
                        .set_text(&format!("{}", data.2));
    });
    // Show the main window
    win.show_all();
}
