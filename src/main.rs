use std::fs;

fn main() {
    let dir = ".";

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        println!(
            "{}",
            entry
                .path()
                .as_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );
    }
}
