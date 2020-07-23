use gtk;
use gtk::prelude::*;
use std::collections::BTreeMap;
use std::io::prelude::*;

// write the name to the path using YAML
fn init_user(name: &str, path: &str) {
    // Generatw a new BTreeMap and insert key for user with value name
    let mut map = BTreeMap::new();
    map.insert("user", name);
    // Generatw a yaml file out of BTreeMap
    let yaml = serde_yaml::to_string(&map).unwrap();
    // Create the path file, exit return if it fails to create the file
    let mut file = std::fs::File::create(path).expect("Can't create user info file");
    // Write to the file, exit return if it fails to create the file
    file.write_all(yaml.as_bytes()).expect("Can't write data to file, userinfo.yaml");
}

// Capitalize the first letter of the s String
fn capitzlize(s: String) -> String {
    // Get the first letter of thw s String and convert it to uppercase
    let mut string = s.chars().nth(0).unwrap().to_uppercase().to_string();
    // Get an iterator to the characters of the String and move forward by one character
    // since that is what we capitalize in the previous line
    let mut chars = s.chars();
    chars.next();
    // Append all other characters to the end of string and return it
    for c in chars {
        string.push(c);
    }
    string
}

pub fn show_setup(b: gtk::Builder, path: String) {
    // Get the window and start displaying it
    let window: gtk::Window = b.get_object("user_setup").unwrap();
    // Get the username entry field
    let user_entry: gtk::Entry = b.get_object("user_entry").unwrap();
    // Autofill stuff. Get the USER enviroment variable and set the its value in the entry
    // just in case the user wants to use the same name as that is in its computer
    let env_user = std::env::var("USER")
            .expect("Enviroment variable USER not defined");
    // Generatw a new string from by capitalizing the first letter from the user enviroment
    // variable
    let user = String::from(capitzlize(env_user));
    // Set the text in user_entry
    user_entry.set_text(&user);
    // Get the done button
    let button: gtk::Button = b.get_object("done").unwrap();
    // Clone the window to be used in the connect_signal of the button
    let w_clone = window.clone();

    // Functionality on clicking
    button.connect_clicked(move |_| {
        // Get the text of the entry box
        let gstr = user_entry.get_text().unwrap();
        // Create the configuration directory
        let _ = std::fs::create_dir(path.clone());
        // Create a string containing the path to the userinfo file
        let mut userinfo = String::new();
        userinfo.push_str(&path);
        userinfo.push_str("/userinfo.yaml");
        // Put all necessory details using the init_user function
        init_user(gstr.as_str(), &userinfo);
        // Destroy the window
        w_clone.destroy();
        // Start the main program
        crate::init_main(path.clone());
    } );
    // Show the window
    window.show_all();
}
