//! # mkhtml Terminal Wrapper
//! Calls `mkhtmllib` using terminal arguments given by user,
//!
//! Accepted (command line) arguments are:
//! - `--pages-dir` => `pages_dir`,
//! - `--parts-dir` => `parts_dir`,
//! - `--static-dir` => `static_dir`,
//! - `--build-dir"` => `build_dir`.

extern crate core;
extern crate fs_extra;
extern crate mkhtmllib;
extern crate walkdir;

use mkhtmllib::{mkhtml, Config};
use std::env::args;
use std::fs::canonicalize;
use std::path::Path;

// TODO: Better error handling than just panicking
// TODO: Use PathBuf
// TODO: GUI WRAPPER ??


/// # Main function
/// Handles command line arguments,
/// Creates a `Config`,
/// Sends `Config` to `mkhtml()`.
fn main() {
    let mut args: Vec<String> = args().collect();
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    // Dry run is for test (see below), when doing dry run let's assume we want to build
    if VERSION == "dry" { args.push("b".to_string()); }

    println!(
        "           __  __  __  __  _   _  _____  __  __  _
          |  \\/  ||  |/  /| |_| ||_   _||  \\/  || |__
          |_|\\/|_||__|\\__\\|_| |_|  |_|  |_|\\/|_||____|");
    println!("              {} - github:jusdepatate/mkhtml\n", VERSION);

    // This wrapper needs 2 arguments at least to work
    if args.is_empty() {
        help()
    } else if (args.contains(&"build".to_string())) | (args.contains(&"b".to_string())) {
        let mut config = Config::new();

        // every config arguments
        let config_args = ["--pages-dir", "--parts-dir", "--static-dir", "--build-dir"];

        for i in config_args {
            // if any config argument detected in call
            if args.contains(&i.to_string()) {
                let path = handle_args(i.to_string(), args.clone());

                // build Config from arguments
                match i {
                    "--pages-dir" => config.set_pages_dir(path),
                    "--parts-dir" => config.set_parts_dir(path),
                    "--static-dir" => config.set_static_dir(path),
                    "--build-dir" => config.set_build_dir(path),
                    _ => panic!("What have you done sir!"),
                }
            }
        }

        // print paths
        println!("pages_dir: {}\nparts_dir: {}\nstatic_dir: {}\nbuild_dir: {}\n",
                 config.clone().get_pages_dir(), config.clone().get_parts_dir(), config.clone().get_static_dir(), config.clone().get_build_dir());

        // send Config to mkhtmllib
        mkhtml(config);

        println!("\nLooks like all files were built");
        println!("Please report errors at https://github.com/jusdepatate/mkhtml\n");
    } else {
        help()
    }
}

/// Prints a simple help message.
fn help() {
    println!("No valid argument detected,");
    println!("If you wish to build, run again with 'build' argument.\n");
    println!("If you wish to specify a path for pages, parts, static and/or build,");
    println!("use --pages-dir [path], --parts-dir [dir], --static-dir [path] and/or --build-dir [path]");
}

/// # Handle Arguments
/// Handles the whole argument `Vec` with the name of the argument we are looking for, return a `String`,
///
/// Will look for the next element in the list after `arg_name`,
///
/// Returns a path in a `String`.
fn handle_args(arg_name: String, args_array: Vec<String>) -> String {
    let index = args_array.iter().position(|x| x == &arg_name).unwrap();
    // find index of "--[pages|parts|static|build]-dir"

    // Checking that there is actually more element in the Vec than the position of `arg_name`+1,
    // (Because we are gonna look for the content of `arg_name`+1.
    if args_array.len() >= index + 1 {

        // index   = index of "--[pages|parts|static|build]-dir"
        // index+1 = assumed index of path

        let path_str = args_array[index + 1].clone();
        let path = Path::new(&path_str);

        // Checking that the path exists/is a dir.
        if path.is_dir() {
            // returns absolute path as string
            return canonicalize(path).unwrap().into_os_string().into_string().unwrap();
        } else {
            panic!("You seem to have specified a wrong path");
        }
    } else {
        panic!("You seem to have used a path argument without an argument");
    }
}

#[cfg(test)]
mod tests {
    use std::env::{current_dir, set_var};
    use {handle_args, main};

    #[test]
    fn test_handle_args() {
        let cwd = current_dir().unwrap().into_os_string().into_string().unwrap();
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
        set_var("CARGO_PKG_VERSION", "dry");
        main()
    }
}