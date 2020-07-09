use gtk::prelude::*;

fn get_word_count(buff: &gtk::TextBuffer) -> usize {
    let start = buff.get_start_iter();
    let end = buff.get_end_iter();
    let gstring = buff.get_text(&start, &end, true);
    let string = gstring.as_deref().unwrap();
    let split_string = string.split_whitespace();
    split_string.count()
}

pub fn init_add(b: gtk::Builder, path: String) {
    let add_window: gtk::Window = b.get_object("add_window").unwrap();
    let title: gtk::Entry = b.get_object("title").unwrap();
    let content: gtk::TextView = b.get_object("content").unwrap();
    let buffer = content.get_buffer().unwrap();
    let line_no: gtk::Label = b.get_object("line_no").unwrap();
    let col_no: gtk::Label = b.get_object("col_no").unwrap();

    let char_count: gtk::Label = b.get_object("char_count").unwrap();
    let word_count: gtk::Label = b.get_object("word_count").unwrap();
    let line_count: gtk::Label = b.get_object("line_count").unwrap();
    let rand_id = crate::util::gen_fcode();

    let save: gtk::Button = b.get_object("save_button").unwrap();
    buffer.connect_property_cursor_position_notify(move |tb| {
        let text_iter = tb.get_iter_at_mark(&tb.get_insert().unwrap());
        char_count.set_text(&format!("Chars: {}", tb.get_char_count()));
        word_count.set_text(&format!("Words: {}", get_word_count(&tb)));
        line_count.set_text(&format!("Lines: {}", tb.get_line_count()));
        line_no.set_text(&format!("Line: {}", text_iter.get_line()));
        col_no.set_text(&format!("Col: {}", text_iter.get_line_offset()));
    });

    save.connect_clicked(move |_| {
        let start = buffer.get_start_iter();
        let end = buffer.get_end_iter();
        let gstring = buffer.get_text(&start, &end, true);
        let string = gstring.as_deref().unwrap();
        let title_gstr = title.get_text().unwrap();
        let title_str = title_gstr.as_str();
        crate::util::save(string, title_str, path.clone(), rand_id);
        let notes: gtk::ListStore = b.get_object("notes_list").unwrap();
        crate::start_main::add_records(notes, &path)
    });

    add_window.connect_destroy(|win| {
        win.hide();
    } );
    // add_window.connect
    add_window.show_all();
}
