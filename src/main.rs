use std::fs;
use std::path::{Path, PathBuf};

fn main() {

    let mut flacs: Vec<PathBuf> = Vec::new();

    get_flacs(Path::new("/media/hibiki/Music"), &mut flacs);

    for x in flacs.iter() {
        println!("{:?}", x);
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
                None => _

            };

            if is_flac {
                buffer.push(path);
            }

        }

    }

}
