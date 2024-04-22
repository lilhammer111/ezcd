use std::env;

pub fn load_file(rela_path: &str) -> String {
    let home_dir = env::var("HOME").expect("Failed to read the home fir");
    format!("{}/{}", home_dir, rela_path)
}
