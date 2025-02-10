use glob::glob;
use std::thread;
use std::result::Result;

fn scan_dir(path: &str, exts: Vec<String>) -> Vec<String>{
    let mut files: Vec<String> = Vec::new();
    let mut dir_handles = Vec::new();
    for dir in glob(&format!("{path}/*/")).unwrap().filter_map(Result::ok){
        let t_ext= exts.clone();
        dir_handles.push(thread::spawn(move || {
            scan_dir(dir.to_str().unwrap(), t_ext)
        }));
    };

    for ext in exts {
        for media_file in glob(&format!("{path}/*.{ext}")).unwrap().filter_map(Result::ok){
            let media_path = media_file.to_str().unwrap().to_string();
            files.push(media_path);
        };
    }


    // Wait for results from our spawned threads before returning
    for handle in dir_handles{
        if let Ok(mut t_files) = handle.join() {files.append(&mut t_files);}
    }

    files
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let exts = Vec::from([String::from("mp4"), String::from("mov"), String::from("mkv")]);
        let files = scan_dir("/home/nyangogo/Videos", exts);
        println!("{:?}", files)

    }
}
