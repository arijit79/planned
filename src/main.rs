use gtk;
mod show_setup;
mod start_main;
mod util;
mod add_window;
mod config_delete;

fn gen_config_path() -> String {
    let mut config = String::new();
    // Read the XDG_CONFIG_HOME Linux enviroment variable
    let xdg_config_home = std::env::var("XDG_CONFIG_HOME");
    // If it exists, then the add /planned to it and consider it to be the
    // config dir
    if let Ok(i) = xdg_config_home {
        config.push_str(&i);
        config.push_str("/planned")
    } else {
        // Else filnd the HOME enviroment variable and add .config/planned to it
        // and consider it to be the config dir
        config.push_str(&std::env::var("HOME")
                .expect("Enviroment variable HOME not defined"));
        config.push_str("/.config/planned")
    }
    config
}

fn init_main(config_path: String) {
    // Read the main.glade file and run it with start_main function
    let main_src = include_str!("../ui/main.glade");
    start_main::start_main(main_src.to_string(), config_path);
}

fn main() {
    // Initialize the Gtk backend
    let e = gtk::init();
    match e {
        Ok(_) => {},
        Err(_) => eprintln!("Unable to initialize GTK")
    }
    // Generate the config path and path to userinfo.yaml file
    let config_path = gen_config_path();
    let userinfo_path = config_path.clone() + "/userinfo.yaml";

    // If the userinfo.yaml file does not exist, launch the user setup screen
    if ! std::path::Path::new(&userinfo_path).exists() {
        let source = include_str!("../ui/setup.glade");
        let builder = gtk::Builder::new_from_string(source);
        show_setup::show_setup(builder, config_path.clone());
    } else {
        // Else launch the main window
        init_main(config_path);
    }

    // Start the main loop
    gtk::main();
}
