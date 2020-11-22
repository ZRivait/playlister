use std::{fs, env};
use std::io::Write;
use std::path::{Path, PathBuf};
use metaflac::Tag;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let music_dir = Path::new(&args[1]);

    let (key, val) = prepare_tag_arg(&args[2]);

    let mut flacs: Vec<PathBuf> = Vec::new();

    get_flacs(music_dir, &mut flacs);

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./playlist.m3u").unwrap();

    for flac in flacs.iter() {
    
        let tag = get_flac_tag(flac, &key.to_uppercase());

        if tag == val {
            file.write(flac.to_str().unwrap().as_bytes()).expect("unable to write data");
            file.write("\n".as_bytes()).expect("unable to write data");
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

fn prepare_tag_arg(arg: &String) -> (String, String) {

    let index = match arg.find('='){

        Some(x) => x,
        None => panic!("malformed tag argument")

    };

    let mut key = arg.clone();
    let mut val = key.split_off(index);
    val = val.strip_prefix('=').unwrap().to_string();

    (key, val)
}
