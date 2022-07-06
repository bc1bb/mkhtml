extern crate fs_extra;
extern crate core;
extern crate walkdir;

use std::{env, fs};
use std::fs::{File, remove_dir_all};
use std::io::{Read, Write};
use std::path::Path;
use fs_extra::dir;
use fs_extra::dir::CopyOptions;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!(
        "           __  __  __  __  _   _  _____  __  __  _
          |  \\/  ||  |/  /| |_| ||_   _||  \\/  || |__
          |_|\\/|_||__|\\__\\|_| |_|  |_|  |_|\\/|_||____|");
    println!("              {} - github:jusdepatate/mkhtml\n", VERSION);

    if args.len() < 2 {
        help()
    } else if (args[1] == "build") | (args[1] == "b") {
        mkhtml()
    } else {
        help()
    }
}

fn help() {
    println!("No valid argument detected,");
    println!("If you wish to build, run again with 'build' argument.");
}

fn mkhtml() {

    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    // get execution dir and turns it into a string

    let (pages_dir, parts_dir, static_dir, build_dir) =
        (cwd.clone() + "/pages", cwd.clone() + "/parts", cwd.clone() + "/static", cwd + "/builds");
    // create variables for all paths we will need

    if Path::new(&build_dir).is_dir() {
        remove_dir_all(build_dir.clone())
            .expect("Oops! mkhtml couldn't clean build_dir because an error was dropped.");
    }

    for d in [pages_dir.clone(), parts_dir.clone(), static_dir.clone(), build_dir.clone()] {
        // for every paths we need
        chk_dir(d);
        // check if it exists, if not, create directory
    };

    println!("pages_dir: {}\nparts_dir: {}\nstatic_dir: {}\nbuild_dir: {}\n",
             pages_dir, parts_dir, static_dir, build_dir);
    // print paths

    let files = WalkDir::new(pages_dir).follow_links(true);
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

            let header = read_file(parts_dir.clone() + "/header.html");
            let footer = read_file(parts_dir.clone() + "/footer.html");
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

    match dir::copy(static_dir.clone(), build_dir, &CopyOptions::new()) {
        Err(err) => panic!("Oops! mkhtml couldn't copy {} because an error was dropped:\n{}", static_dir, err),
        Ok(_) => println!("Copying static_dir into build_dir..."),
    };

    println!("\nLooks like all files were built");
    println!("Please report errors at https://github.com/jusdepatate/mkhtml\n");
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