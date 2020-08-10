use crate::{delete_window::init_delete, util::Note};
use gtk::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

// Types of toolbar buttons
enum ToolButton {
    Delete,
    Edit,
}

fn init_tag_page(
    stack: &gtk::Stack,
    tag: &str,
    note: &crate::util::Note,
    list_map: &mut HashMap<String, gtk::ListStore>,
) {
    let page = stack.get_child_by_name(tag);
     if let Some(_) = page {
         let liststore = list_map.get(format!("list-{}", tag).as_str()).unwrap();
         let n = liststore.get_n_columns();
         note.on_list_store(&liststore, n as usize);
     } else {
         // Initialize the list store data type
         let liststore_dtype = [glib::Type::String, glib::Type::String, glib::Type::String, glib::Type::String];
         // Initialize the list store and tree view
         let liststore = gtk::ListStore::new(&liststore_dtype);
         let tree = gtk::TreeView::new();
         // Initialize the Tree columns and their renderers
         let text = gtk::TreeViewColumn::new();
         let date = gtk::TreeViewColumn::new();
         let tags = gtk::TreeViewColumn::new();
         let text_render = gtk::CellRendererText::new();
         let date_render = gtk::CellRendererText::new();
         let tags_render = gtk::CellRendererText::new();

         // Add list store to the tree
         tree.set_model(Some(&liststore));
         // Configure the 1st column by adding title, area and associate its renderer
         text.set_title("Title");
         text.pack_start(&text_render, true);
         text.add_attribute(&text_render, "text", 0);
         // Configure the 2nd column by adding title, area and associate its renderer
         date.set_title("Date");
         date.pack_start(&date_render, true);
         date.add_attribute(&date_render, "text", 1);
         // Configure the 3rs column by adding title, area and associate its renderer
         tags.set_title("Tags");
         tags.pack_start(&tags_render, true);
         tags.add_attribute(&tags_render, "text", 2);
         // Add the columns to the tree
         tree.append_column(&text);
         tree.append_column(&date);
         tree.append_column(&tags);
         // Add the note to the liststore
         note.on_list_store(&liststore, 0);
         // Add the treeview to the stack
         stack.add_titled(&tree, tag, tag);
         list_map.insert(format!("list-{}", tag), liststore);
     }
}

// Add records to a gtk ListStore
pub fn refresh_ui(l: &gtk::ListStore, mut dir: PathBuf, 
    tag_stack: gtk::Stack) -> HashMap<String, gtk::ListStore> {
    // Clear the ListBox, just in case it isen't
    l.clear();
    // Vector of available tags
    let mut list_map: HashMap<String, gtk::ListStore> = HashMap::new();
    // Check if the notes directory exists
    dir.push("notes");
    if !dir.exists() {
        std::fs::create_dir(dir).expect("Unable to initialize notes directory");
    } else {
        // Read all files in the directory
        for (count, file) in std::fs::read_dir(&dir).unwrap().enumerate() {
            // Get the relative filename of the file in respect to the data dir
            // Get the OsString and convert it into &str
            let rel_ostr = file.unwrap().file_name();
            let relative_file = rel_ostr.to_str().unwrap();
            // Generate the filename
            let mut filename = PathBuf::new();
            filename.push(&dir);
            filename.push(relative_file);
            // Create a new note instance from the generated filename
            let note = &Note::new(filename).unwrap();
            // Add it to the ListStore at the count position, which is an enumeration
            note.on_list_store(&l, count);
            for tag in note.tags.iter() {
                init_tag_page(&tag_stack, tag, note, &mut list_map);
            }
        }
    }
    list_map
}

// Function to provide info to get information from a note
fn view_note(selection: gtk::TreeSelection) -> (String, String, String) {
    let text = get_selected_filename(selection);
    // Parse the note the note
    let note = Note::new(PathBuf::from(text)).unwrap();
    // Return the required data
    (note.title, note.date, note.content)
}

fn get_selected_filename(selection: gtk::TreeSelection) -> String {
    // Column where filename is tored
    let index = 3;
    // Get the model and iterator from the selection
    let (model, iter) = selection.get_selected().unwrap();
    // Extract the text from the nth column of the model
    let col = model.get_value(&iter, index).get::<String>().unwrap();
    col.unwrap()
}

// Confugure the add note button
fn config_add_button(b: &gtk::Builder, data: (PathBuf, gtk::ListStore, gtk::Stack)) {
    // Get the button
    let add_button: gtk::ToolButton = b.get_object("add_note").unwrap();
    // Configure the windw
    add_button.connect_clicked(move |_| {
        // Initialize the add ote window
        crate::add_window::init_add(data.0.clone(), data.1.clone(), None, data.2.clone());
    });
}

// Configure toolbar buttons
fn config_tool_button(
    b: gtk::Builder,
    id: &str,
    path: PathBuf,
    notes: gtk::ListStore,
    notes_selection: gtk::TreeSelection,
    view: gtk::Box,
    btype: ToolButton,
    tag_stack: gtk::Stack,
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
                let filen = get_selected_filename(notes_selection.clone());
                let note = Note::new(PathBuf::from(filen)).unwrap();
                crate::add_window::init_add(path.clone(), notes.clone(), Some(note), tag_stack.clone());
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
pub fn start_main(dir: std::path::PathBuf) {
    // Get the window using the builder and initialize the Window
    let b = gtk::Builder::new_from_string(include_str!("../ui/main.glade"));
    let win: gtk::Window = b.get_object("main_window").unwrap();
    // Generate path to the userinfo file
    let mut userinfo_file = std::path::PathBuf::from(&dir);
    userinfo_file.push("userinfo.yaml");
    // Get the user details from the userinfo file
    let user = crate::util::get_user(userinfo_file);
    // Set the subtitle of the window to the user's name
    b.get_object::<gtk::HeaderBar>("titlebar")
        .unwrap()
        .set_subtitle(Some(&user));
    // Get the ListStore which will contain all the notes
    let notes: gtk::ListStore = b.get_object("notes_list").unwrap();
    let tag_stack: gtk::Stack = b.get_object("stack1").unwrap();
    // Add data to the notes ListStore
    let tags_map = refresh_ui(&notes, dir.clone(), tag_stack.clone());
    // Configure the add note button
    config_add_button(&b, (dir.clone(), notes.clone(), tag_stack.clone() ));
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
        tag_stack.clone()
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
        tag_stack.clone()
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
