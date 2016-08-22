use std::io::{Read,Write};

#[derive(Debug)]
enum Op {
	Start,
    Left(i16),
    Right(i16),
    Add(i16),
    Substract(i16),
    In(i16),
    Out(i16),
    Forward(i16),
	Back(i16),
	End,
}

#[derive(Debug)]
pub struct BFBox {
    reader: Read,
    writer: Write,
    tape: Tape,
    programm: BfProgramm,
    max_cycles: u32,
    cycles: u32
}

impl BFBox {
	pub fn new(text : &str, reader: &mut Read, writer : &mut Write, max_cycles: u32) -> Result<Self, i16> {
		match  BfProgramm::new(text) {
			Ok(programm) => Ok(BFBox {
					reader: reader,
		    		writer: writer,
		    		tape: Tape::new(),
		    		programm: programm,
		    		max_cycles: max_cycles,
		    		cycles: 0,
				}),
			err => err,
		}
		
	}

	pub fn run(&mut self) -> Result<(), i16> {
		loop {
			if self.cycles > self.max_cycles {
				return Err(100)
			}
			match self.programm.next(self.tape.read()) {
				Op::End => {break;},
				Op::Left(x) => try!(self.tape.left(x)),
				Op::Right(x) => try!(self.tape.right(x)),
				Op::Add(x) => try!(self.tape.add(x)),
				Op::Substract(x) => try!(self.tape.substract(x)),
				Op::In(_) => {
					let x = try!(self.readIn());
					try!(self.tape.write(x))
				},
				Op::Out(_) => try!(self.writeOut(self.tape.read())),
				_ => return Err(100),
			}
		}
		Ok(())
	}

	fn readIn(&mut self)  -> Result<i16, i16> {
		let mut buffer = [0];
		match self.reader.read(&mut buffer[..]) {
			Ok(1) => Ok(buffer[0] as i16),
			_ => Err(100),
		}
	}

	fn writeOut(&mut self, value : u8) -> Result<(), i16> {
		let buffer = [value];
		match self.writer.write(&buffer[..]) {
			Ok(1) => Ok(()),
			_ => Err(100),
		}
	}
}



#[derive(Debug)]
struct BfProgramm {
	ops: Vec<Op>,
	pointer: usize,
}

impl BfProgramm {
	fn new(text : &str) -> Result<Self, i16> {
		match parse_bftext(text) {
		    Ok(ops) =>  Ok(BfProgramm{
							ops: ops,
							pointer: 0,
						}),
		    err => err,
		}
	}

	fn next(&mut self, tape_val : u8) -> Op {
		self.pointer = self.pointer + 1;
		match self.ops[self.pointer] {
			Op::Back(index) => {
				if tape_val != 0 {
					self.pointer = index as usize;
					return self.next(tape_val)
				}
			}
			Op::Forward(index) => {
				if tape_val == 0 {
					self.pointer = index as usize;
					return self.next(tape_val)
				}
			}
			op => {}
		}
		self.ops[self.pointer]
	}
}

fn parse_bftext(text : &str) -> Result<Vec<Op>, i16> {
	let mut ops = text.chars().filter_map(|c| {
			match c {
				'>' => Some(Op::Right(1)),
				'<' => { Some(Op::Left(1)) }
				'+' => { Some(Op::Add(1)) }
				'-' => { Some(Op::Substract(1)) }
				'.' => { Some(Op::Out(1)) }
				',' => { Some(Op::In(1)) }
				'[' => { Some(Op::Forward(0)) }
				']' => { Some(Op::Back(0)) }
				_ => { None } // BF skips non-instruction char's.
			}
		})
		// Compressing repeating operations
		.fold(vec![Op::Start], |mut acc, op| {
			let last_op = acc.pop();
			match op {
				Op::Right(_) => {
					if let Some(Op::Right(x)) = last_op {
						acc.push(Op::Right(x + 1));
					} else {
						acc.push(last_op.unwrap());
						acc.push(Op::Right(1));
					}
				},
				Op::Left(_) => {
					if let Some(Op::Left(x)) = last_op {
						acc.push(Op::Left(x + 1));
					} else {
						acc.push(last_op.unwrap());
						acc.push(Op::Left(1));
					}
				}
				Op::Add(_) => {
					if let Some(Op::Add(x)) = last_op {
						acc.push(Op::Add(x + 1));
					} else {
						acc.push(last_op.unwrap());
						acc.push(Op::Add(1));
					}
				}
				Op::Substract(_) => {
					if let Some(Op::Substract(x)) = last_op {
						acc.push(Op::Substract(x + 1));
					} else {
						acc.push(last_op.unwrap());
						acc.push(Op::Substract(1));
					}
				}
				other_op => {
					acc.push(last_op.unwrap());
					acc.push(other_op);
				}
			}
			acc
		});
	ops.push(Op::End);
	//marking correspondent Forward - Back operations
	let mut stack : Vec<usize> = Vec::new();
	for i in 0..ops.len() {
		match ops[i] {
			Op::Forward(_) => {
				stack.push(i)
			}
			Op::Back(_) => {
				match stack.pop() {
					Some(index) => {
						ops[i] = Op::Back(index as i16);
						ops[index] = Op::Forward(i as i16);
					}
					None => {
						return Err(100)
					}
				}
			}
			_ => {}
		}
	}
	if stack.len() > 0 {
		return Err(100)
	}
	Ok(ops)
}