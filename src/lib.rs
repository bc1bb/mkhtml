//! # mkhtmllib
//! This is supposed to go along the official Terminal Wrapper, but it is actually just supposed to have any Wrapper,
//!
//! Makes HTML files from `header.html` and `footer.html` and `pages`,
//!
//! Used to be a simple bash script that I used to build simple sites years ago, then I lost control over myself..
//!
//! mkhtml works in a simple way, it builds HTML files using a simple pattern:
//! - {header.html}
//! - {pages/\*}
//! - {footer.html}
//!
//! Built files will be named after their name in `pages_dir`
//!
//! Copies {static/\*} into {build/static/}.

extern crate fs_extra;
extern crate walkdir;

use fs_extra::dir::{copy, CopyOptions};
use std::env::current_dir;
use std::fs::{create_dir, remove_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;


/// # mkhtml main function
/// Takes a config as input, does not return anything,
///
/// Builds files using configuration given in `Config`,
///
/// Adds a simple HTML comment watermark at the top of every HTML file built with it,
///
/// Uses `walkdir` to go through every folders and sub folders than might be in `pages_dir`,
///
/// If `CARGO_PKG_VERSION` environment variable is "dry", files will be deleted after run,
///
/// Panics on errors.
pub fn mkhtml(config: Config) {
    if Path::new(&config.build_dir).is_dir() {
        remove_dir_all(config.build_dir.clone())
            .expect("Oops! mkhtml couldn't clean build_dir because an error was dropped.");
    }

    for d in [config.pages_dir.clone(), config.parts_dir.clone(), config.static_dir.clone(), config.build_dir.clone()] {
        // for every paths we need
        chk_dir(d);
        // check if it exists, if not, create directory
    }

    // list files in pages_dir
    let files = WalkDir::new(config.pages_dir.clone()).follow_links(true);

    for file in files {
        let f = file.unwrap();

        // for every directory in pages_dir (recursive)
        if f.path().is_dir() {

            // create path strings and build final_path from path in pages_dir
            let base_path = f.path().as_os_str().to_os_string().into_string().unwrap();
            let from = Path::new(&config.pages_dir.clone()).canonicalize().unwrap().into_os_string().into_string().unwrap();
            let to = Path::new(&config.build_dir.clone()).canonicalize().unwrap().into_os_string().into_string().unwrap();
            let final_path = str::replace(&base_path, &from, &to);

            // create path to check in build_dir & checks it
            chk_dir(final_path);
        } else {
            // for every files in pages_dir (recursive)
            let watermark_str =
                "<!-- Built with mkhtml 3 (https://github.com/jusdepatate/mkhtml) -->".to_string();

            // create path strings and build final_path from path in pages_dir
            let base_path = f.path().as_os_str().to_os_string().into_string().unwrap();
            let from = Path::new(&config.pages_dir.clone()).canonicalize().unwrap().into_os_string().into_string().unwrap();
            let to = Path::new(&config.build_dir.clone()).canonicalize().unwrap().into_os_string().into_string().unwrap();
            let final_path = str::replace(&base_path, &from, &to);

            // read header and footer files
            let header = read_file(config.parts_dir.clone() + "/header.html");
            let footer = read_file(config.parts_dir.clone() + "/footer.html");

            // create file body using watermark, header, page content and footer
            let file_body = watermark_str + "\n" +
                &*header + "\n" +
                &*read_file(base_path) + "\n" +
                &*footer;

            // write content to file
            write_file(final_path.clone(), file_body);
        };
    };

    // Copying `static_dir` into `build_dir`.
    match copy(config.static_dir.clone(), config.build_dir.clone(), &CopyOptions::new()) {
        Err(err) => panic!("Oops! mkhtml couldn't copy {} because an error was dropped:\n{}", config.static_dir, err),
        Ok(_) => (),
    };

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    if VERSION == "dry" {
        remove_dir_all(config.build_dir.clone()).unwrap();
    }
}

/// # Write File
/// Takes a path: `String` and content: `String` as argument,
///
/// Does not return anything,
///
/// Simply writes to path given as argument,
///
/// Panics on errors.
fn write_file(path_str: String, content: String) {
    let path = Path::new(&path_str);

    // try to create file, panic on error
    let mut file = match File::create(&path) {
        Err(err) => panic!("Oops! mkhtml couldn't create {} because an error was dropped:\n{}", path_str, err),
        Ok(file) => file,
    };

    // try to write to file, panic on error
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {} because an error was dropped:\n{}", path_str, why),
        Ok(_) => (),
    };
}

/// # Read File
/// Takes a path: `String` as argument,
///
/// Does return a `String`,
///
/// Simply reads a path given as argument,
///
/// Panics on errors.
fn read_file(path_str: String) -> String {
    let path = Path::new(&path_str);

    // try to open file, panic on error
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Oops! mkhtml couldn't read {} because an error was dropped:\n{}", path_str, err),
    };

    // try to read file, panic on error
    let mut content = "".to_string();
    match file.read_to_string(&mut content) {
        Ok(_) => return content,
        Err(err) => panic!("Oops! mkhtml couldn't read {} because an error was dropped:\n{}", path_str, err),
    };
}

/// # Check/Create directory
/// Takes a directory path as argument,
///
/// Check that it exists, if not, creates given path as a folder,
///
/// Panics on errors.
fn chk_dir(path_str: String) {
    let path = Path::new(&path_str);

    // if path doesn't exist
    if !path.is_dir() {
        // create path, panic on error
        match create_dir(path) {
            Ok(_) => return,
            Err(err) => panic!("Oops! mkhtml couldn't create {} because an error was dropped:\n{}", path_str, err),
        };
    };
}

/// # Config struct
/// Holds basic configuration of mkhtml, including all required paths for it to work,
///
/// Has `get_*` and `set_*`,
///
/// Clone-able.
#[derive(Clone)]
pub struct Config {
    pages_dir: String,
    parts_dir: String,
    static_dir: String,
    build_dir: String,
}

impl Config {
    /// Returns a sample `Config`.
    pub fn new() -> Config {
        // get cwd and turn it into a string
        let cwd = current_dir().unwrap().into_os_string().into_string().unwrap();

        // Build default configuration
        Config {
            pages_dir: cwd.clone() + "/pages",
            parts_dir: cwd.clone() + "/parts",
            static_dir: cwd.clone() + "/static",
            build_dir: cwd + "/builds",
        }
    }

    pub fn get_pages_dir(self) -> String { return self.pages_dir }
    pub fn get_parts_dir(self) -> String { return self.parts_dir }
    pub fn get_static_dir(self) -> String { return self.static_dir }
    pub fn get_build_dir(self) -> String { return self.build_dir }

    pub fn set_pages_dir(&mut self, path: String) { self.pages_dir = path }
    pub fn set_parts_dir(&mut self, path: String) { self.parts_dir = path }
    pub fn set_static_dir(&mut self, path: String) { self.static_dir = path }
    pub fn set_build_dir(&mut self, path: String) { self.build_dir = path }
}

#[cfg(test)]
mod tests {
    use {chk_dir, read_file, write_file};
    use std::env::{current_dir, current_exe};
    use walkdir::WalkDir;

    #[test]
    fn test_write_file() {
        // tries to write a file that's named after the current exe +".1" with test
        write_file(current_exe().unwrap().into_os_string().into_string().unwrap() + ".1", "test".to_string());
    }

    #[test]
    #[should_panic]
    fn test_write_file_panic() {
        // tries to write to "/" (either not a file or permission denied)
        write_file("/".to_string(), "test".to_string());
    }

    #[test]
    fn test_read_file() {
        // find the first file in the parent directory and read it
        for file in WalkDir::new("..").into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                read_file(file.path().canonicalize().unwrap().into_os_string().into_string().unwrap());
                return
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_read_file_panic() {
        // tries to read "/" (is either not a file or doesnt exist)
        read_file("/".to_string());
    }

    #[test]
    fn test_chk_dir() {
        // checks that the current directory exists
        chk_dir(current_dir().unwrap().into_os_string().into_string().unwrap());
    }

    #[test]
    #[should_panic]
    fn test_chk_dir_panic() {
        // check that this stupidly named folder exists
        chk_dir("/b3VpCg==/".to_string());
    }
}