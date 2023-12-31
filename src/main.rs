mod bindings;
mod camera;
mod renderer;
mod textures;
mod window;
use tokio;

#[tokio::main]
async fn main() {
    let mut engine = window::Engine::new().await;
    engine.run();
}
/// set the variable content to the hashmap containing the content and file name of all lua files in the directory
#[macro_export]
macro_rules! load_dir {
    ($directory:expr) => {{
        let dir = $directory;
        use std::fs;
        let dir = fs::read_dir(dir);
        //create hashmap
        let mut map = std::collections::HashMap::new();
        let dir = match dir {
            Ok(dir) => dir,
            Err(e) => {
                panic!("Error Reading Directory {}", e)
            }
        };

        for entry in dir {
            //get the file name and content
            let file_name = entry
                .unwrap()
                .file_name()
                .into_string()
                .expect("Error at file_name");
            let content = fs::read_to_string(format!("{}/{}", $directory, file_name))
                .expect("Error at content reading");
            //insert into hashmap
            map.insert(file_name, content);
        }
        map
    }};
}
#[macro_export]
macro_rules! load_dir_into_bytes {
    ($directory:expr) => {{
        let dir = $directory;
        use std::fs;
        let dir = fs::read_dir(dir);
        //create hashmap
        let mut map = std::collections::HashMap::new();
        let dir = match dir {
            Ok(dir) => dir,
            Err(e) => {
                panic!("Error Reading Directory {}", e)
            }
        };

        for entry in dir {
            //get the file name and content
            let file_name = entry
                .unwrap()
                .file_name()
                .into_string()
                .expect("Error at file_name");
            let content = fs::read(format!("{}/{}", $directory, file_name))
                .expect("Error at content reading");
            //insert into hashmap
            map.insert(file_name, content);
        }
        map
    }};
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn test_load_dir() {
        let map = load_dir!("./src");
        assert_eq!(map.len(), fs::read_dir("./src").unwrap().count());
    }
    #[test]
    fn test_load_dir_into_bytes() {
        let map = load_dir_into_bytes!("./assets/textures");
        assert_eq!(
            map.len(),
            fs::read_dir("./assets/textures").unwrap().count()
        );
    }
}
