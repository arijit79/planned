use gtk;
use gtk::prelude::*;
use std::collections::BTreeMap;
use std::io::prelude::*;

fn init_user(name: &str, path: &str) {
    let mut map = BTreeMap::new();
    map.insert("user", name);
    let yaml = serde_yaml::to_string(&map).unwrap();
    let mut file = std::fs::File::create(path).expect("Can't create user info file");
    file.write_all(yaml.as_bytes()).expect("Can't write data to file, userinfo.yaml");
    println!("S");
}

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

fn capitzlize(s: String) -> String {
    let mut string = s.chars().nth(0).unwrap().to_uppercase().to_string();
    let mut chars = s.chars();
    chars.next();
    for c in chars {
        string.push(c);
    }
    string
}

fn show_setup(b: gtk::Builder, path: String) {
    let window: gtk::Window = b.get_object("user_setup").unwrap();
    window.connect_destroy( |_| std::process::exit(0));
    let user_entry: gtk::Entry = b.get_object("user_entry").unwrap();
    let env_user = std::env::var("USER")
            .expect("Enviroment variable USER not defined");

    let user = String::from(capitzlize(env_user));
    user_entry.set_text(&user);
    let button: gtk::Button = b.get_object("done").unwrap();

    let w_clone = window.clone();
    button.connect_clicked(move |_| {
        let gstr = user_entry.get_text().unwrap();
        let _ = std::fs::create_dir(path.clone());
        let mut userinfo = String::new();
        userinfo.push_str(&path);
        userinfo.push_str("/userinfo.yaml");
        init_user(gstr.as_str(), &userinfo);
        w_clone.destroy()
    } );

    window.show_all();
}

fn main() {
    let e = gtk::init();
    match e {
        Ok(_) => {},
        Err(_) => eprintln!("Unable to initialize GTK")
    }

    let source = include_str!("../ui/setup.glade");
    let builder = gtk::Builder::new_from_string(source);
    let config_path = gen_config_path();

    let userinfo_path = config_path.clone() + "/userinfo.yaml";

    if ! std::path::Path::new(&userinfo_path).exists() {
        show_setup(builder, config_path);
    }

    gtk::main();
}
