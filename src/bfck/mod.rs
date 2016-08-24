use std::io::{Read, Write};

pub fn error_description(code: i16) -> &'static str {
    match code {
        1 => "Invalid bf source (Symbols [ and ] does not match)",
        2 => "Error writing to output stream",
        3 => "Error reading input stream",
        4 => "Invalid operation pointer",
        5 => "Attempt to write negative value",
        6 => "Addressing above 30000",
        7 => "Addressing below 0",
        _ => "Unknown error",
    }
}

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
struct Tape {
    cells: Vec<u8>,
    pointer: usize,
}

fn wrap(value: i16) -> i16 {
    let new_value = value % 256;
    if new_value < 0 {
        return new_value + 256;
    } else {
        return new_value;
    }
}

impl Tape {
    fn new() -> Self {
        Tape {
            cells: vec!(0; 30000),
            pointer: 0,
        }
    }

    fn read(&mut self) -> u8 {
        self.cells[self.pointer]
    }

    fn write(&mut self, value: i16) -> Result<(), i16> {
        self.cells[self.pointer] = wrap(value) as u8;
        Ok(())
    }

    fn left(&mut self, value: i16) -> Result<(), i16> {
        let decrement = value as usize;
        if self.pointer < decrement {
            return Err(7);
        }
        self.pointer = self.pointer - decrement;
        Ok(())
    }

    fn right(&mut self, value: i16) -> Result<(), i16> {
        let increment = value as usize;
        if self.pointer + increment > 30000 {
            return Err(6);
        }
        self.pointer = self.pointer + increment;
        Ok(())
    }

    fn add(&mut self, value: i16) -> Result<(), i16> {
        let read = self.read() as i16;
        let new_value = read + value;
        try!(self.write(new_value));
        Ok(())
    }

    fn substract(&mut self, value: i16) -> Result<(), i16> {
        let read = self.read() as i16;
        let new_value = read - value;
        try!(self.write(new_value));
        Ok(())
    }
}

#[derive(Debug)]
pub struct BFBox {
    tape: Tape,
    programm: BfProgramm,
}

impl BFBox {
    pub fn new(text: &String) -> Result<Self, i16> {
        let programm = try!(BfProgramm::new(text));
        Ok(BFBox {
            tape: Tape::new(),
            programm: programm,
        })
    }

    pub fn run(&mut self, reader: &mut Read, writer: &mut Write) -> Result<(), i16> {
        loop {
            let curr_value = self.tape.read();
            match *self.programm.next(curr_value) {
                Op::End => { return Ok(()); },
                Op::Left(x) => try!(self.tape.left(x)),
                Op::Right(x) => try!(self.tape.right(x)),
                Op::Add(x) => try!(self.tape.add(x)),
                Op::Substract(x) => try!(self.tape.substract(x)),
                Op::In(_) => {
                    let x = try!(self.read_in(reader));
                    try!(self.tape.write(x))
                },
                Op::Out(_) => try!(self.write_out(writer, curr_value)),
                _ => return Err(4),
            }
        }
    }

    fn read_in(&mut self, reader: &mut Read) -> Result<i16, i16> {
        let mut buffer = [0];
        match reader.read(&mut buffer[..]) {
            Ok(1) => Ok(buffer[0] as i16),
            _ => Err(3),
        }
    }

    fn write_out(&mut self, writer: &mut Write, value: u8) -> Result<(), i16> {
        let buffer = [value];
        match writer.write(&buffer[..]) {
            Ok(1) => Ok(()),
            _ => Err(2),
        }
    }
}

#[derive(Debug)]
struct BfProgramm {
    ops: Vec<Op>,
    pointer: usize,
}

impl BfProgramm {
    fn new(text: &String) -> Result<Self, i16> {
        let ops = try!(parse_bftext(text));
        Ok(BfProgramm {
            ops: ops,
            pointer: 0,
        })
    }

    fn next(&mut self, tape_val: u8) -> &Op {
        loop {
            self.pointer = self.pointer + 1;
            let op = &self.ops[self.pointer];
            match *op {
                Op::Back(index) => {
                    match tape_val {
                        0 => {},
                        _ => { self.pointer = index as usize; },
                    }
                }
                Op::Forward(index) => {
                    match tape_val {
                        0 => { self.pointer = index as usize; },
                        _ => {},
                    }
                }
                _ => return op
            }
        }
    }
}

fn parse_bftext(text: &String) -> Result<Vec<Op>, i16> {
    let mut ops = text.chars().filter_map(|c| {
        match c {
            '>' => Some(Op::Right(1)),
            '<' => Some(Op::Left(1)),
            '+' => Some(Op::Add(1)),
            '-' => Some(Op::Substract(1)),
            '.' => Some(Op::Out(1)),
            ',' => Some(Op::In(1)),
            '[' => Some(Op::Forward(0)),
            ']' => Some(Op::Back(0)),
            _ => None,// BF skips non-instruction char's.
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
    let mut stack: Vec<usize> = Vec::new();
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
                        return Err(1)
                    }
                }
            }
            _ => {}
        }
    }
    if stack.len() > 0 {
        return Err(1)
    }
    Ok(ops)
}