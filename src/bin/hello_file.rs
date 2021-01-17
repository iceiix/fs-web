use fs_web::fs;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let filename = "hello.txt";

    let mut f = fs::File::create(filename)?;
    println!("f = {:?}", f);
    f.write_all(b"Hello, world!")?;

    let metadata = fs::metadata(filename);
    println!("metadata = {:?}", metadata);

    let mut f = fs::File::open(filename)?;
    println!("f = {:?}", f);
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("contents = {:?}", contents);

    fs::remove_file(filename)?;

    Ok(())
}

