use std::io::Cursor;

mod bfck;

fn run_box() -> Result<(), i32> {
	let text = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
	let mut read_buff = Cursor::new(Vec::new());
	let mut write_buff = Cursor::new(Vec::new());
	let mut bfck_box = try!(bfck::BFBox::new(&text, 1000));
	try!(bfck_box.run(&mut read_buff, &mut write_buff));
	println!("{:?}", write_buff);
	Ok(())
}


fn main() {
	println!("{:?}", run_box());
}