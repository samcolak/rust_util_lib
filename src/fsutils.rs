

use std::{fs, path::Path};



pub fn enumerate_directory(
    
    path: &str,
    relative: &str,     
    recurse: bool

) -> Vec<String> {

    let mut vec_out: Vec<String> = Vec::new();

    let actualpath = match relative.is_empty() {
        true => path.to_string(),
        false => format!("{path}/{relative}")
    };

    if Path::new(&actualpath).exists()

        && let Ok(t) = fs::read_dir(&actualpath) {

            for p in t.flatten() {

                let filename: String = p.file_name().to_str().unwrap().to_string();

                if recurse {
                
                    let qualified: String = format!("{relative}/{filename}");
                    vec_out.push(qualified.clone());                                    

                    if p.path().is_dir() {
                        // its a directory !!
                        let subdirectory_files = enumerate_directory(path, &qualified, true);
                        if !subdirectory_files.is_empty() {
                            vec_out.extend(subdirectory_files);
                        }
                    }

                } else {

                    vec_out.push(filename.clone());

                }

            }

        };

    vec_out.sort();
    vec_out

}