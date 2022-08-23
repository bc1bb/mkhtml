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
    } else if (args.contains(&"build".to_string())) | (args.contains(&"b".to_string())) {
        let mut config = Config::new();
        // mutable because we might need to edit according to arguments

        let config_args = [ "--pages-dir", "--parts-dir", "--static-dir", "--build-dir" ];
        // every config arguments

        for i in config_args {
            if args.contains(&i.to_string()) {
                // if any config argument detected in call
                let path = handle_args(i.to_string(), args.clone());

                match i {
                    "--pages-dir" => config.pages_dir = path,
                    "--parts-dir" => config.parts_dir = path,
                    "--static-dir" => config.static_dir = path,
                    "--build-dir" => config.build_dir = path,
                    _ => panic!("What have you done sir!"),
                }
                // build config from arguments
            }
        }

        mkhtml(config);
    } else {
        help()
    }
}

fn help() {
    println!("No valid argument detected,");
    println!("If you wish to build, run again with 'build' argument.\n");
    println!("If you wish to specify a path for pages, parts, static and/or build,");
    println!("use --pages-dir [path], --parts-dir [dir], --static-dir [path] and/or --build-dir [path]");
}

fn mkhtml(config: Config) {
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

    match dir::copy(config.static_dir.clone(), config.build_dir.clone(), &CopyOptions::new()) {
        Err(err) => panic!("Oops! mkhtml couldn't copy {} because an error was dropped:\n{}", config.static_dir, err),
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

fn handle_args(dir: String, args_array: Vec<String>) -> String {
    let index = args_array.iter().position(|x| x == &dir).unwrap();
    // find index of "--[pages|parts|static|build]-dir"

    if args_array.len() >= index+1 {
        // stupido check no 1

        let path_str = args_array[index+1].clone();
        let path = Path::new(&path_str);
        // index   = index of "--[pages|parts|static|build]-dir"
        // index+1 = assumed index of path

        if path.is_dir() {
            // stupido check no 2

            return fs::canonicalize(path).unwrap().into_os_string().into_string().unwrap();
            // returns absolute path as string
        } else {
            panic!("You seem to have specified a wrong path");
        }
    } else {
        panic!("You seem to have used a path argument without an argument");
    }
}

struct Config {
    pages_dir: String,
    parts_dir: String,
    static_dir: String,
    build_dir: String,
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