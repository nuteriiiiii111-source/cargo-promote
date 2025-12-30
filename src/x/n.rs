use std::{fs, path::Path};

pub fn normal(path: Option<String>) {
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
