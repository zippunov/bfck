pub mod bfck;

use std::io::Cursor;
use bfck::BFBox;

#[test]
fn input_test(){
    let code = String::from(",+[-->++++[>++++++++<-]<[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]>>[-]>>--[-[>-<[-]]]>+[-<+++++++++++++<[->-[>+>>]>[+[-<+>]>+>>]<<<<<]>[-]>[-]+>[<-->-[<+>-]]<[<<<<+++++++++++++>>>>-]]<<[-]<<+.[-]<,+]");
    let input = String::from("Slava Zipp");
    let mut read_buff = Cursor::new(input);
    let mut write_buff = Cursor::new(Vec::new());
    let mut bfck_box = match BFBox::new(&code) {
        Ok(b) => b,
        _ => {
            return assert!(false);
        },
    };
    match bfck_box.run(&mut read_buff, &mut write_buff) {
        Ok(_) => (),
        Err(x) => {
            println!("Got error {}", x);
            assert!(false)
        },
    }
    let result_string = String::from_utf8(write_buff.into_inner()).unwrap();
    assert_eq!("Fynin Mvcc", result_string);
}