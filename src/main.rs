use gtk;
mod show_setup;

fn gen_config_path() -> String {
    let mut config = String::new();
    let xdg_config_home = std::env::var("XDG_CONFIG_HOME");
    if let Ok(i) = xdg_config_home {
        config.push_str(&i);
        config.push_str("planned")
    } else {
        config.push_str(&std::env::var("HOME")
                .expect("Enviroment variable HOME not defined"));
        config.push_str("/.config/planned")
    }
    config
}

fn main() {
    let e = gtk::init();
    match e {
        Ok(_) => {},
        Err(_) => eprintln!("Unable to initialize GTK")
    }
    let config_path = gen_config_path();
    let userinfo_path = config_path.clone() + "/userinfo.yaml";

    if ! std::path::Path::new(&userinfo_path).exists() {
        let source = include_str!("../ui/setup.glade");
        let builder = gtk::Builder::new_from_string(source);
        show_setup::show_setup(builder, config_path);
    }

    gtk::main();
}
