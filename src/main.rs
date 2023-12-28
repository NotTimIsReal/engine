mod bindings;
mod renderer;
mod window;
use tokio;
#[tokio::main]
async fn main() {
    let mut engine = window::Engine::new().await;
    engine.run();
}
/// set the variable content to the hashmap containing the content and file name of all lua files in the directory
#[macro_export]
macro_rules! load_lua_files {
    ($directory:expr, $var1:ident) => {{
        let dir = $directory;
        use std::fs;
        let dir = fs::read_dir(dir);
        //create hashmap
        let mut map = std::collections::HashMap::new();
        for entry in dir.unwrap() {
            //get the file name and content
            let file_name = entry.unwrap().file_name().into_string().unwrap();
            let content = fs::read_to_string(file_name.clone()).unwrap();
            //insert into hashmap
            map.insert(file_name, content);
        }
        $var1 = map;
    }};
}
