use std::io::Cursor;

mod bfck;


fn main() {
	let text = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
	let mut read_buff = Cursor::new(Vec::new());
	let mut write_buff = Cursor::new(Vec::new());
	let mut bfck_box = try!(bfck::BFBox::new(&text, &mut read_buff, &mut write_buff, 10000));
	try!(bfck_box.run);
	println!("{:?}", write_buff);
}