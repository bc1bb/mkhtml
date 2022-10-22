#![crate_name = "mkhtmllib"]
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
use std::path::PathBuf;
use walkdir::WalkDir;
use Error::{CopyFailed, ReadFailed, RemoveFailed, WriteFailed};


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
/// Returns `()` on success, `mkhtmllib::Error` on errors.
pub fn mkhtml(config: Config) -> Result<(), Error> {
    // if build dir already exists, delete
    // Report error
    if config.clone().get_build_dir().is_dir() {
        let rm = remove_dir_all(config.clone().get_build_dir());

        if rm.is_err() {
            return Err(RemoveFailed);
        }
    }

    // for every paths we need
    for d in config.clone().iter() {
        // check if it exists, if not, create directory
        chk_dir(d)?;
    }

    // list files in pages_dir
    let files = WalkDir::new(config.clone().get_pages_dir()).follow_links(true);

    for file in files {
        let f = file.unwrap();

        // for every directory in pages_dir (recursive)
        if f.path().is_dir() {
            // create path strings and build final_path from path in pages_dir
            let base_path = f.path().as_os_str().to_os_string().into_string().unwrap();

            let from = config.clone().get_pages_dir().canonicalize().unwrap().into_os_string().into_string().unwrap();
            let to = config.clone().get_pages_dir().canonicalize().unwrap().into_os_string().into_string().unwrap();

            let final_path = str::replace(&base_path, &from, &to);

            // create path to check in build_dir & checks it
            chk_dir(PathBuf::from(final_path))?;
        } else {
            // for every files in pages_dir (recursive)
            let watermark_str =
                "<!-- Built with mkhtml 3 (https://github.com/jusdepatate/mkhtml) -->".to_string();

            // create path strings and build final_path from path in pages_dir
            let base_path = f.path().as_os_str().to_os_string().into_string().unwrap();
            let from = config.clone().get_pages_dir().canonicalize().unwrap().into_os_string().into_string().unwrap();
            let to = config.clone().get_build_dir().canonicalize().unwrap().into_os_string().into_string().unwrap();
            let final_path = str::replace(&base_path, &from, &to);

            // read header and footer files
            let header = read_file(config.clone().get_parts_dir().join("header.html"))?;
            let footer = read_file(config.clone().get_parts_dir().join("footer.html"))?;

            // create file body using watermark, header, page content and footer
            let file_body = watermark_str + "\n" +
                &*header + "\n" +
                &*read_file(PathBuf::from(base_path))? + "\n" +
                &*footer;

            // write content to file
            write_file(PathBuf::from(final_path.clone()), file_body)?;
        };
    };

    // Copying `static_dir` into `build_dir`.
    let copy = copy(config.clone().get_static_dir(), config.clone().get_build_dir(), &CopyOptions::new());
    if copy.is_err() {
        return Err(CopyFailed)
    }

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    if VERSION == "dry" {
        remove_dir_all(config.get_build_dir()).unwrap();
    }

    Ok(())
}

/// # Write File
/// Takes a path: `String` and content: `String` as argument,
///
/// Does not return anything,
///
/// Simply writes to path given as argument,
///
/// Returns `()` on success, `mkhtmllib::Error` on errors.
fn write_file(path: PathBuf, content: String) -> Result<(), Error> {
    // try to create file
    let create = File::create(&path);
    if create.is_err() {
        return Err(WriteFailed)
    }
    let mut file = create.unwrap();

    // try to write to file, panic on error
    let write = file.write_all(content.as_bytes());

    if write.is_err() {
        return Err(WriteFailed)
    }
    Ok(())
}

/// # Read File
/// Takes a path: `PathBuf` as argument,
///
/// Returns file content in a `String`,
///
/// Simply reads a path given as argument,
///
/// Returns `()` on success, `mkhtmllib::Error` on errors.
fn read_file(path: PathBuf) -> Result<String, Error> {
    // try to open file, panic on error
    let open = File::open(path);
    if open.is_err() {
        return Err(ReadFailed)
    }

    let mut file = open.unwrap();

    // try to read file, panic on error
    let mut content = "".to_string();
    return match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(_) => Err(ReadFailed),
    }
}

/// # Check/Create directory
/// Takes a directory path as argument,
///
/// Check that it exists, if not, creates given path as a folder,
///
/// Returns `()` on success, `mkhtmllib::Error` on errors.
fn chk_dir(path: PathBuf) -> Result<(), Error>{
    // if path doesn't exist
    if !path.is_dir() {
        // create path, panic on error
        let create = create_dir(path);
        if create.is_err() {
            return Err(WriteFailed);
        }
    }
    Ok(())
}

/// # mkhtml lib errors
#[derive(Debug)]
pub enum Error {
    WriteFailed,
    RemoveFailed,
    CopyFailed,
    ReadFailed,
}

/// # Config struct
/// Holds basic configuration of mkhtml, including all required paths for it to work,
///
/// Has `get_*` and `set_*`,
///
/// Clone-able.
#[derive(Clone)]
pub struct Config {
    pages_dir: PathBuf,
    parts_dir: PathBuf,
    static_dir: PathBuf,
    build_dir: PathBuf,
}

impl Config {
    /// Returns a sample `Config`.
    pub fn new() -> Config {
        // get cwd and turn it into a string
        let cwd = current_dir().unwrap();

        // Build default configuration
        Config {
            pages_dir: cwd.join("pages"),
            parts_dir: cwd.join("parts"),
            static_dir: cwd.join("static"),
            build_dir: cwd.join("builds"),
        }
    }

    /// Returns config in a `[PathBuf; 4]`
    pub fn iter(self) -> [PathBuf; 4] {
        return [self.pages_dir, self.parts_dir, self.static_dir, self.build_dir]
    }

    pub fn get_pages_dir(self) -> PathBuf { return self.pages_dir }
    pub fn get_parts_dir(self) -> PathBuf { return self.parts_dir }
    pub fn get_static_dir(self) -> PathBuf { return self.static_dir }
    pub fn get_build_dir(self) -> PathBuf { return self.build_dir }

    pub fn set_pages_dir(&mut self, path: PathBuf) { self.pages_dir = path }
    pub fn set_parts_dir(&mut self, path: PathBuf) { self.parts_dir = path }
    pub fn set_static_dir(&mut self, path: PathBuf) { self.static_dir = path }
    pub fn set_build_dir(&mut self, path: PathBuf) { self.build_dir = path }
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