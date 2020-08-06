mod add_window;
mod delete_window;
mod show_setup;
mod start_main;
mod util;

use std::path::PathBuf;

fn init_main(path: std::path::PathBuf) {
    // Read the main.glade file and run it with start_main function
    start_main::start_main(path);
}

fn main() {
    // Initialize the Gtk backend
    let e = gtk::init();
    match e {
        Ok(_) => {}
        Err(_) => eprintln!("Unable to initialize GTK"),
    }
    // Generate the config path and path to userinfo.yaml file
    let data_dir = PathBuf::from(directories::ProjectDirs::from("org",
                        "arijit79", "planned")
                        .unwrap().data_dir());

    let mut userinfo_yaml = PathBuf::from(&data_dir);
    userinfo_yaml.push("userinfo.yaml");
    println!("{}", userinfo_yaml.display());

    // If the userinfo.yaml file does not exist, launch the user setup screen
    if !userinfo_yaml.exists() {
        let source = include_str!("../ui/setup.glade");
        let builder = gtk::Builder::new_from_string(source);
        show_setup::show_setup(builder, data_dir.clone());
    } else {
        // Else launch the main window
        init_main(data_dir);
    }

    // Start the main loop
    gtk::main();
}
