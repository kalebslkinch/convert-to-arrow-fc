use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn write(data: &String, filename: String) {
    let mut file = File::create(format!("{}", filename)).unwrap();
    file.write_all(&data.as_bytes()).unwrap();
}

pub fn read(filename: &String) -> String {
    let mut file = File::open(format!("{}", filename)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
