use std::{env, fs, path};
use tar;
fn main() {
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:warning=Building Assets");
    if fs::metadata("assets").is_err() {
        fs::create_dir("assets").unwrap();
    }
    let profile = env::var("PROFILE").unwrap();
    let out_dir = env::current_exe().unwrap();
    //split the out_dir at where the profile name starts
    let out_dir = out_dir
        .to_str()
        .unwrap()
        .split(&profile)
        .collect::<Vec<&str>>()[0]
        .to_string();
    let p = path::Path::new(&out_dir).join(profile).join("game.assets");
    let archive: std::fs::File;
    //remove the exisitng game.assets file if it already exists
    //TODO: should make it only update the changed assets in the future
    if fs::metadata(&p).is_ok() {
        fs::remove_file(&p).unwrap();
    }
    archive = fs::File::create(p).unwrap();

    let mut archive = tar::Builder::new(archive);
    for entry in fs::read_dir("assets").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path().clone(); // Clone the path before passing it to append_dir_all
        if path.is_dir() {
            archive
                .append_dir_all(path.file_name().unwrap(), path.clone())
                .unwrap();
        } else {
            archive
                .append_path_with_name(&path, path.file_name().unwrap())
                .unwrap();
        }
    }
}
