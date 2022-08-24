extern crate walkdir;
extern crate fs_extra;

use std::{env, fs};
use std::fs::{File, remove_dir_all};
use std::io::{Read, Write};
use std::path::Path;
use fs_extra::dir;
use walkdir::WalkDir;

pub fn mkhtml(config: Config) {
    if Path::new(&config.build_dir).is_dir() {
        remove_dir_all(config.build_dir.clone())
            .expect("Oops! mkhtml couldn't clean build_dir because an error was dropped.");
    }

    for d in [config.pages_dir.clone(), config.parts_dir.clone(), config.static_dir.clone(), config.build_dir.clone()] {
        // for every paths we need
        chk_dir(d);
        // check if it exists, if not, create directory
    };

    println!("pages_dir: {}\nparts_dir: {}\nstatic_dir: {}\nbuild_dir: {}\n",
             config.pages_dir, config.parts_dir, config.static_dir, config.build_dir);
    // print paths

    let files = WalkDir::new(config.pages_dir).follow_links(true);
    // list files in pages_dir

    for file in files {
        let f = file.unwrap();

        if f.path().is_dir() {
            // for every directory in pages_dir (recursive)

            let base_path = f.path().as_os_str().to_os_string().into_string().unwrap();
            let final_path = str::replace(&base_path, "pages", "builds");
            // create path strings and build final_path from path in pages_dir

            chk_dir(final_path);
            // create path to check in build_dir & checks it
        } else {
            // for every files in pages_dir (recursive)
            let watermark_str = "<!-- Built with mkhtml 2.0 (https://github.com/jusdepatate/mkhtml) -->".to_string();

            let base_path = f.path().as_os_str().to_os_string().into_string().unwrap();
            let final_path = str::replace(&base_path, "pages", "builds");
            // create path strings and build final_path from path in pages_dir

            let header = read_file(config.parts_dir.clone() + "/header.html");
            let footer = read_file(config.parts_dir.clone() + "/footer.html");
            // read header and footer files

            let file_body = watermark_str + "\n" +
                &*header + "\n" +
                &*read_file(base_path) + "\n" +
                &*footer;
            // create file body using watermark, header, page content and footer

            write_file(final_path.clone(), file_body);
            // write content to file
        };
    };

    match dir::copy(config.static_dir.clone(), config.build_dir.clone(), &dir::CopyOptions::new()) {
        Err(err) => panic!("Oops! mkhtml couldn't copy {} because an error was dropped:\n{}", config.static_dir, err),
        Ok(_) => println!("Copying static_dir into build_dir..."),
    };

    println!("\nLooks like all files were built");
    println!("Please report errors at https://github.com/jusdepatate/mkhtml\n");

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    if VERSION == "dry" {
        remove_dir_all(config.build_dir.clone()).unwrap();
    }
}

fn write_file(path_str: String, content: String) {
    let path = Path::new(&path_str);

    let mut file = match File::create(&path) {
        Err(err) => panic!("Oops! mkhtml couldn't create {} because an error was dropped:\n{}", path_str, err),
        Ok(file) => file,
    };
    // try to create file, handle errors

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {} because an error was dropped:\n{}", path_str, why),
        Ok(_) => println!("Building {} ...", path_str.clone()),
    };
    // try to write to file, handle errors
}

fn read_file(path_str: String) -> String {
    let path = Path::new(&path_str);

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Oops! mkhtml couldn't read {} because an error was dropped:\n{}", path_str, err),
    };
    // try to open file, handle errors

    let mut content = "".to_string();
    match file.read_to_string(&mut content) {
        Ok(_) => return content,
        Err(err) => panic!("Oops! mkhtml couldn't read {} because an error was dropped:\n{}", path_str, err),
    };
    // try to read file, handle errors
}

fn chk_dir(path_str: String) {
    let path = Path::new(&path_str);

    if ! path.is_dir() {
        // if path doesn't exist
        match fs::create_dir(path) {
            Ok(_) => return,
            Err(err) => panic!("Oops! mkhtml couldn't create {} because an error was dropped:\n{}", path_str, err),
        };
        // create directory,  handle errors
    };
}

pub struct Config {
    pub pages_dir: String,
    pub parts_dir: String,
    pub static_dir: String,
    pub build_dir: String,
    // Configuration Structure
}

impl Config {
    pub fn new() -> Config {
        let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
        // get execution dir and turns it into a string

        Config {
            pages_dir: cwd.clone() + "/pages",
            parts_dir: cwd.clone() + "/parts",
            static_dir: cwd.clone() + "/static",
            build_dir: cwd + "/builds"
        }
        // Build default configuration
    }
}

