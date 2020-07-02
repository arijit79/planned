use gtk;
use gtk::prelude::*;

fn main() {
    let e = gtk::init();

    match e {
        Ok(_) => {},
        Err(_) => {}
    }

    let source = include_str!("../setup.glade");
    let builder = gtk::Builder::new_from_string(source);

    let window: gtk::Window = builder.get_object("window1").unwrap();
    window.show_all();

    gtk::main();
}
