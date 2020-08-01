use crate::{delete_window::init_delete, util::Note};
use gtk::prelude::*;

// Types of toolbar buttons
enum ToolButton {
    Delete,
    Edit,
}

// Add records to a gtk ListStore
pub fn add_records(l: &gtk::ListStore, dir: &str) {
    // Clear the ListBox, just in case it isen't
    l.clear();
    // Check if the notes directory exists
    let notes_dir = dir.to_string() + "/notes/";
    let path = std::path::Path::new(&notes_dir);
    if !path.exists() {
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
        let note = Note::new(&fn_str).unwrap();
        // Add it to the ListStore at the count position, which is an enumeration
        note.on_list_store(&l, count);
    }
}

// Function to provide info to get information from a note
fn view_note(selection: gtk::TreeSelection) -> (String, String, String) {
    let text = get_selected_col(selection, 2);
    // Parse the note the note
    let note = Note::new(&text).unwrap();
    // Return the required data
    (note.title, note.date, note.content)
}

fn get_selected_col(selection: gtk::TreeSelection, index: i32) -> String {
    // Get the model and iterator from the selection
    let (model, iter) = selection.get_selected().unwrap();
    // Extract the text from the nth column of the model
    let col = model.get_value(&iter, index).get::<String>().unwrap();
    col.unwrap()
}

// Confugure the add note button
fn config_add_button(b: &gtk::Builder, data: (String, gtk::ListStore)) {
    // Get the button
    let add_button: gtk::ToolButton = b.get_object("add_note").unwrap();
    // Configure the windw
    add_button.connect_clicked(move |_| {
        // Initialize the add ote window
        crate::add_window::init_add(data.0.clone(), data.1.clone(), None);
    });
}

// Configure toolbar buttons
fn config_tool_button(
    b: gtk::Builder,
    id: &str,
    path: String,
    notes: gtk::ListStore,
    notes_selection: gtk::TreeSelection,
    view: gtk::Box,
    btype: ToolButton,
) -> gtk::ToolButton {
    // Get the button
    let button: gtk::ToolButton = b.get_object(id).unwrap();
    // Match the ToolButton type
    match btype {
        // If it is of delete type onfigure it with a delete function
        ToolButton::Delete => {
            button.connect_clicked(move |_| {
                init_delete(notes.clone(), notes_selection.clone(), view.clone())
            });
        }
        // If it is of edit type onfigure it with a edit function
        ToolButton::Edit => {
            button.connect_clicked(move |_| {
                let filen = get_selected_col(notes_selection.clone(), 2);
                let note = Note::new(&filen).unwrap();
                crate::add_window::init_add(path.clone(), notes.clone(), Some(note));
                view.hide();
            });
        }
    }
    button
}

fn update_note_view(b: gtk::Builder, selection: gtk::TreeSelection) {
    let data = view_note(selection);
    b.get_object::<gtk::Label>("note_title")
        .unwrap()
        .set_text(&format!("Note Title:\t{}", data.0));
    b.get_object::<gtk::Label>("creation_date")
        .unwrap()
        .set_text(&format!("Creation Date:\t{}", data.1));
    b.get_object::<gtk::TextBuffer>("textbuffer1")
        .unwrap()
        .set_text(&data.2);
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
    b.get_object::<gtk::HeaderBar>("titlebar")
        .unwrap()
        .set_subtitle(Some(&user));
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
    let delete_button = config_tool_button(
        b.clone(),
        "delete_button",
        dir.clone(),
        notes.clone(),
        notes_selection.clone(),
        notes_view.clone(),
        ToolButton::Delete,
    );
    // Get the configured edit button
    let edit_button = config_tool_button(
        b.clone(),
        "edit_button",
        dir,
        notes,
        notes_selection.clone(),
        notes_view.clone(),
        ToolButton::Edit,
    );
    // Set the TreeView to be activated in a single click
    notes_tree.set_activate_on_single_click(true);
    // Destroy the entire process if this process is killed
    win.connect_destroy(|_| gtk::main_quit());
    // Configure notes_tree TreeView for selection action
    notes_tree.connect_row_activated(move |_, _, _| {
        // Set the delete button to be sensitive and display the notes content
        delete_button.set_sensitive(true);
        edit_button.set_sensitive(true);
        notes_view.set_visible(true);
        update_note_view(b.clone(), notes_selection.clone())
    });
    // Show the main window
    win.show_all();
}
