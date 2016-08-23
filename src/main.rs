use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::io::Cursor;
use std::io;
use std::env;

mod bfck;

fn read_file(path_str : &String) -> String {
	let path = Path::new(path_str);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut file_body = String::new();
    match file.read_to_string(&mut file_body) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {},
    }
    file_body
}

fn run_box(code : &String) -> Result<(), i32> {
	//let mut read_buff = Cursor::new(Vec::new());
	//let mut write_buff = Cursor::new(Vec::new());
	let mut bfck_box = try!(bfck::BFBox::new(&code, 1000000000));
	// try!(bfck_box.run(&mut read_buff, &mut write_buff));
	try!(bfck_box.run(&mut io::stdin(), &mut io::stdout()));
	// println!("{:?}", write_buff);
	Ok(())
}


fn main() {
	let path = match env::args().nth(1) {
		Some(p) => p,
		None => panic!("Here comes usage message"),
	};
	let code = read_file(&path);
	match run_box(&code) {
		Ok(_) => {},
		Err(x) => println!("Error {}", x)
	}
}