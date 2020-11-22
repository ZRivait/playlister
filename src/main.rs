use std::fs;
use std::path::{Path, PathBuf};
use metaflac::Tag;

fn main() {

    
    let mut flacs: Vec<PathBuf> = Vec::new();

    get_flacs(Path::new("/media/hibiki/Music"), &mut flacs);

    for flac in flacs.iter() {
    
        let tag = get_flac_tag(flac, "RATING");
        
        if tag == "5" {
            println!("{:?}", flac);
        }

    }

}

fn get_flacs(dir: &Path, buffer: &mut Vec<PathBuf>) {


    let top_level_dir = match fs::read_dir(dir) {
    
        Ok(x) => x,
        Err(_) => panic!("error opening directory")

    };

    for entry in top_level_dir {

        let file = match entry{

            Ok(x) => x,
            Err(_) => panic!("io error")

        };
        
        let path = file.path();

        if path.is_dir() == true {
            
            get_flacs(path.as_path(), buffer)

        }
        else {

            let is_flac = match path.extension() {

                Some(x) => x == "flac",
                None => false

            };

            if is_flac {
                buffer.push(path);
            }
        }
    }
}

fn get_flac_tag(path_to_flac: &Path, tag_to_get: &str) -> String {

    let tag = Tag::read_from_path(path_to_flac).unwrap();
    let comments = tag.vorbis_comments().unwrap();

    match comments.get(tag_to_get) {
        
        Some(x) => x[0].clone(),
        None => String::from("")

    }
}
