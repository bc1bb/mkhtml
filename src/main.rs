extern crate fs_extra;
extern crate core;
extern crate walkdir;
extern crate mkhtmllib;

use std::{env, fs};
use std::path::Path;


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
        let mut config = mkhtmllib::Config::new();
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

        mkhtmllib::mkhtml(config);
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

