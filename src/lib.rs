extern crate walkdir;
extern crate fs_extra;

use std::{env, fs};
use std::path::Path;
use walkdir::WalkDir;
use std::io::{Read, Write};


pub fn mkhtml(config: Config) {
    if Path::new(&config.build_dir).is_dir() {
        fs::remove_dir_all(config.build_dir.clone())
            .expect("Oops! mkhtml couldn't clean build_dir because an error was dropped.");
    }

    for d in [config.pages_dir.clone(), config.parts_dir.clone(), config.static_dir.clone(), config.build_dir.clone()] {
        // for every paths we need
        chk_dir(d);
        // check if it exists, if not, create directory
    };

    let files = WalkDir::new(config.pages_dir.clone()).follow_links(true);
    // list files in pages_dir

    for file in files {
        let f = file.unwrap();

        if f.path().is_dir() {
            // for every directory in pages_dir (recursive)

            let base_path = f.path().as_os_str().to_os_string().into_string().unwrap();
            let from = Path::new(&config.pages_dir.clone()).canonicalize().unwrap().into_os_string().into_string().unwrap();
            let to = Path::new(&config.build_dir.clone()).canonicalize().unwrap().into_os_string().into_string().unwrap();
            let final_path = str::replace(&base_path, &from, &to);
            // create path strings and build final_path from path in pages_dir

            chk_dir(final_path);
            // create path to check in build_dir & checks it
        } else {
            // for every files in pages_dir (recursive)
            let watermark_str = "<!-- Built with mkhtml 3 (https://github.com/jusdepatate/mkhtml) -->".to_string();

            let base_path = f.path().as_os_str().to_os_string().into_string().unwrap();
            let from = Path::new(&config.pages_dir.clone()).canonicalize().unwrap().into_os_string().into_string().unwrap();
            let to = Path::new(&config.build_dir.clone()).canonicalize().unwrap().into_os_string().into_string().unwrap();
            let final_path = str::replace(&base_path, &from, &to);
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

    match fs_extra::dir::copy(config.static_dir.clone(), config.build_dir.clone(), &fs_extra::dir::CopyOptions::new()) {
        Err(err) => panic!("Oops! mkhtml couldn't copy {} because an error was dropped:\n{}", config.static_dir, err),
        Ok(_) => (),
    };

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    if VERSION == "dry" {
        fs::remove_dir_all(config.build_dir.clone()).unwrap();
    }
}

fn write_file(path_str: String, content: String) {
    let path = Path::new(&path_str);

    let mut file = match fs::File::create(&path) {
        Err(err) => panic!("Oops! mkhtml couldn't create {} because an error was dropped:\n{}", path_str, err),
        Ok(file) => file,
    };
    // try to create file, handle errors

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {} because an error was dropped:\n{}", path_str, why),
        Ok(_) => (),
    };
    // try to write to file, handle errors
}

fn read_file(path_str: String) -> String {
    let path = Path::new(&path_str);

    let mut file = match fs::File::open(&path) {
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

#[derive(Clone)]
pub struct Config {
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
    use std::env;
    use walkdir::WalkDir;
    use ::{read_file, write_file};
    use ::{chk_dir};

    #[test]
    fn test_write_file() {
        write_file(env::current_exe().unwrap().into_os_string().into_string().unwrap() + ".1", "test".to_string());
        // tries to write a file thats named after the current exe +".1" with test
    }

    #[test]
    #[should_panic]
    fn test_write_file_panic() {
        write_file("/".to_string(), "test".to_string());
        // tries to write to "/" (either not a file or permission denied)
    }

    #[test]
    fn test_read_file() {
        for file in WalkDir::new("..").into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                read_file(file.path().canonicalize().unwrap().into_os_string().into_string().unwrap());
                return
            }
        }
        // find the first file in the parent directory and read it
    }

    #[test]
    #[should_panic]
    fn test_read_file_panic() {
        read_file("/".to_string());
        // tries to read "/" (is either not a file or doesnt exist)
    }

    #[test]
    fn test_chk_dir() {
        chk_dir(env::current_dir().unwrap().into_os_string().into_string().unwrap());
        // checks that the current directory exists
    }

    #[test]
    #[should_panic]
    fn test_chk_dir_panic() {
        chk_dir("/b3VpCg==/".to_string());
        // check that this stupidly named folder exists
    }
}