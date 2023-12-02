use std::{fs, env};

mod a1_trebuchet;
mod a2_trebuchet;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn read_file(path: &str) -> String {
	fs::read_to_string(&path).expect(format!("Failed to read {}", &path).as_str())
}

fn main() {
    println!("{}", a2_trebuchet::trebuchet(read_file("input/trebuchet.txt").as_str()));
}
