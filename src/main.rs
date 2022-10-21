extern crate core;
extern crate fs_extra;
extern crate walkdir;
extern crate mkhtmllib;

use std::{env, fs};
use std::path::Path;


fn main() {
    let mut args: Vec<String> = env::args().collect();
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    if VERSION == "dry" { args.push("b".to_string()); }

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
                    "--pages-dir" => config.set_pages_dir(path),
                    "--parts-dir" => config.set_parts_dir(path),
                    "--static-dir" => config.set_static_dir(path),
                    "--build-dir" => config.set_build_dir(path),
                    _ => panic!("What have you done sir!"),
                }
                // build config from arguments
            }
        }

        println!("pages_dir: {}\nparts_dir: {}\nstatic_dir: {}\nbuild_dir: {}\n",
                 config.clone().get_pages_dir(), config.clone().get_parts_dir(), config.clone().get_static_dir(), config.clone().get_build_dir());
        // print paths

        mkhtmllib::mkhtml(config);

        println!("\nLooks like all files were built");
        println!("Please report errors at https://github.com/jusdepatate/mkhtml\n");
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

#[cfg(test)]
mod tests {
    use std::env;
    use ::{handle_args, main};

    #[test]
    fn test_handle_args() {
        let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
        let mut fake_args: Vec<String> = Vec::new();
        fake_args.push("--pages-dir".to_string());
        fake_args.push(cwd.clone());

        assert_eq!(handle_args("--pages-dir".to_string(), fake_args), cwd);
    }

    #[test]
    #[should_panic]
    fn test_handle_args_panic() {
        let wd = "/b3VpCg==/".to_string();
        let mut fake_args: Vec<String> = Vec::new();
        fake_args.push("--pages-dir".to_string());
        fake_args.push(wd.clone());

        assert_eq!(handle_args("--pages-dir".to_string(), fake_args), wd);
    }

    #[test]
    fn dry_run() {
        env::set_var("CARGO_PKG_VERSION", "dry");
        main()
    }
}