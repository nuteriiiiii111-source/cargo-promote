use std::{env, fs, path::Path};

use regex::RegexBuilder;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let mut path = args.next();
    if path == Some("-d".to_string()) {
        path = args.next();
        let path = path.expect("expected path");
        let path = Path::new(&path);
        if !path.exists() {
            panic!("The file '{:?}' does not exist in the CWD.", path);
        }
        if !path.is_file() {
            panic!("this is not a file");
        };
        if let Some(text) = path.to_str() {
            if let Some(start) = text.find('/') {
                let rest = &text[start + 1..];
                if let Some(end) = rest.find('/') {
                    let result = &rest[..end];
                    let default = format!("{}/mod.rs", result);
                    let dir = text.rsplit_once("/").unwrap();
                    if fs::read_dir(dir.0).unwrap().count() == 1 {
                        let replace = format!("{}.rs", result);
                        let target_path = path.to_str().unwrap().replace(&default, &replace);
                        let target_path = Path::new(&target_path);
                        fs::rename(path, target_path).expect("Didi work");
                        fs::remove_dir(format!("src/{}", result)).expect("Cannot delete folder");
                    } else {
                        let content = fs::read_to_string(&text).unwrap();
                        let main = format!("src/{result}/mod.rs");
                        let main = Path::new(&main);
                        if !main.exists() {
                            panic!("The file '{:?}' does not exist in the CWD.", path);
                        }
                        let mut main_content =
                            fs::read_to_string(&main).expect("Couldnt read content");

                        let regex = RegexBuilder::new(r"pub mod ([A-Z 0-9]+(_[A-Z 0-9]+)?);")
                            .case_insensitive(true)
                            .build()
                            .unwrap();
                        for item in regex.captures_iter(&content) {
                            let name = item.get(1).unwrap().as_str();
                            let file = format!("src/{result}/{name}.rs");
                            let path = Path::new(&file);
                            if !path.exists() {
                                panic!("The file '{:?}' does not exist in the CWD.", path);
                            }
                            if !path.is_file() {
                                panic!("this is not a file");
                            };
                            let content = fs::read_to_string(&path).unwrap();

                            let modify = format!("{name} {{\n{content}}}");
                            let reg = format!("{name};");
                            main_content = main_content.replace(&reg, &modify);
                        }
                        let last = format!("src/{result}.rs");
                        match fs::write(last, main_content) {
                            Ok(_) => fs::remove_dir_all(format!("src/{result}"))
                                .expect("Cannot remove {result}"),
                            Err(e) => panic!("Error {e}"),
                        }
                    }
                }
            }
        }
    } else {
        let path = path.expect("expected path");
        let path = Path::new(&path);
        if !path.exists() {
            panic!("The file '{:?}' does not exist in the CWD.", path);
        }
        if path.is_file() {
            let dir_path = path.to_str().expect("path exist");
            if !dir_path.ends_with(".rs") {
                panic!("not a rust file");
            };
            let dir_path = dir_path.trim_end_matches(".rs");
            fs::create_dir(dir_path).expect("couldnt create directory");
            let target_path = path.to_str().unwrap().replace(".rs", "/mod.rs");
            let target_path = Path::new(&target_path);
            fs::rename(path, target_path).expect("couldnt create file");
        } else {
            panic!("this is not a file");
        };
    }
}
