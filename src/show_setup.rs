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

pub fn show_setup(b: gtk::Builder, path: String) {
    let window: gtk::Window = b.get_object("user_setup").unwrap();
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
        w_clone.close();
        crate::init_main(path.clone());
    } );

    window.show_all();
}
