
use std::{fs, path::Path};

pub fn enumerate_directory(
    
    path: &str,
    relative: &str,     
    recurse: bool

) -> Vec<String> {

    let mut _out: Vec<String> = Vec::new();

    let _actualpath = match relative.is_empty() {
        true => path.to_string(),
        false => format!("{path}/{relative}")
    };

    if Path::new(&_actualpath).exists()

        && let Ok(t) = fs::read_dir(&_actualpath) {

            for _p in t.flatten() {

                let _filename = _p.file_name().to_str().unwrap().to_string();

                if recurse {
                
                    let _qualified = format!("{relative}/{_filename}");
                    _out.push(_qualified.clone());                                    

                    if _p.path().is_dir() {
                        // its a directory !!
                        let _subdirectory_files = enumerate_directory(path, &_qualified, true);
                        if !_subdirectory_files.is_empty() {
                            _out.extend(_subdirectory_files);
                        }
                    }

                } else {

                    _out.push(_filename.clone());

                }

            }

        };

    _out.sort();
    _out

}