use std::{io::{self, Read}, fs::File};

pub fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn get_all_dll(path: &str) -> Vec<String> {
    let mut list = Vec::new();
    for element in std::path::Path::new(path).read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "dll" && path.file_name().unwrap() != "dndserver_patch.dll" {
                list.push(path.display().to_string());
            }
        }
    }

    list
}