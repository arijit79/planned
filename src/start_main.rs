use gtk;
use gtk::prelude::*;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;

fn get_word_count(buff: &gtk::TextBuffer) -> usize {
    let start = buff.get_start_iter();
    let end = buff.get_end_iter();
    let gstring = buff.get_text(&start, &end, false);
    let string = gstring.as_deref().unwrap();
    let split_string: Vec<&str> = string.split(" ").collect();
    split_string.len() - 1
}

pub fn add_records(l: gtk::ListStore, dir: String) {
    let notes_dir = dir + "/notes/";
    let path = std::path::Path::new(&notes_dir);
    if ! path.exists() {
        std::fs::create_dir(path).expect("Unable to initialize notes directory");
    }
    for (count, file) in std::fs::read_dir(path).unwrap().enumerate() {
        let filename = file.unwrap().file_name();
        let mut fn_str = String::from(&notes_dir);
        fn_str.push_str(filename.to_str().unwrap());
        let mut f = File::open(fn_str).expect("Can't open file");
        let mut data = String::new();
        f.read_to_string(&mut data);
        let note: BTreeMap<String, String> = serde_yaml::from_str(&data)
            .expect("Cannot get valid data from the notes dir");
        let title = note.get("title").unwrap();
        let date = note.get("date").unwrap();
        l.insert_with_values(Some(count as u32), &[0, 1], &[title, date]);
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

pub fn init_add(b: gtk::Builder) {
    let add_window: gtk::Window = b.get_object("add_window").unwrap();
    let content: gtk::TextView = b.get_object("content").unwrap();
    let buffer = content.get_buffer().unwrap();
    let line_no: gtk::Label = b.get_object("line_no").unwrap();
    let col_no: gtk::Label = b.get_object("col_no").unwrap();

    let char_count: gtk::Label = b.get_object("char_count").unwrap();
    let word_count: gtk::Label = b.get_object("word_count").unwrap();
    let line_count: gtk::Label = b.get_object("line_count").unwrap();

    add_window.show_all();
    buffer.connect_property_cursor_position_notify(move |tb| {
        let text_iter = tb.get_iter_at_mark(&tb.get_insert().unwrap());
        char_count.set_text(&format!("Chars: {}", tb.get_char_count()));
        word_count.set_text(&format!("Wordss: {}", get_word_count(&tb)));
        line_count.set_text(&format!("Lines: {}", tb.get_line_count()));
        line_no.set_text(&format!("Line: {}", text_iter.get_line()));
        col_no.set_text(&format!("Col: {}", text_iter.get_line_offset()));
    });
}

pub fn start_main(b: gtk::Builder, dir: String) {
    let win: gtk::Window = b.get_object("main_window").unwrap();
    let titlebar: gtk::HeaderBar = b.get_object("titlebar").unwrap();
    let mut userinfo_file = String::new();
    userinfo_file.push_str(&dir);
    userinfo_file.push_str("/userinfo.yaml");
    let user = get_user(userinfo_file);
    titlebar.set_subtitle(Some(&user));
    let notes: gtk::ListStore = b.get_object("notes_list").unwrap();
    let add_button: gtk::Button = b.get_object("add_note").unwrap();
    add_button.connect_clicked(move |_| {
        init_add(b.clone());
    });
    add_records(notes, dir);
    win.connect_destroy(|_| std::process::exit(0));
    win.show_all();
}
