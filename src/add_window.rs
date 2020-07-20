use gtk::prelude::*;

fn get_buffer_str(buff: &gtk::TextBuffer) -> String {
    // Get the start and end iterators
    let start = buff.get_start_iter();
    let end = buff.get_end_iter();
    // Get the string out of the iters
    let gstring = buff.get_text(&start, &end, true);
    // Convert the GString to &str
    gstring.as_deref().unwrap().to_string()
}

// Gets the number of words written
fn get_word_count(buff: &gtk::TextBuffer) -> usize {
    // Get the string from the buffer
    let string = get_buffer_str(&buff);
    // Split based on whitespace
    let split_string = string.split_whitespace();
    // Return the count of the split string
    split_string.count()
}

// Initialize the add note window
pub fn init_add(path: String, notes: gtk::ListStore) {
    // Make the Builder from add_note.glade file
    let b = gtk::Builder::new_from_string(include_str!("../ui/add_note.glade"));
    // Show the window
    b.get_object::<gtk::Window>("add_window").unwrap().show_all();
    // Get the buffer and save button from the file
    let buffer: gtk::TextBuffer = b.get_object("textbuffer1").unwrap();
    let save: gtk::Button = b.get_object("save_button").unwrap();

    // Get all status bar labels for word count, character count etc
    let line_no: gtk::Label = b.get_object("line_no").unwrap();
    let col_no: gtk::Label = b.get_object("col_no").unwrap();
    let char_count: gtk::Label = b.get_object("char_count").unwrap();
    let word_count: gtk::Label = b.get_object("word_count").unwrap();
    let line_count: gtk::Label = b.get_object("line_count").unwrap();

    // Generate a unique id for the note, will be used for its filename
    // Do it here because, if the user saves a note twice, it should not generate
    // two different codes
    let rand_id = crate::util::gen_fcode();

    // Whenever the buffer changes, update status bar information
    buffer.connect_property_cursor_position_notify(move |tb| {
        let text_iter = tb.get_iter_at_mark(&tb.get_insert().unwrap());
        char_count.set_text(&format!("Chars: {}", tb.get_char_count()));
        word_count.set_text(&format!("Words: {}", get_word_count(&tb)));
        line_count.set_text(&format!("Lines: {}", tb.get_line_count()));
        line_no.set_text(&format!("Line: {}", text_iter.get_line()));
        col_no.set_text(&format!("Col: {}", text_iter.get_line_offset()));
    });
    // Save button functionality
    save.connect_clicked(move |_| {
        // Get the buffer string
        let string = get_buffer_str(&buffer);
        // Get the title entry widget's GString
        let title_gstr = b.get_object::<gtk::Entry>("title").unwrap()
                                                    .get_text().unwrap();
        // Convert the GString to &str
        let title_str = title_gstr.as_str();
        // Save the note to a file
        crate::util::save(&string, title_str, path.clone(), rand_id);
        // Add the note to the notes ListStore which is automatically taken by nNotes
        // TreeView
        crate::start_main::add_records(&notes, &path);
    });
}
